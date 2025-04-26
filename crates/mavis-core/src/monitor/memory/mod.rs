// Memory monitoring for Windows systems

use crate::error::CoreError;
use log::debug;
use windows::Win32::System::Performance::{
    PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_LARGE, PDH_FMT_COUNTERVALUE,
};
use windows::Win32::System::SystemInformation::{
    GlobalMemoryStatusEx, MEMORYSTATUSEX,
};

/// Memory monitoring functionality
pub struct MemoryMonitor {
    query_handle: isize,
    available_bytes_counter: isize,
    last_total: u64,
    last_used: u64,
    last_percentage: f32,
}

impl MemoryMonitor {
    /// Create a new memory monitor
    pub fn new() -> Result<Self, CoreError> {
        let mut query_handle = 0;
        let mut available_bytes_counter = 0;
        
        // Initialize PDH query
        let query_result = unsafe {
            PdhOpenQueryW(None, 0, &mut query_handle)
        };
        
        if query_result != 0 {
            return Err(CoreError::PdhError(format!(
                "Failed to open PDH query for memory: error code {}",
                query_result
            )));
        }
        
        // Add memory counter for available bytes
        let counter_path = windows::core::HSTRING::from("\\Memory\\Available Bytes");
        let counter_result = unsafe {
            PdhAddEnglishCounterW(
                query_handle,
                &counter_path,
                0,
                &mut available_bytes_counter,
            )
        };
        
        if counter_result != 0 {
            unsafe {
                windows::Win32::System::Performance::PdhCloseQuery(query_handle);
            }
            
            return Err(CoreError::PdhError(format!(
                "Failed to add memory available counter: error code {}",
                counter_result
            )));
        }
        
        // Initial data collection to establish baseline
        unsafe {
            PdhCollectQueryData(query_handle);
        }
        
        Ok(Self {
            query_handle,
            available_bytes_counter,
            last_total: 0,
            last_used: 0,
            last_percentage: 0.0,
        })
    }
    
    /// Get current memory usage: (total_bytes, used_bytes, usage_percentage)
    pub fn get_usage(&mut self) -> Result<(u64, u64, f32), CoreError> {
        // Use GlobalMemoryStatusEx for more accurate memory information
        let mut memory_status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        
        let status_result = unsafe {
            GlobalMemoryStatusEx(&mut memory_status)
        };
        
        if status_result.is_err() {
            // If GlobalMemoryStatusEx fails, fallback to PDH counters
            return self.get_usage_pdh();
        }
        
        let total_physical_memory = memory_status.ullTotalPhys;
        let available_physical_memory = memory_status.ullAvailPhys;
        let used_physical_memory = total_physical_memory - available_physical_memory;
        
        let memory_percentage = if total_physical_memory > 0 {
            (used_physical_memory as f64 / total_physical_memory as f64 * 100.0) as f32
        } else {
            0.0
        };
        
        // Store values for potential fallback
        self.last_total = total_physical_memory;
        self.last_used = used_physical_memory;
        self.last_percentage = memory_percentage;
        
        debug!(
            "Memory: {:.1} GB total, {:.1} GB used ({:.1}%)",
            total_physical_memory as f64 / 1_073_741_824.0,
            used_physical_memory as f64 / 1_073_741_824.0,
            memory_percentage
        );
        
        Ok((total_physical_memory, used_physical_memory, memory_percentage))
    }
    
    // Fallback method using PDH counter for Available Bytes
    fn get_usage_pdh(&mut self) -> Result<(u64, u64, f32), CoreError> {
        // Still try to get total physical memory accurately if possible
        let mut memory_status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        let total_physical_memory = if unsafe { GlobalMemoryStatusEx(&mut memory_status) }.is_ok() {
            memory_status.ullTotalPhys
        } else {
            // Very unlikely fallback, use last known total or 0
            self.last_total
        };

        if total_physical_memory == 0 {
            // Cannot calculate percentage without total
            return Ok((0, 0, 0.0));
        }

        unsafe {
            // Collect new PDH data
            let collect_result = PdhCollectQueryData(self.query_handle);
            if collect_result != 0 {
                log::warn!("PDH fallback: Failed to collect query data: {}", collect_result);
                // Return last known values if collection fails
                return Ok((self.last_total, self.last_used, self.last_percentage));
            }

            // Get Available Bytes counter
            let mut pdh_available_bytes = PDH_FMT_COUNTERVALUE::default();
            let format_result = PdhGetFormattedCounterValue(
                self.available_bytes_counter,
                PDH_FMT_LARGE,
                None,
                &mut pdh_available_bytes,
            );

            if format_result != 0 {
                log::warn!("PDH fallback: Failed to format Available Bytes counter: {}", format_result);
                return Ok((self.last_total, self.last_used, self.last_percentage));
            }

            // Calculate usage based on total physical and available bytes
            let available = pdh_available_bytes.Anonymous.largeValue as u64;
            let used = if total_physical_memory > available {
                total_physical_memory - available
            } else {
                0 // Available shouldn't exceed total, but handle defensively
            };

            let percentage = (used as f64 / total_physical_memory as f64 * 100.0) as f32;

            // Update last known values
            self.last_total = total_physical_memory;
            self.last_used = used;
            self.last_percentage = percentage;

            debug!(
                "Memory (PDH Fallback): {:.1} GB total, {:.1} GB used ({:.1}%)",
                total_physical_memory as f64 / 1_073_741_824.0,
                used as f64 / 1_073_741_824.0,
                percentage
            );

            Ok((total_physical_memory, used, percentage))
        }
    }
}

impl Drop for MemoryMonitor {
    fn drop(&mut self) {
        // Clean up PDH resources
        unsafe {
            windows::Win32::System::Performance::PdhCloseQuery(self.query_handle);
        }
    }
}