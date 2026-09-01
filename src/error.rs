use thiserror::Error;

/// Errors that can occur during PID file operations.
#[derive(Error, Debug)]
pub enum PidError {
    /// The PID file already exists and the process is still running.
    #[error("process {0} is still running")]
    AlreadyRunning(u32),
    /// Failed to read the PID file.
    #[error("failed to read PID file: {0}")]
    Read(#[from] std::io::Error),
    /// The PID file contains invalid content.
    #[error("invalid PID file content: {0}")]
    InvalidContent(String),
    /// Failed to write the PID file.
    #[error("failed to write PID file: {0}")]
    Write(std::io::Error),
    /// Failed to remove the PID file.
    #[error("failed to remove PID file: {0}")]
    Remove(std::io::Error),
}
