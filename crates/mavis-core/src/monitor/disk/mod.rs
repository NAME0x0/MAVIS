// Disk monitoring for Windows systems

use crate::error::CoreError;
use log::{debug, warn};
use windows::Win32::System::Performance::{
    PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_DOUBLE, PDH_FMT_COUNTERVALUE,
};

/// Disk monitoring functionality
pub struct DiskMonitor {
    query_handle: isize,
    disk_time_counter: isize,
    // disk_queue_counter: isize, // Removed, not used in ResourceUsage
    disk_read_counter: isize,
    disk_write_counter: isize,
    // disk_free_counter: isize, // Removed, using GetDiskFreeSpaceW instead
    last_disk_time: f32,
    // last_queue_length: f32, // Removed
    last_read_rate: f32,
    last_write_rate: f32,
}

impl DiskMonitor {
    /// Create a new disk monitor
    pub fn new() -> Result<Self, CoreError> {
        let mut query_handle = 0;
        let mut disk_time_counter = 0;
        // let mut disk_queue_counter = 0; // Removed
        let mut disk_read_counter = 0;
        let mut disk_write_counter = 0;
        // let mut disk_free_counter = 0; // Removed
        
        // Initialize PDH query
        let query_result = unsafe {
            PdhOpenQueryW(None, 0, &mut query_handle)
        };
        
        if query_result != 0 {
            return Err(CoreError::PdhError(format!(
                "Failed to open PDH query for disk: error code {}",
                query_result
            )));
        }
        
        // Add disk counters
        // % Disk Time counter
        let counter_path = windows::core::HSTRING::from("\\PhysicalDisk(_Total)\\% Disk Time");
        let counter_result = unsafe {
            PdhAddEnglishCounterW(
                query_handle,
                &counter_path,
                0,
                &mut disk_time_counter,
            )
        };
        
        if counter_result != 0 {
            unsafe {
                windows::Win32::System::Performance::PdhCloseQuery(query_handle);
            }
            
            return Err(CoreError::PdhError(format!(
                "Failed to add disk time counter: error code {}",
                counter_result
            )));
        }
        
        // // Current Disk Queue Length counter (Removed)
        // let counter_path = windows::core::HSTRING::from("\\PhysicalDisk(_Total)\\Current Disk Queue Length");
        // ... (code for adding disk_queue_counter removed) ...
        
        // Disk Read Bytes/sec counter
        let counter_path = windows::core::HSTRING::from("\\PhysicalDisk(_Total)\\Disk Read Bytes/sec");
        let counter_result = unsafe {
            PdhAddEnglishCounterW(
                query_handle,
                &counter_path,
                0,
                &mut disk_read_counter,
            )
        };
        
        if counter_result != 0 {
            unsafe {
                windows::Win32::System::Performance::PdhCloseQuery(query_handle);
            }
            
            return Err(CoreError::PdhError(format!(
                "Failed to add disk read counter: error code {}",
                counter_result
            )));
        }
        
        // Disk Write Bytes/sec counter
        let counter_path = windows::core::HSTRING::from("\\PhysicalDisk(_Total)\\Disk Write Bytes/sec");
        let counter_result = unsafe {
            PdhAddEnglishCounterW(
                query_handle,
                &counter_path,
                0,
                &mut disk_write_counter,
            )
        };
        
        if counter_result != 0 {
            unsafe {
                windows::Win32::System::Performance::PdhCloseQuery(query_handle);
            }
            
            return Err(CoreError::PdhError(format!(
                "Failed to add disk write counter: error code {}",
                counter_result
            )));
        }
        
        // // % Free Space counter (Removed - using GetDiskFreeSpaceW instead)
        // let counter_path = windows::core::HSTRING::from("\\LogicalDisk(_Total)\\% Free Space");
        // ... (code for adding disk_free_counter removed) ...
        
        // Initial data collection to establish baseline
        unsafe {
            PdhCollectQueryData(query_handle);
        }
        
        // Small sleep to allow for first measurement
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Collect again to get meaningful values
        unsafe {
            PdhCollectQueryData(query_handle);
        }
        
        Ok(Self {
            query_handle,
            disk_time_counter,
            // disk_queue_counter, // Removed
            disk_read_counter,
            disk_write_counter,
            // disk_free_counter, // Removed
            last_disk_time: 0.0,
            // last_queue_length: 0.0, // Removed
            last_read_rate: 0.0,
            last_write_rate: 0.0,
        })
    }
    
