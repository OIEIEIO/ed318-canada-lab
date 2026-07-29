# v0.3.0 Semantic Audit

## Release identity

- Accepted baseline reviewed: `ed318-canada-lab v0.2.1`
- Target: `ed318-canada-lab v0.3.0`
- Release purpose: ED-318 Implementation and Strategy Reference Library
- Date: 2026-07-29

## Baseline review

Every file in the uploaded v0.2.1 ZIP was enumerated and read. The baseline contained 59 files and 5,951 text lines, including all eight schemas, all vendored examples, existing reports, documentation, manifests, and the Rust placeholder.

## Intended semantic changes

1. Reframe the project from a validator-first laboratory to a reference-first implementation study.
2. Establish controlled terminology separating the standard, reference schemas, reference examples, national implementations, publishing strategies, and derived samples.
3. Add evidence-backed Ireland and Spain profiles from complete supplied source files.
4. Add small deterministic representative samples instead of committing large national source files.
5. Record complete-source size, SHA-256, feature counts, selection indices, selected identifiers, sample size, sample SHA-256, and uncertainty.
6. Document the Czech publication ecosystem while deferring field-level claims until a Czech JSON file is inspected.
7. Recast Canada as a future implementation-strategy study rather than an existing national profile.
8. Defer parser and validator design until the implementation evidence is better understood.

## Preserved semantics

- The project remains non-authoritative and non-operational.
- Imported schema and vendor snapshots remain read-only.
- Provenance, reproducibility, uncertainty, and audit discipline remain core requirements.
- No aircraft-control or geo-awareness safety capability is claimed.
- MIT licensing for project-authored material remains unchanged.

## Dataset evidence reviewed

### Ireland

- Supplied source size: 7,639,550 bytes.
- Top-level type: FeatureCollection.
- Features: 86.
- Observed geometry: MultiPolygon.
- Internal provider: Irish Aviation Authority.
- Representative sample selects source indices 0, 42, and 85.

### Spain

- Aero: 13,017,686 bytes; 1,679 Polygon features; nested layout.
- Infra: 95,114,816 bytes; 14,104 Polygon features; flattened layout.
- Urbano: 114,775 bytes; 4 Polygon features; nested layout.
- Representative samples select first, middle, and final features; Urbano retains all four features because the complete source is already small.

### Czech Republic

- Publication inventory reviewed from user-supplied evidence.
- No Czech JSON inspected.
- Field-level comparison intentionally deferred.

## Verification completed

- Parsed all 21 JSON and GeoJSON files in the target repository successfully.
- Reproduced all four derived samples from exact source feature indices and compared parsed content for equality.
- Confirmed all imported files under `schemas/ed-318/` are byte-for-byte unchanged.
- Confirmed all imported files under `vendor/ED-318/` are byte-for-byte unchanged.
- Checked all relative Markdown links; no broken internal links found.
- Generated complete modified, added, and removed path inventories.

## Rust checks

The execution environment did not contain `cargo`, `rustc`, or `rustfmt`. Therefore these commands were not run here:

```text
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The Rust change is limited to the package version and a formatted three-line placeholder `main.rs`. The user must run the four commands above before accepting the release.

## Known limitations

- Ireland authoritative download URL and redistribution terms remain unverified.
- Spain redistribution terms and retrieval date remain unverified.
- Strict schema validation was intentionally not performed.
- No Czech field-level sample is included.
- The samples are reserialized derived artifacts, not byte slices or unchanged official files.
- The reference-schema GitHub snapshot authority remains described according to its retained provenance rather than elevated by assumption.

## Audit conclusion

The target repository matches the approved v0.3.0 direction: a learning, implementation, and publishing-strategy reference library grounded in real Ireland and Spain data. It does not introduce premature parser or validator capability.
