//! PID file management with RAII cleanup and process liveness checks.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Error types for PID file operations.
pub mod error;
/// RAII guard for daemon process lifecycle.
pub mod guard;
/// PID file creation, reading, and management.
pub mod pidfile;
