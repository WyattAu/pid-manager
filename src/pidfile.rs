use std::fs;
use std::path::{Path, PathBuf};

use crate::error::PidError;

/// Represents a PID file on disk.
#[derive(Debug)]
pub struct PidFile {
    path: PathBuf,
    pid: u32,
}

impl PidFile {
    /// Create a new PID file. Fails if the file exists and the process is alive.
    pub fn create(path: impl AsRef<Path>) -> Result<Self, PidError> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            if let Some(existing_pid) = Self::read(&path)? {
                if Self::is_running(existing_pid) {
                    return Err(PidError::AlreadyRunning(existing_pid));
                }
            }
        }
        let pid = std::process::id();
        fs::write(&path, pid.to_string()).map_err(PidError::Write)?;
        Ok(Self { path, pid })
    }

    /// Read a PID from an existing PID file.
    pub fn read(path: impl AsRef<Path>) -> Result<Option<u32>, PidError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(None);
        }
        // Safety limit: PID files should never exceed 256 bytes
        let content = fs::read_to_string(path).map_err(PidError::Read)?;
        if content.len() > 256 {
            return Err(PidError::InvalidContent("file too large".into()));
        }
        let content = content.trim();
        if content.is_empty() {
            return Ok(None);
        }
        let pid = content
            .parse::<u32>()
            .map_err(|e| PidError::InvalidContent(e.to_string()))?;
        Ok(Some(pid))
    }

    /// Check if a process with the given PID is running.
    pub fn is_running(pid: u32) -> bool {
        nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), None).is_ok()
    }

    /// Get the path of this PID file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the PID stored in this file.
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Remove the PID file from disk.
    pub fn remove(&self) -> Result<(), PidError> {
        fs::remove_file(&self.path).map_err(PidError::Remove)?;
        Ok(())
    }
}

impl Drop for PidFile {
    fn drop(&mut self) {
        let _ = self.remove();
    }
}
