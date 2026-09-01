use std::path::Path;

use crate::error::PidError;
use crate::pidfile::PidFile;

/// RAII guard that manages a PID file for a daemon process.
/// Creates the PID file on construction, removes it on drop.
pub struct DaemonGuard {
    pidfile: Option<PidFile>,
}

impl DaemonGuard {
    /// Create a new daemon guard. Writes the PID file immediately.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, PidError> {
        let pidfile = PidFile::create(path)?;
        Ok(Self {
            pidfile: Some(pidfile),
        })
    }

    /// Check if another instance of this daemon is already running.
    pub fn is_already_running(path: impl AsRef<Path>) -> bool {
        match PidFile::read(&path) {
            Ok(Some(pid)) => PidFile::is_running(pid),
            _ => false,
        }
    }

    /// Explicitly release the PID file without waiting for drop.
    pub fn release(mut self) -> Result<(), PidError> {
        if let Some(pidfile) = self.pidfile.take() {
            pidfile.remove()?;
        }
        Ok(())
    }
}

impl Drop for DaemonGuard {
    fn drop(&mut self) {
        if let Some(pidfile) = self.pidfile.take() {
            let _ = pidfile.remove();
        }
    }
}
