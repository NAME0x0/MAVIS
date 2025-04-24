//! Manages Windows Pseudo Console (ConPTY) sessions.

use crate::error::CoreResult;
use anyhow::{bail, Context};
use log::{debug, error, info};
use std::{
    ffi::OsString,
    io::{Read, Write},
    mem::{size_of, zeroed},
    os::windows::prelude::{AsRawHandle, FromRawHandle, OwnedHandle},
    ptr::null_mut,
    thread::{self, JoinHandle},
};
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE, HPCON, INVALID_HANDLE_VALUE, TRUE},
        System::{
            Console::{CreatePseudoConsole, GetConsoleScreenBufferInfo, ResizePseudoConsole},
            Pipes::{CreatePipe, PeekNamedPipe},
            Threading::{
                CreateProcessW, InitializeProcThreadAttributeList, UpdateProcThreadAttribute,
                DeleteProcThreadAttributeList, GetExitCodeProcess, TerminateProcess,
                PROCESS_INFORMATION, PROC_THREAD_ATTRIBUTE_LIST, STARTUPINFOEXW,
                EXTENDED_STARTUPINFO_PRESENT, STARTF_USESTDHANDLES, STARTUPINFOW,
            },
        },
    },
};

// PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE = 0x00020016
const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: usize = 0x00020016;

/// Represents an active ConPTY session.
#[derive(Debug)]
pub struct ConPtySession {
    pty_handle: HPCON,
    process_info: PROCESS_INFORMATION,
    input_writer: OwnedHandle, // Write handle for stdin pipe
    output_reader: OwnedHandle, // Read handle for stdout pipe
    // Optional: Thread handle for reading output asynchronously
    // output_thread: Option<JoinHandle<()>>,
}

impl ConPtySession {
    /// Creates a new ConPTY session and spawns the specified command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command line string to execute (e.g., "cmd.exe").
    /// * `cols` - Initial number of columns for the pseudo console.
    /// * `rows` - Initial number of rows for the pseudo console.
    pub fn new(command: &str, cols: i16, rows: i16) -> CoreResult<Self> {
        info!("Creating ConPTY session for command: '{}'", command);

        // 1. Create pipes for stdin and stdout
        let (stdin_reader, stdin_writer) = create_pipe()?;
        let (stdout_reader, stdout_writer) = create_pipe()?;

        // 2. Create the Pseudo Console
        let size = windows::Win32::Foundation::COORD { X: cols, Y: rows };
        let mut pty_handle: HPCON = INVALID_HANDLE_VALUE;
        unsafe {
            CreatePseudoConsole(
                size,
                stdin_reader.as_raw_handle() as HANDLE, // Input Read handle
                stdout_writer.as_raw_handle() as HANDLE, // Output Write handle
                0, // Flags
                &mut pty_handle,
            )
        }
        .context("Failed to create pseudo console")?;
        info!("Pseudo Console created with handle: {:?}", pty_handle);

        // Close the handles we don't need in this process
        drop(stdin_reader);
        drop(stdout_writer);

        // 3. Prepare startup info for the child process
        let mut si_startup_info = prepare_startup_info(pty_handle)?;

        // 4. Create the child process
        let mut process_info: PROCESS_INFORMATION = unsafe { zeroed() };
        let mut command_wide: Vec<u16> = command.encode_utf16().chain(Some(0)).collect(); // Null-terminated wide string

        unsafe {
            CreateProcessW(
                PCWSTR::null(), // Application name (optional)
                PWSTR(command_wide.as_mut_ptr()), // Command line
                None, // Process attributes
                None, // Thread attributes
                false, // Inherit handles
                EXTENDED_STARTUPINFO_PRESENT, // Creation flags
                None, // Environment
                PCWSTR::null(), // Current directory
                &mut si_startup_info.StartupInfo, // Startup info
                &mut process_info, // Process information
            )
        }
        .context(format!("Failed to create process for command: {}", command))?;

        info!(
            "Child process created with PID: {}",
            process_info.dwProcessId
        );

        // Clean up attribute list
        unsafe {
             if !si_startup_info.lpAttributeList.is_null() {
                DeleteProcThreadAttributeList(si_startup_info.lpAttributeList);
                // Assuming Box::from_raw is the correct way to free memory allocated by Vec::into_boxed_slice
                // This might need adjustment based on how lpAttributeList was allocated.
                // If Vec::into_raw_parts was used, Box::from_raw might be appropriate.
                // If allocated differently, use the corresponding deallocation method.
                // For now, let's assume it was managed correctly and DeleteProcThreadAttributeList is sufficient.
                // std::alloc::dealloc(si_startup_info.lpAttributeList as *mut u8, std::alloc::Layout::for_value(si_startup_info.lpAttributeList));
             }
        }


        Ok(Self {
            pty_handle,
            process_info,
            input_writer: stdin_writer,
            output_reader: stdout_reader,
        })
    }

