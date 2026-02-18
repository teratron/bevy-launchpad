//! Single-instance process lock management.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Lifetime guard that holds the OS file lock for the current process.
#[derive(Debug)]
pub struct InstanceLockGuard {
    file: File,
    lock_file: PathBuf,
}

impl Drop for InstanceLockGuard {
    fn drop(&mut self) {
        let _ = self.file.sync_all();
        let _ = std::fs::remove_file(&self.lock_file);
    }
}

/// Resource that stores the instance lock guard.
#[derive(bevy::prelude::Resource, Debug)]
pub struct SingleInstanceLock {
    pub guard: Option<InstanceLockGuard>,
}

/// Startup error for single-instance protection.
#[derive(Debug, thiserror::Error)]
pub enum SingleInstanceError {
    #[error("another instance is already running (lock file: {lock_file:?})")]
    AlreadyRunning { lock_file: PathBuf },
    #[error("failed to acquire startup lock: {0}")]
    Io(#[from] std::io::Error),
}

/// Acquire a global lock for the process.
///
/// Returns `Ok(Some(guard))` if the lock was acquired, or `Ok(None)` if `allow_multiple` is true.
/// Returns `Err(SingleInstanceError::AlreadyRunning)` if another instance is already holding the lock.
pub fn acquire_single_instance_lock(
    lock_file_path: &Path,
    allow_multiple: bool,
) -> Result<Option<InstanceLockGuard>, SingleInstanceError> {
    if allow_multiple {
        return Ok(None);
    }

    let file = match OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(lock_file_path)
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            if try_clear_stale_lock(lock_file_path).map_err(SingleInstanceError::Io)? {
                OpenOptions::new()
                    .create_new(true)
                    .read(true)
                    .write(true)
                    .open(lock_file_path)
                    .map_err(|retry_err| {
                        if retry_err.kind() == std::io::ErrorKind::AlreadyExists {
                            SingleInstanceError::AlreadyRunning {
                                lock_file: lock_file_path.to_path_buf(),
                            }
                        } else {
                            SingleInstanceError::Io(retry_err)
                        }
                    })?
            } else {
                return Err(SingleInstanceError::AlreadyRunning {
                    lock_file: lock_file_path.to_path_buf(),
                });
            }
        }
        Err(e) => return Err(SingleInstanceError::Io(e)),
    };

    write_lock_metadata(file, lock_file_path).map(Some)
}

fn try_clear_stale_lock(lock_file: &Path) -> std::io::Result<bool> {
    let content = std::fs::read_to_string(lock_file)?;
    let Some(pid) = parse_pid_from_lock(&content) else {
        return Ok(false);
    };

    if is_process_alive(pid) {
        return Ok(false);
    }

    std::fs::remove_file(lock_file)?;
    Ok(true)
}

fn parse_pid_from_lock(content: &str) -> Option<u32> {
    content
        .lines()
        .find_map(|line| line.strip_prefix("pid="))
        .and_then(|pid| pid.trim().parse::<u32>().ok())
}

#[cfg(windows)]
fn is_process_alive(pid: u32) -> bool {
    // Minimal FFI to avoid external crate dependency.
    type Handle = *mut std::ffi::c_void;
    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    const STILL_ACTIVE: u32 = 259;

    unsafe extern "system" {
        fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> Handle;
        fn GetExitCodeProcess(hProcess: Handle, lpExitCode: *mut u32) -> i32;
        fn CloseHandle(hObject: Handle) -> i32;
    }

    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if handle.is_null() {
        return false;
    }

    let mut exit_code: u32 = 0;
    let ok = unsafe { GetExitCodeProcess(handle, &mut exit_code) };
    unsafe { CloseHandle(handle) };

    ok != 0 && exit_code == STILL_ACTIVE
}

#[cfg(all(unix, not(target_os = "macos")))]
fn is_process_alive(pid: u32) -> bool {
    std::path::Path::new("/proc").join(pid.to_string()).exists()
}

#[cfg(target_os = "macos")]
fn is_process_alive(pid: u32) -> bool {
    unsafe {
        // signal 0 checks if process exists and we have permission
        libc::kill(pid as i32, 0) == 0
            || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
    }
}

fn write_lock_metadata(
    mut file: File,
    lock_file: &Path,
) -> Result<InstanceLockGuard, SingleInstanceError> {
    let pid = std::process::id();
    writeln!(&mut file, "pid={pid}").map_err(SingleInstanceError::Io)?;
    file.flush().map_err(SingleInstanceError::Io)?;
    Ok(InstanceLockGuard {
        file,
        lock_file: lock_file.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn lock_is_skipped_when_multiple_instances_enabled() {
        let temp_dir = std::env::temp_dir().join("launchpad-test-1");
        let _ = fs::create_dir_all(&temp_dir);
        let lock_file = temp_dir.join("instance.lock");

        let result =
            acquire_single_instance_lock(&lock_file, true).expect("lock acquire should pass");
        assert!(result.is_none());

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn second_lock_fails_while_first_is_alive() {
        let unique = format!(
            "launchpad-single-instance-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let data_dir = std::env::temp_dir().join(unique);
        fs::create_dir_all(&data_dir).expect("temp lock dir should be created");
        let lock_file = data_dir.join("instance.lock");

        let first = acquire_single_instance_lock(&lock_file, false)
            .expect("first lock should be created")
            .expect("lock guard must exist");
        let second = acquire_single_instance_lock(&lock_file, false);

        assert!(matches!(
            second,
            Err(SingleInstanceError::AlreadyRunning { .. })
        ));

        drop(first);
        let _ = fs::remove_dir_all(data_dir);
    }

    #[test]
    fn stale_lock_is_recovered_when_pid_is_not_alive() {
        let unique = format!(
            "launchpad-stale-lock-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let data_dir = std::env::temp_dir().join(unique);
        fs::create_dir_all(&data_dir).expect("temp lock dir should be created");
        let lock_file = data_dir.join("instance.lock");

        // Write a stale PID
        fs::write(&lock_file, "pid=99999999\n").expect("stale lock file should be created");

        let lock = acquire_single_instance_lock(&lock_file, false)
            .expect("stale lock should be cleared")
            .expect("lock guard should be acquired after stale cleanup");
        drop(lock);

        let _ = fs::remove_dir_all(data_dir);
    }

    #[test]
    fn parse_pid_from_lock_reads_pid_line() {
        assert_eq!(parse_pid_from_lock("pid=123\n"), Some(123));
        assert_eq!(parse_pid_from_lock("foo=bar\npid=42\n"), Some(42));
        assert_eq!(parse_pid_from_lock(""), None);
        assert_eq!(parse_pid_from_lock("pid=abc"), None);
    }
}
