# PATTERN_ADOPTION: pid-manager

Relationship to the WyattAu pattern canon. Statuses follow the closed
vocabulary: adopted, partial, planned, refused, n/a. Evidence paths are
relative to this repo.

| Pattern | Status | Evidence |
|---|---|---|
| P105 | partial | Single-instance lock primitive for daemons and scheduled jobs: src/pidfile.rs:15-23 (create fails with AlreadyRunning while a live holder exists), src/guard.rs:8-19 and src/guard.rs:38-44 (RAII release on drop). The crate supplies the guard; schedule policy and overlap handling stay caller-owned |
| P94 | partial | Liveness probe only: src/pidfile.rs:51-53 (signal probe). No heartbeat loop and no wedge detection; an alive probe says nothing about a wedged process, which is the failure mode P94 targets |
