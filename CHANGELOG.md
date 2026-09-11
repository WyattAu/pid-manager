# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

## [0.2.0] - 2026-09-11

### Changed
- **Breaking:** removed the unused optional `serde` feature (flagged by
  cargo-machete; no code path referenced it). Downstream crates enabling
  `pid-manager/features = ["serde"]` must drop the flag. Minor-bump
  versioning (0.2.0) preserves the pre-1.0 semver contract.

## [0.1.0] - 2026-09-05

### Added
- Initial public release.