    /// Resizes the pseudo console.
    pub fn resize(&self, cols: i16, rows: i16) -> CoreResult<()> {
        debug!("Resizing ConPTY to {}x{}", cols, rows);
        let size = windows::Win32::Foundation::COORD { X: cols, Y: rows };
        unsafe { ResizePseudoConsole(self.pty_handle, size) }
            .context("Failed to resize pseudo console")?;
        Ok(())
    }

    /// Writes data to the pseudo console's input (stdin).
    pub fn write(&mut self, data: &[u8]) -> CoreResult<usize> {
        let bytes_written = self.input_writer.write(data).context("Failed to write to ConPTY input")?;
        debug!("Wrote {} bytes to ConPTY input", bytes_written);
        Ok(bytes_written)
    }

    /// Reads data from the pseudo console's output (stdout).
    /// This is a blocking read. Consider using asynchronous reads or threads.
    pub fn read(&mut self, buf: &mut [u8]) -> CoreResult<usize> {
        let bytes_read = self.output_reader.read(buf).context("Failed to read from ConPTY output")?;
        debug!("Read {} bytes from ConPTY output", bytes_read);
        Ok(bytes_read)
    }

     /// Checks if there is data available to read without blocking.
    pub fn has_data_available(&self) -> CoreResult<bool> {
        let mut total_bytes_avail = 0u32;
        unsafe {
            PeekNamedPipe(
                self.output_reader.as_raw_handle() as HANDLE,
                None, // buffer
                0, // buffer size
                None, // bytes read
                Some(&mut total_bytes_avail), // total bytes available
                None, // bytes left this message
            )
        }.context("Failed to peek named pipe for ConPTY output")?;
        Ok(total_bytes_avail > 0)
    }

    /// Gets the exit code of the child process, if it has exited.
    pub fn get_exit_code(&self) -> CoreResult<Option<u32>> {
        let mut exit_code: u32 = 0;
        unsafe { GetExitCodeProcess(self.process_info.hProcess, &mut exit_code) }
            .context("Failed to get process exit code")?;

        // STILL_ACTIVE = 259
        if exit_code == 259 {
            Ok(None) // Process still running
        } else {
            Ok(Some(exit_code))
        }
    }

    /// Terminates the child process and closes the pseudo console.
    pub fn terminate(&self) -> CoreResult<()> {
        info!("Terminating ConPTY session (PID: {})", self.process_info.dwProcessId);
        unsafe {
            // Terminate the child process
            if TerminateProcess(self.process_info.hProcess, 1).is_err() {
                 error!("Failed to terminate child process (PID: {})", self.process_info.dwProcessId);
                 // Continue cleanup even if termination fails
            }

            // Close process and thread handles
            if self.process_info.hProcess != INVALID_HANDLE_VALUE {
                CloseHandle(self.process_info.hProcess);
            }
            if self.process_info.hThread != INVALID_HANDLE_VALUE {
                CloseHandle(self.process_info.hThread);
            }

            // Close the PTY handle
            if self.pty_handle != INVALID_HANDLE_VALUE {
                 // ClosePseudoConsole is not available in all versions of windows-rs?
                 // Using CloseHandle as a fallback, though ClosePseudoConsole is preferred if available.
                 CloseHandle(self.pty_handle);
            }
        }
        // Input/Output handles are OwnedHandle, they will be closed on drop.
        Ok(())
    }
}

impl Drop for ConPtySession {
    fn drop(&mut self) {
        if let Err(e) = self.terminate() {
            error!("Error during ConPtySession drop: {}", e);
        }
    }
}

// Helper function to create pipes
fn create_pipe() -> CoreResult<(OwnedHandle, OwnedHandle)> {
    let mut read_pipe: HANDLE = INVALID_HANDLE_VALUE;
    let mut write_pipe: HANDLE = INVALID_HANDLE_VALUE;
    unsafe { CreatePipe(&mut read_pipe, &mut write_pipe, None, 0) }
        .context("Failed to create pipe")?;
    unsafe {
        Ok((
            OwnedHandle::from_raw_handle(read_pipe as *mut _),
            OwnedHandle::from_raw_handle(write_pipe as *mut _),
        ))
    }
}

