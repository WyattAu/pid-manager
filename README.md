# pid-manager

PID file management with RAII cleanup and process liveness checks.

## Features

- **PidFile** — Create, read, and manage PID files with automatic cleanup
- **DaemonGuard** — RAII guard for daemon process lifecycle
- **is_running** — Check if a process is alive via signal probe
- Automatic stale PID detection and cleanup
- Typed errors with `thiserror`

## Installation

```bash
cargo add pid-manager
```

## Quick Start

```rust
use pid_manager::{PidFile, DaemonGuard, error::PidError};

fn main() -> Result<(), PidError> {
    // Create a PID file (fails if another instance is running)
    let _guard = DaemonGuard::new("/var/run/myapp.pid")?;

    // Or use PidFile directly
    let pidfile = PidFile::create("/var/run/myapp.pid")?;
    println!("PID: {}", pidfile.pid());

    // Check if a process is running
    if PidFile::is_running(1234) {
        println!("Process 1234 is alive");
    }

    // PID file is automatically removed when dropped
    Ok(())
}
```

## API Reference

### PidFile

```rust
// Create a PID file (error if process already running)
let pidfile = PidFile::create("/var/run/app.pid")?;

// Read a PID from an existing file
let pid = PidFile::read("/var/run/app.pid")?;

// Check if a process is running
let alive = PidFile::is_running(pid);

// Accessors
pidfile.path();  // &Path
pidfile.pid();   // u32

// Explicit cleanup (also happens on drop)
pidfile.remove()?;
```

### DaemonGuard

```rust
// RAII guard — writes PID file, removes on drop
let guard = DaemonGuard::new("/var/run/daemon.pid")?;

// Check if another instance is running
if DaemonGuard::is_already_running("/var/run/daemon.pid") {
    return Err(...);
}

// Explicit release
guard.release()?;
```

### Errors

```rust
pub enum PidError {
    AlreadyRunning(u32),    // PID file exists and process is alive
    Read(io::Error),        // Failed to read PID file
    InvalidContent(String), // PID file contains invalid data
    Write(io::Error),       // Failed to write PID file
    Remove(io::Error),      // Failed to remove PID file
}
```

## License

MIT OR Apache-2.0
