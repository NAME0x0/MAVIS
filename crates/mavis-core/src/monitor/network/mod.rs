// Network monitoring for Windows systems

use crate::error::CoreError;
use log::{debug, warn};
use std::time::{Duration, Instant};
use windows::Win32::System::Performance::{
    PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_LARGE, PDH_FMT_COUNTERVALUE,
};

/// Network monitoring functionality
pub struct NetworkMonitor {
    query_handle: isize,
    bytes_in_counter: isize,
    bytes_out_counter: isize,
    last_bytes_in: u64,
    last_bytes_out: u64,
    last_in_rate: u64,
    last_out_rate: u64,
    last_check: Instant,
}

impl NetworkMonitor {
    /// Create a new network monitor
    pub fn new() -> Result<Self, CoreError> {
        let mut query_handle = 0;
        let mut bytes_in_counter = 0;
        let mut bytes_out_counter = 0;
        
        // Initialize PDH query
        let query_result = unsafe {
            PdhOpenQueryW(None, 0, &mut query_handle)
        };
        
        if query_result != 0 {
            return Err(CoreError::PdhError(format!(
                "Failed to open PDH query for network: error code {}",
                query_result
            )));
        }
        
        // Add network counter for bytes received
        let counter_path = windows::core::HSTRING::from("\\Network Interface(*)\\Bytes Received/sec");
        let counter_result = unsafe {
            PdhAddEnglishCounterW(
                query_handle,
                &counter_path,
                0,
                &mut bytes_in_counter,
            )
        };
        
        if counter_result != 0 {
            unsafe {
                windows::Win32::System::Performance::PdhCloseQuery(query_handle);
            }
            
            return Err(CoreError::PdhError(format!(
                "Failed to add network bytes received counter: error code {}",
                counter_result
            )));
        }
        
        // Add network counter for bytes sent
        let counter_path = windows::core::HSTRING::from("\\Network Interface(*)\\Bytes Sent/sec");
        let counter_result = unsafe {
            PdhAddEnglishCounterW(
                query_handle,
                &counter_path,
                0,
                &mut bytes_out_counter,
            )
        };
        
        if counter_result != 0 {
            unsafe {
                windows::Win32::System::Performance::PdhCloseQuery(query_handle);
            }
            
            return Err(CoreError::PdhError(format!(
                "Failed to add network bytes sent counter: error code {}",
                counter_result
            )));
        }
        
        // Initial data collection to establish baseline
        unsafe {
            PdhCollectQueryData(query_handle);
        }
        
        // Small sleep to allow for first measurement
        std::thread::sleep(Duration::from_millis(100));
        
        let monitor = Self {
            query_handle,
            bytes_in_counter,
            bytes_out_counter,
            last_bytes_in: 0,
            last_bytes_out: 0,
            last_in_rate: 0,
            last_out_rate: 0,
            last_check: Instant::now(),
        };
        
        Ok(monitor)
    }
    
    /// Get current network usage in bytes per second: (download_bytes_per_sec, upload_bytes_per_sec)
    pub fn get_usage(&mut self) -> Result<(u64, u64), CoreError> {
        unsafe {
            // Collect new data
            let collect_result = PdhCollectQueryData(self.query_handle);
            
            if collect_result != 0 {
                warn!("Failed to collect PDH data for network: error code {}", collect_result);
                return Ok((self.last_in_rate, self.last_out_rate));
            }
            
            // Get network bytes received
            let mut bytes_in_value = PDH_FMT_COUNTERVALUE::default();
            let format_result = PdhGetFormattedCounterValue(
                self.bytes_in_counter,
                PDH_FMT_LARGE,
                None,
                &mut bytes_in_value,
            );
            
            if format_result != 0 {
                warn!("Failed to format network bytes received counter: error code {}", format_result);
                return Ok((self.last_in_rate, self.last_out_rate));
            }
            
            // Get network bytes sent
            let mut bytes_out_value = PDH_FMT_COUNTERVALUE::default();
            let format_result = PdhGetFormattedCounterValue(
                self.bytes_out_counter,
                PDH_FMT_LARGE,
                None,
                &mut bytes_out_value,
            );
            
            if format_result != 0 {
                warn!("Failed to format network bytes sent counter: error code {}", format_result);
                return Ok((self.last_in_rate, self.last_out_rate));
            }
            
            // Calculate rates based on counter values
            let bytes_in = bytes_in_value.Anonymous.largeValue as u64;
            let bytes_out = bytes_out_value.Anonymous.largeValue as u64;
            
            // Windows already provides bytes per second, so we can use these values directly
            self.last_in_rate = bytes_in;
            self.last_out_rate = bytes_out;
            
            debug!(
                "Network: {:.2} MB/s down, {:.2} MB/s up",
                bytes_in as f64 / 1_048_576.0,
                bytes_out as f64 / 1_048_576.0
            );
            
            Ok((bytes_in, bytes_out))
        }
    }
}

impl Drop for NetworkMonitor {
    fn drop(&mut self) {
        // Clean up PDH resources
        unsafe {
            windows::Win32::System::Performance::PdhCloseQuery(self.query_handle);
        }
    }
}