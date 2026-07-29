# Changelog

All notable project changes are documented here.

## v0.2.1 — 2026-07-29

### Added

- Professional project, architecture, standards, country, and provenance documentation.
- Unchanged ED-318 schema files under `schemas/ed-318/`.
- Unchanged ED-318 examples and helper files under `vendor/ED-318/`.
- ED-318 import provenance manifest and SHA-256 checksum inventory.
- Country classification templates and initial classification notes.
- Release semantic audit and baseline diff report.

### Changed

- Package version advanced from `0.2.0-foundation` to `0.2.1`.
- `src/main.rs` formatted without changing placeholder behaviour.
- Project-authored material licensed under MIT.
- Existing placeholder documentation expanded into the documentation/provenance baseline.

### Not added

- No JSON Schema validator.
- No ED-318 parser or Rust wire model.
- No normalization engine.
- No geo-awareness query engine.
- No operational aircraft integration.

## v0.2.0-foundation — 2026-07-29

- Established repository, documentation, country/profile, dataset, report, and Rust project structure.
- Retired the earlier parser CLI from the accepted baseline.
- Retained a foundation-only Rust executable.