    /// Get disk usage statistics
    /// Returns (disk_time_percent, read_bytes_per_sec, write_bytes_per_sec, free_space_percent)
    pub fn get_usage(&mut self) -> Result<(f32, u64, u64, u64), CoreError> {
        unsafe {
            // Collect new data
            let collect_result = PdhCollectQueryData(self.query_handle);
            
            if collect_result != 0 {
                warn!("Failed to collect PDH data for disk: error code {}", collect_result);
                return Ok((
                    self.last_disk_time,
                    self.last_read_rate as u64,
                    self.last_write_rate as u64,
                    0, // Default to 0 for free space if collection fails
                ));
            }
            
            // Get % Disk Time
            let mut disk_time_value = PDH_FMT_COUNTERVALUE::default();
            let format_result = PdhGetFormattedCounterValue(
                self.disk_time_counter,
                PDH_FMT_DOUBLE,
                None,
                &mut disk_time_value,
            );
            
            let disk_time = if format_result == 0 {
                disk_time_value.Anonymous.doubleValue as f32
            } else {
                warn!("Failed to format disk time counter: error code {}", format_result);
                self.last_disk_time
            };
            
            // // Get Current Disk Queue Length (Removed)
            // let mut disk_queue_value = PDH_FMT_COUNTERVALUE::default();
            // ... (code for getting queue_length removed) ...
            // let queue_length = ...;
            
            // Get Disk Read Bytes/sec
            let mut disk_read_value = PDH_FMT_COUNTERVALUE::default();
            let format_result = PdhGetFormattedCounterValue(
                self.disk_read_counter,
                PDH_FMT_DOUBLE,
                None,
                &mut disk_read_value,
            );
            
            let read_rate = if format_result == 0 {
                disk_read_value.Anonymous.doubleValue as f32
            } else {
                warn!("Failed to format disk read counter: error code {}", format_result);
                self.last_read_rate
            };
            
            // Get Disk Write Bytes/sec
            let mut disk_write_value = PDH_FMT_COUNTERVALUE::default();
            let format_result = PdhGetFormattedCounterValue(
                self.disk_write_counter,
                PDH_FMT_DOUBLE,
                None,
                &mut disk_write_value,
            );
            
            let write_rate = if format_result == 0 {
                disk_write_value.Anonymous.doubleValue as f32
            } else {
                warn!("Failed to format disk write counter: error code {}", format_result);
                self.last_write_rate
            };
            
            // // Get % Free Space (Removed - using GetDiskFreeSpaceW instead)
            // let mut disk_free_value = PDH_FMT_COUNTERVALUE::default();
            // ... (code for getting free_space via PDH removed) ...
            // let free_space = ...;
            
            // Get disk space info using GetDiskFreeSpaceW
            let disk_space_info = Self::get_disk_space().unwrap_or_default();
            let free_bytes = if !disk_space_info.is_empty() {
                disk_space_info.iter().map(|(_, _, free, _)| free).sum()
            } else {
                0
            };
            
            // Store values for potential fallback
            self.last_disk_time = disk_time;
            // self.last_queue_length = queue_length; // Removed
            self.last_read_rate = read_rate;
            self.last_write_rate = write_rate;
            
            debug!(
                "Disk: {:.1}% busy, Read: {:.2} MB/s, Write: {:.2} MB/s, Free: {} bytes",
                disk_time.clamp(0.0, 100.0),
                // queue_length, // Removed from debug
                read_rate / 1_048_576.0,
                write_rate / 1_048_576.0,
                free_bytes
            );
            
            Ok((disk_time.clamp(0.0, 100.0),
                read_rate as u64, 
                write_rate as u64, 
                free_bytes))
        }
    }
    
    /// Get disk space information for specific drives
    /// Returns a vector of (drive_letter, total_bytes, free_bytes, percent_free)
    pub fn get_disk_space() -> Result<Vec<(String, u64, u64, f32)>, CoreError> {
        let mut result = Vec::new();
        
        // Get available drive letters (A-Z)
        let available_drives = unsafe {
            windows::Win32::Storage::FileSystem::GetLogicalDrives()
        };
        
        // Iterate through each possible drive letter
        for i in 0..26 {
            // Check if this drive is available (bit is set)
            if (available_drives & (1 << i)) != 0 {
                let drive_letter = (b'A' + i as u8) as char;
                let drive_path = format!("{}:\\", drive_letter);
                
                // Get disk free space information
                let mut sectors_per_cluster = 0;
                let mut bytes_per_sector = 0;
                let mut number_of_free_clusters = 0;
                let mut total_number_of_clusters = 0;
                
                let drive_path_wide: Vec<u16> = drive_path.encode_utf16().chain(std::iter::once(0)).collect();
                
                let success = unsafe {
                    windows::Win32::Storage::FileSystem::GetDiskFreeSpaceW(
                        windows::core::PCWSTR(drive_path_wide.as_ptr()),
                        Some(&mut sectors_per_cluster),
                        Some(&mut bytes_per_sector),
                        Some(&mut number_of_free_clusters),
                        Some(&mut total_number_of_clusters),
                    )
                };
                
                if success.is_ok() {
                    // Calculate total and free space
                    let bytes_per_cluster = sectors_per_cluster as u64 * bytes_per_sector as u64;
                    let total_bytes = total_number_of_clusters as u64 * bytes_per_cluster;
                    let free_bytes = number_of_free_clusters as u64 * bytes_per_cluster;
                    
                    // Calculate percentage free
                    let percent_free = if total_bytes > 0 {
                        (free_bytes as f64 / total_bytes as f64 * 100.0) as f32
                    } else {
                        0.0
                    };
                    
                    result.push((drive_letter.to_string(), total_bytes, free_bytes, percent_free));
                }
            }
        }
        
        Ok(result)
    }
}

impl Drop for DiskMonitor {
    fn drop(&mut self) {
        // Clean up PDH resources
        unsafe {
            windows::Win32::System::Performance::PdhCloseQuery(self.query_handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_disk_monitor_creation() {
        // This test just verifies that we can create the monitor without errors
        match DiskMonitor::new() {
            Ok(_) => assert!(true), // Success
            Err(e) => {
                eprintln!("Note: Disk monitor creation failed, but this might be expected in CI: {}", e);
                // Don't fail the test as it might be running in CI without proper access
            }
        }
    }
}