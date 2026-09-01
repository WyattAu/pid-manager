use std::fs;
use tempfile::TempDir;

use proptest::prelude::*;

use pid_manager::pidfile::PidFile;

proptest! {
    #[test]
    fn test_pid_file_write_read_roundtrip(pid in 1u32..u32::MAX) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.pid");

        fs::write(&path, pid.to_string()).unwrap();
        let read = PidFile::read(&path).unwrap();
        prop_assert_eq!(read, Some(pid));
    }

    #[test]
    fn test_is_running_consistency(pid in 1u32..u32::MAX) {
        // is_running should return a consistent result for a given PID
        // (ignoring TOCTOU races, just checking no panic)
        let _ = PidFile::is_running(pid);
    }

    #[test]
    fn test_path_preservation(name in "[a-zA-Z0-9_-]{1,30}") {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(format!("{}.pid", name));

        let pidfile = PidFile::create(&path).unwrap();
        prop_assert_eq!(pidfile.path(), path.as_path());
    }
}
