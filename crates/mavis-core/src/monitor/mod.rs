// System resource monitoring module for MAVIS

pub mod cpu;
pub mod memory;
pub mod network;
pub mod disk;

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::config::Config;
use crate::error::CoreError;
use log::{error, info};
use tokio::sync::watch;

/// Resource usage data collected by the monitor
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// CPU usage as a percentage (0-100)
    pub cpu_usage: f32,
    /// Total physical memory in bytes
    pub total_memory: u64,
    /// Used physical memory in bytes
    pub used_memory: u64,
    /// Memory usage as a percentage (0-100)
    pub memory_usage: f32,
    /// Network download speed in bytes per second
    pub network_down_bytes: u64,
    /// Network upload speed in bytes per second
    pub network_up_bytes: u64,
    /// Disk busy time as a percentage (0-100)
    pub disk_usage: f32,
    /// Disk read speed in bytes per second
    pub disk_read_bytes: u64,
    /// Disk write speed in bytes per second
    pub disk_write_bytes: u64,
    /// Total free disk space in bytes
    pub disk_free_bytes: u64,
    /// Time when this data was collected
    pub timestamp: std::time::Instant,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            total_memory: 0,
            used_memory: 0,
            memory_usage: 0.0,
            network_down_bytes: 0,
            network_up_bytes: 0,
            disk_usage: 0.0,
            disk_read_bytes: 0,
            disk_write_bytes: 0,
            disk_free_bytes: 0,
            timestamp: std::time::Instant::now(),
        }
    }
}

/// Manages system resource monitoring
pub struct ResourceMonitor {
    /// Latest resource usage information
    usage: Arc<Mutex<ResourceUsage>>,
    /// Monitoring interval in milliseconds
    update_interval: u32,
    /// Whether monitoring is currently active
    active: bool,
    /// Monitor thread handle
    thread_handle: Option<JoinHandle<()>>,
    /// Sender for stop signal
    stop_tx: watch::Sender<bool>,
    /// Receiver for stop signal (kept for cloning)
    stop_rx: watch::Receiver<bool>,
}

impl ResourceMonitor {
    /// Create a new resource monitor with the specified configuration
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        let update_interval = config.monitoring.update_interval_ms;
        let usage = Arc::new(Mutex::new(ResourceUsage::default()));
        // Create a watch channel for the stop signal (initially false)
        let (stop_tx, stop_rx) = watch::channel(false);
        
