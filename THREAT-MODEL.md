# Threat Model — pid-manager

Status: **v1.0** · Method: STRIDE over the public API surface
(`PidFile`, `DaemonGuard`, `PidError`).

Trust boundaries: (1) the filesystem directory hosting the PID file
(possibly shared or world-writable — e.g. `/tmp`), (2) the content of
pre-existing PID files (attacker-writable), (3) the OS process table
(PIDs are recycled by the kernel).

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Single-instance guarantee | Two daemons running concurrently because a stale PID file was trusted or ignored |
| A2 | Liveness verdict correctness | A recycled PID belonging to an unrelated process reported as "already running" |
| A3 | Filesystem hygiene | Orphaned PID files surviving daemon crashes; PID files left behind on clean shutdown |
| A4 | Availability of the guarded daemon | Malformed or unreadable PID file blocking startup |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Stale PID file from a crashed daemon blocks restart (false positive) or is blindly trusted | Tampering | `PidFile::new`, `is_already_running` | A pre-existing file's content is parsed defensively — malformed content is a typed error, and a valid PID is checked for liveness (`is_running`) before declaring a conflict | `test_invalid_pid_file_content`, `test_already_running_detection`, `test_is_running_with_current_process` |
| T2 | PID recycling: unrelated process reuses the recorded PID | Spoofing | `is_running` | Liveness is delegated to the OS (`kill(pid, 0)`-equivalent); the residual risk of recycled PIDs is documented — see OPEN-1 | `test_is_running_with_current_process` |
| T3 | Daemon dies without cleanup, leaving a ghost PID file | Repudiation | `DaemonGuard` | RAII: dropping the guard removes the file; explicit `release` skips removal intentionally (detached daemon case) | `test_raii_drop_removes_file`, `test_daemon_guard_release` |
| T4 | PID file content tampering feeds wrong data to operators | Tampering | `PidFile::read` | Round-trip read/write is verified; invalid content is rejected rather than propagated | `test_create_read_remove_roundtrip`, `test_invalid_pid_file_content` |
| T5 | Concurrent creation races (two processes create simultaneously) | Spoofing | `PidFile::create` | Creation is atomic (create-new semantics); the second creator observes `is_already_running`-style errors instead of overwriting | `test_already_running_detection` |

## OPEN RISKS (missing mitigations — not fabricated)

- **OPEN-1 — PID recycling remains inherently racy.** After a crash, the
  kernel may hand the recorded PID to an unrelated process; liveness
  checking alone cannot distinguish it. Callers needing stronger
  guarantees should record a start-time/boot-id alongside the PID.
- **OPEN-2 — directory-level attacks (symlink swap, pre-created files in
  world-writable directories) are not defended.** Operators should place
  PID files in root-owned directories (`/var/run`), which is the
  documented deployment expectation.

## Out of Scope

- Signal handling beyond process liveness checks.
- Locking strategies stronger than atomic PID-file creation (e.g.
  `flock`-based single-instance).
- Windows support (Unix process semantics only).

## Residual Risks

- A hostile local user able to write the PID file's directory can
  pre-place a file with a victim's PID, denying startup of the guarded
  daemon (fail-closed, denial only).
