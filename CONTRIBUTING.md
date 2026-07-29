# Contributing

## Before making changes

Read `PROJECT_PHILOSOPHY.md`, `docs/TERMINOLOGY.md`, `datasets/README.md`, and the relevant national implementation profile.

## Evidence discipline

Do not use the words “official schema,” “example schema,” or “country schema” without qualification. Use the controlled terminology in `docs/TERMINOLOGY.md`.

Claims about national publications must identify the source evidence. Distinguish facts observed directly in supplied files from publisher-page metadata and from engineering interpretation.

## External datasets

Do not commit large national source files by default. Add:

- a provenance manifest;
- complete-file size and SHA-256 checksum;
- authoritative download location where known;
- retrieval date when known;
- a deterministic derived sample when redistribution is appropriate;
- a clear transformation disclosure.

Never call a derived sample an unchanged official dataset.

## Reference material

Do not edit imported files under `schemas/ed-318/` or `vendor/ED-318/` in place. Introduce a separately identified snapshot when upstream material changes.

## National profiles

Use the common profile questions:

- Who publishes the data?
- What is the publication product?
- How is it organized?
- What geometry is used?
- Where are vertical limits represented?
- How are authorities and schedules represented?
- What lifecycle and integrity information is published?
- How closely does it resemble the reference schemas?
- Which claims remain unverified?

## Software changes

Software capability is not part of v0.3.0. Future Rust changes require an approved design, tests, and these checks:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Release audit

Each release must record:

- accepted baseline and target version;
- complete reviewed-file inventory;
- modified, added, and removed files;
- semantic changes and preserved behaviour;
- dataset provenance and sampling changes;
- validation and build checks actually run;
- unresolved limitations.