        Ok(Self {
            usage,
            update_interval,
            active: false,
            thread_handle: None,
            stop_tx,
            stop_rx, // Store the receiver for cloning
        })
    }
    
    /// Start monitoring system resources
    pub fn start(&mut self, config: &Config) -> Result<(), CoreError> {
        if self.active {
            return Ok(());
        }
        
        let usage_clone = self.usage.clone();
        let interval = Duration::from_millis(self.update_interval as u64);
        let monitor_cpu = config.monitoring.monitor_cpu;
        let monitor_ram = config.monitoring.monitor_ram;
        let monitor_network = config.monitoring.monitor_network;
        let monitor_disk = config.monitoring.monitor_disk;
        
        // Create a runtime for the async monitor tasks
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|e| CoreError::IoError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create runtime: {}", e))))?; // Wrap error
            
        // Create a stop receiver clone for the thread
        let mut stop_rx_clone = self.stop_rx.clone();
        
        // Spawn monitoring thread
        let thread_handle = thread::spawn(move || {
            rt.block_on(async {
                let mut interval_timer = tokio::time::interval(interval);
                
                // Initialize monitors
                let mut cpu_monitor = if monitor_cpu {
                    match cpu::CpuMonitor::new() {
                        Ok(monitor) => Some(monitor),
                        Err(e) => {
                            error!("Failed to initialize CPU monitor: {}", e);
                            None
                        }
                    }
                } else {
                    None
                };
                
                let mut memory_monitor = if monitor_ram {
                    match memory::MemoryMonitor::new() {
                        Ok(monitor) => Some(monitor),
                        Err(e) => {
                            error!("Failed to initialize memory monitor: {}", e);
                            None
                        }
                    }
                } else {
                    None
                };
                
                let mut network_monitor = if monitor_network {
                    match network::NetworkMonitor::new() {
                        Ok(monitor) => Some(monitor),
                        Err(e) => {
                            error!("Failed to initialize network monitor: {}", e);
                            None
                        }
                    }
                } else {
                    None
                };
                
                let mut disk_monitor = if monitor_disk {
                    match disk::DiskMonitor::new() {
                        Ok(monitor) => Some(monitor),
                        Err(e) => {
                            error!("Failed to initialize disk monitor: {}", e);
                            None
                        }
                    }
                } else {
                    None
                };
                
                info!("Resource monitoring started with interval {} ms", interval.as_millis());
                
                loop {
                    tokio::select! {
                        _ = interval_timer.tick() => {
                            // Create a new usage object for this collection cycle
                            let mut new_usage = ResourceUsage {
                                timestamp: std::time::Instant::now(),
                                ..Default::default()
                            };
                            
                            // Update CPU usage
                            if let Some(cpu_mon) = &mut cpu_monitor {
                                if let Ok(usage) = cpu_mon.get_usage() {
                                    new_usage.cpu_usage = usage;
                                }
                            }
                            
                            // Update memory usage
                            if let Some(mem_mon) = &mut memory_monitor {
                                if let Ok((total, used, percentage)) = mem_mon.get_usage() {
                                    new_usage.total_memory = total;
                                    new_usage.used_memory = used;
                                    new_usage.memory_usage = percentage;
                                }
                            }
                            
                            // Update network usage
                            if let Some(net_mon) = &mut network_monitor {
                                if let Ok((down, up)) = net_mon.get_usage() {
                                    new_usage.network_down_bytes = down;
                                    new_usage.network_up_bytes = up;
                                }
                            }
                            
                            // Update disk usage
                            if let Some(disk_mon) = &mut disk_monitor {
                                if let Ok((usage, read, write, free)) = disk_mon.get_usage() {
                                    new_usage.disk_usage = usage;
                                    new_usage.disk_read_bytes = read;
                                    new_usage.disk_write_bytes = write;
                                    new_usage.disk_free_bytes = free;
                                }
                            }
                            
                            // Update the shared usage data
                            if let Ok(mut current_usage) = usage_clone.lock() {
                                *current_usage = new_usage;
                            }
                        }
                        
                        // Check for stop signal
                        Ok(_) = stop_rx_clone.changed() => {
                            if *stop_rx_clone.borrow() { // Check if the signal is true (stop)
                                info!("Stop signal received, exiting monitor loop.");
                                break;
                            }
                        }
                    }
                }
            });
        });
        
        self.thread_handle = Some(thread_handle);
        self.active = true;
        
        Ok(())
    }
    
    /// Stop monitoring system resources
    pub async fn stop(&mut self) -> Result<(), CoreError> { // Made async
        if !self.active {
            return Ok(());
        }

        // Send stop signal (true) via the watch channel
        if let Err(e) = self.stop_tx.send(true) {
            error!("Failed to send stop signal: {}", e);
            // Don't necessarily return an error, try to join the thread anyway
        }

        // Join the monitoring thread
        if let Some(handle) = self.thread_handle.take() {
            if let Err(e) = handle.join() {
                error!("Failed to join monitoring thread: {:?}", e);
            }
        }
        
        self.active = false;
        info!("Resource monitoring stopped");
        
        Ok(())
    }
    
    /// Get the current resource usage
    pub fn get_usage(&self) -> ResourceUsage {
        self.usage.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[test]
    fn test_resource_monitor() {
        let config = Config::default();
        let monitor = ResourceMonitor::new(&config);
        assert!(monitor.is_ok());
    }
}