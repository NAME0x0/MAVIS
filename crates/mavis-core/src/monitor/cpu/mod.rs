// CPU monitoring for Windows systems

use crate::error::CoreError;
use log::{debug, error, warn};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use windows::Win32::System::Performance::{
    PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_DOUBLE, PDH_FMT_COUNTERVALUE,
};

// Global PDH query handle
static PDH_QUERY: Lazy<Mutex<Option<PDHQueryWrapper>>> = Lazy::new(|| Mutex::new(None));

/// Wrapper around PDH query handle for cleanup
struct PDHQueryWrapper {
    query: isize,
    counter: isize,
}

impl Drop for PDHQueryWrapper {
    fn drop(&mut self) {
        // Clean up PDH resources
        unsafe {
            windows::Win32::System::Performance::PdhCloseQuery(self.query);
        }
    }
}

/// CPU monitoring functionality
pub struct CpuMonitor {
    last_usage: f32,
}

impl CpuMonitor {
    /// Create a new CPU monitor
    pub fn new() -> Result<Self, CoreError> {
        // Initialize PDH query once
        let mut pdh_guard = PDH_QUERY.lock().unwrap();
        
        if pdh_guard.is_none() {
            let mut query_handle = 0;
            let mut counter_handle = 0;
            
            // Create PDH query
            let query_result = unsafe {
                PdhOpenQueryW(None, 0, &mut query_handle)
            };
            
            if query_result != 0 {
                return Err(CoreError::PdhError(format!(
                    "Failed to open PDH query: error code {}",
                    query_result
                )));
            }
            
            // Add CPU counter
            let counter_path = windows::core::HSTRING::from("\\Processor(_Total)\\% Processor Time");
            let counter_result = unsafe {
                PdhAddEnglishCounterW(
                    query_handle,
                    &counter_path,
                    0,
                    &mut counter_handle,
                )
            };
            
            if counter_result != 0 {
                unsafe {
                    windows::Win32::System::Performance::PdhCloseQuery(query_handle);
                }
                
                return Err(CoreError::PdhError(format!(
                    "Failed to add CPU counter: error code {}",
                    counter_result
                )));
            }
            
            // Store the query wrapper
            *pdh_guard = Some(PDHQueryWrapper {
                query: query_handle,
                counter: counter_handle,
            });
            
            // Initial data collection to establish baseline
            // First call often returns 0, so we ignore the result
            unsafe {
                PdhCollectQueryData(query_handle);
            }
            
            // Small sleep to allow for first measurement
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            unsafe {
                PdhCollectQueryData(query_handle);
            }
        }
        
        Ok(Self { last_usage: 0.0 })
    }
    
    /// Get current CPU usage as a percentage (0-100)
    pub fn get_usage(&mut self) -> Result<f32, CoreError> {
        let pdh_guard = PDH_QUERY.lock().unwrap();
        
        if let Some(pdh) = &*pdh_guard {
            unsafe {
                // Collect new data
                let collect_result = PdhCollectQueryData(pdh.query);
                
                if collect_result != 0 {
                    warn!("Failed to collect PDH data: error code {}", collect_result);
                    return Ok(self.last_usage); // Return last known value
                }
                
                // Get formatted counter value
                let mut counter_value = PDH_FMT_COUNTERVALUE::default();
                let format_result = PdhGetFormattedCounterValue(
                    pdh.counter,
                    PDH_FMT_DOUBLE,
                    None,
                    &mut counter_value,
                );
                
                if format_result != 0 {
                    warn!("Failed to format counter value: error code {}", format_result);
                    return Ok(self.last_usage); // Return last known value
                }
                
                // Extract and store CPU usage
                let usage = counter_value.Anonymous.doubleValue as f32;
                self.last_usage = usage.clamp(0.0, 100.0);
                
                debug!("Current CPU usage: {:.1}%", self.last_usage);
                
                Ok(self.last_usage)
            }
        } else {
            error!("CPU monitor not properly initialized");
            Ok(0.0)
        }
    }
}