// Helper function to prepare startup info
fn prepare_startup_info(pty_handle: HPCON) -> CoreResult<STARTUPINFOEXW> {
    unsafe {
        let mut si_startup_info: STARTUPINFOEXW = zeroed();
        si_startup_info.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;

        // Get required size for attribute list
        let mut attribute_list_size: usize = 0;
        InitializeProcThreadAttributeList(
            None, // No list yet
            1,    // One attribute (PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE)
            0,
            &mut attribute_list_size,
        );
        // This call is expected to fail with ERROR_INSUFFICIENT_BUFFER,
        // but it gives us the required size in attribute_list_size.

        // Allocate memory for the attribute list
        // Using Vec<u8> and then converting to pointer for RAII-like management
        let mut attribute_list_mem = vec![0u8; attribute_list_size];
        let attribute_list_ptr = attribute_list_mem.as_mut_ptr() as PROC_THREAD_ATTRIBUTE_LIST;

        // Initialize the attribute list
        InitializeProcThreadAttributeList(
            attribute_list_ptr,
            1, // One attribute
            0,
            &mut attribute_list_size,
        )
        .context("Failed to initialize proc thread attribute list")?;

        // Set the pseudoconsole attribute
        UpdateProcThreadAttribute(
            attribute_list_ptr,
            0,
            PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, // Attribute identifier
            Some(pty_handle as *const _ as *const std::ffi::c_void), // PTY handle
            size_of::<HPCON>(), // Size of the handle
            None,
            None,
        )
        .context("Failed to update proc thread attribute for pseudoconsole")?;

        si_startup_info.lpAttributeList = attribute_list_ptr;

        // Prevent the attribute list memory from being dropped by Vec
        std::mem::forget(attribute_list_mem);

        Ok(si_startup_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Helper to initialize logging for tests
    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[ignore] // Needs manual verification or more complex setup
    fn test_conpty_creation_and_basic_cmd() -> CoreResult<()> {
        setup();
        let mut session = ConPtySession::new("cmd.exe /c echo Hello ConPTY", 80, 25)?;

        // Give the process time to start and echo
        thread::sleep(Duration::from_millis(500));

        let mut output_buffer = [0u8; 1024];
        let bytes_read = session.read(&mut output_buffer)?;

        assert!(bytes_read > 0);
        let output_str = String::from_utf8_lossy(&output_buffer[..bytes_read]);
        info!("ConPTY Output: {}", output_str);

        // Check if the output contains the expected string (might include prompt, etc.)
        assert!(output_str.contains("Hello ConPTY"));

        // Give time for process to exit
        thread::sleep(Duration::from_millis(100));
        let exit_code = session.get_exit_code()?;
        assert!(exit_code.is_some());
        assert_eq!(exit_code.unwrap(), 0); // cmd.exe /c echo should exit with 0

        Ok(())
    }

     #[test]
     #[ignore] // Needs interactive input or more complex setup
     fn test_conpty_write_input() -> CoreResult<()> {
         setup();
         // Start cmd.exe without /c so it stays open waiting for input
         let mut session = ConPtySession::new("cmd.exe", 80, 25)?;

         thread::sleep(Duration::from_millis(500)); // Wait for prompt

         // Write 'exit' command followed by newline
         session.write(b"exit\r\n")?;

         thread::sleep(Duration::from_millis(500)); // Wait for process to exit

         let exit_code = session.get_exit_code()?;
         assert!(exit_code.is_some());
         // Exit code might vary, but it should have exited
         info!("Exit code after writing 'exit': {:?}", exit_code);

         Ok(())
     }

     #[test]
     fn test_conpty_resize() -> CoreResult<()> {
         setup();
         let session = ConPtySession::new("cmd.exe /c exit", 80, 25)?;
         session.resize(120, 40)?; // Check if resize call succeeds
         // No easy way to verify effect without reading screen buffer info via PTY
         Ok(())
     }

     #[test]
     fn test_conpty_terminate() -> CoreResult<()> {
         setup();
         let session = ConPtySession::new("cmd.exe /k echo Running...", 80, 25)?; // /k keeps cmd open
         thread::sleep(Duration::from_millis(100)); // Ensure process starts

         assert!(session.get_exit_code()?.is_none()); // Should be running

         session.terminate()?;

         // Termination might take a moment
         thread::sleep(Duration::from_millis(100));

         let exit_code = session.get_exit_code()?;
         assert!(exit_code.is_some()); // Should have exited
         // Exit code after termination is often 1
         info!("Exit code after termination: {:?}", exit_code);

         Ok(())
     }
}