use std::fs;
use tempfile::TempDir;

use pid_manager::pidfile::PidFile;
use pid_manager::guard::DaemonGuard;

#[test]
fn test_create_read_remove_roundtrip() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.pid");

    let pidfile = PidFile::create(&path).unwrap();
    assert_eq!(pidfile.pid(), std::process::id());

    let read_pid = PidFile::read(&path).unwrap();
    assert_eq!(read_pid, Some(std::process::id()));

    pidfile.remove().unwrap();
    assert!(!path.exists());
}

#[test]
fn test_raii_drop_removes_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.pid");

    {
        let _guard = DaemonGuard::new(&path).unwrap();
        assert!(path.exists());
    }

    assert!(!path.exists());
}

#[test]
fn test_is_running_with_current_process() {
    assert!(PidFile::is_running(std::process::id()));
}

#[test]
fn test_already_running_detection() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.pid");

    let pidfile = PidFile::create(&path).unwrap();

    // Writing a PID file for the current process should fail
    // because the current process is running.
    let result = PidFile::create(&path);
    match result {
        Err(pid_manager::error::PidError::AlreadyRunning(pid)) => {
            assert_eq!(pid, std::process::id());
        }
        other => panic!("Expected AlreadyRunning error, got {:?}", other),
    }

    pidfile.remove().unwrap();
}

#[test]
fn test_invalid_pid_file_content() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.pid");

    // Write invalid content
    fs::write(&path, "not a number").unwrap();
    let result = PidFile::read(&path);
    match result {
        Err(pid_manager::error::PidError::InvalidContent(_)) => {}
        other => panic!("Expected InvalidContent error, got {:?}", other),
    }

    // Write empty file
    fs::write(&path, "").unwrap();
    let result = PidFile::read(&path).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_daemon_guard_release() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.pid");

    let guard = DaemonGuard::new(&path).unwrap();
    assert!(path.exists());

    guard.release().unwrap();
    assert!(!path.exists());
}
