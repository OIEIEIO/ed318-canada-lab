# Contributing

## Before making changes

Read `PROJECT_PHILOSOPHY.md`, `docs/architecture/ARCHITECTURE.md`, and `datasets/PROVENANCE_SPEC.md`. Changes must preserve the distinction between official sources, project implementation, normalized output, and national profiles.

## Development checks

For Rust changes, run:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

A contribution that changes behaviour must add or update automated tests. A documentation-only release may retain zero tests if it does not claim software capability.

## Official and authoritative files

Do not edit imported schemas, examples, licences, or source datasets in place. Introduce a new snapshot when upstream content changes. Record provenance and checksums before using the files.

Project commentary about an official file belongs outside the imported snapshot.

## Provenance

Every new external file must identify, as available:

- publisher or authority;
- source URL;
- retrieval date and time;
- dataset or release version;
- commit or immutable identifier;
- licence or terms;
- SHA-256 checksum;
- whether the file was modified;
- known gaps or uncertainty.

Use `countries/PROVENANCE_TEMPLATE.yaml` and `datasets/PROVENANCE_SPEC.md`.

## Country classifications

Do not describe material as ED-318-compliant without evidence. Use the classifications defined in `docs/countries/COUNTRY_TEMPLATE.md`. Proposed and experimental Canadian content must not be placed in `profiles/canada/official/`.

## Code design

Keep validation, exact wire models, normalization, and country interpretation in separate modules. Avoid lossy transformations. Errors should identify the input location and actionable cause. New dependencies require a documented reason and licence review.

## Documentation

Update the README, architecture, roadmap, changelog, semantic audit, and diff report when a change affects their claims. Examples must state whether they are official, copied, normalized, or synthetic.

## Release audit

Each release should include:

- baseline and target versions;
- complete reviewed-file inventory;
- modified-file list;
- semantic changes and non-changes;
- build, lint, format, and test results;
- provenance changes;
- unresolved risks.

## Pull requests

Keep changes focused. Explain why the change is needed, which layer it affects, which files were modified, what tests were run, and whether any external material was added.
