# Requirements — pid-manager

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement. Threat
IDs reference `THREAT-MODEL.md`.

Scope note: `pid-manager` provides PID file management — atomic creation,
defensive parsing, process liveness checks, and RAII cleanup via
`DaemonGuard`.

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-PM-001 | `PidFile::create` writes the current PID atomically; `read` returns it; `remove` deletes the file | MUST |
| REQ-PM-002 | `is_already_running` reports a conflict when a live process owns the recorded PID | MUST |
| REQ-PM-003 | `is_running(pid)` reports liveness for the current process and (by OS semantics) for live foreign PIDs | MUST |
| REQ-PM-004 | `DaemonGuard` removes the PID file on drop (RAII); `release` detaches the guard and skips removal | MUST |
| REQ-PM-005 | `PidError` covers creation, parse, and liveness failures with typed variants | SHOULD |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-PM-100 | Malformed PID-file content (non-numeric, empty, huge) produces a typed error, never a panic or a bogus liveness verdict (T1) | MUST |
| REQ-PM-101 | A stale PID file whose recorded PID is dead does not block startup — liveness is re-checked, not trusted from the file (T1, T2) | MUST |
| REQ-PM-102 | Concurrent creators cannot silently overwrite each other: the second creator gets an error (T5) | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-PM-200 | Create → read → remove round-trips cleanly; removal is idempotent from the guard path | MUST |
| REQ-PM-201 | Drop-based cleanup works even when the daemon body returns early or errors (guard-scoped) | MUST |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-PM-001 | `test_create_read_remove_roundtrip` (`src/lib.rs` tests) | unit |
| REQ-PM-002 | `test_already_running_detection` | unit |
| REQ-PM-003 | `test_is_running_with_current_process` | unit |
| REQ-PM-004 | `test_raii_drop_removes_file`, `test_daemon_guard_release` | unit |
| REQ-PM-005 | `test_invalid_pid_file_content` (error path), all-variants display checks | unit |
| REQ-PM-100 | `test_invalid_pid_file_content` | unit |
| REQ-PM-101 | `test_already_running_detection`, `test_is_running_with_current_process` | unit |
| REQ-PM-102 | `test_already_running_detection` | unit |
| REQ-PM-200 | `test_create_read_remove_roundtrip`, `test_daemon_guard_release` | unit |
| REQ-PM-201 | `test_raii_drop_removes_file` | unit |

## Test Count

- 9 `#[test]` functions in the unit suite.
- All-features suite passes with 0 failures; no-default-features suite passes.
