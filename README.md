# ed318-canada-lab

`ed318-canada-lab` is a neutral, auditable engineering laboratory for studying, validating, implementing, and comparing EUROCAE ED-318 geographical-zone data and future Canadian RPAS geo-awareness profiles.

## Release status

This repository is **v0.2.1**, the documentation and provenance baseline. It intentionally contains no operational parser, validator, geo-awareness engine, or aircraft-control integration. The Rust binary remains a foundation placeholder. The first planned software capability is JSON Schema validation in a later release.

The project is not a regulatory authority. Nothing here defines ED-318, Transport Canada policy, NAV CANADA policy, or an operational authorization.

## Intended readers

The repository is organized for engineers, regulators, researchers, data publishers, software contributors, and RPAS system integrators who need to understand exactly where information came from and which transformations were applied.

## Core principles

1. Preserve official and authoritative source material unchanged.
2. Record provenance and SHA-256 checksums for imported material.
3. Separate standards material from project implementation.
4. Separate exact wire-format models from normalized internal models.
5. Separate national source material from proposed or experimental profiles.
6. Make validation, transformation, and comparison reproducible.
7. State uncertainty and compatibility limits explicitly.

## Architecture

The intended processing boundary is:

```text
Official ED-318 JSON
        ↓
JSON Schema validation
        ↓
Exact ED-318 Rust wire model
        ↓
Normalized internal model
        ↓
Country profiles and geo-awareness logic
```

Canadian assumptions must not be introduced into schema validation or the exact ED-318 wire model. See [`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md).

## Repository map

- `schemas/ed-318/` — unchanged ED-318 JSON Schema files from the imported upstream archive.
- `vendor/ED-318/` — unchanged upstream examples, helper files, README, and licence.
- `datasets/original/` — unmodified authoritative datasets acquired from publishers.
- `datasets/normalized/` — outputs created by future normalization tools.
- `datasets/generated/` — synthetic fixtures, demonstrations, and edge cases.
- `countries/` — country-specific research, classification, and provenance notes.
- `profiles/canada/` — official, proposed, and experimental Canadian profile material kept in separate status directories.
- `docs/` — architecture, standards, Canadian, and country-comparison documents.
- `reports/` — semantic audits and release diff reports.
- `src/` — Rust implementation; currently a placeholder only.
- `tests/` — future automated validation, model, normalization, and CLI tests.

## Imported ED-318 reference

The uploaded `ED-318-main.zip` archive was imported for this release. The archive did not include `.git` metadata, so an exact upstream commit SHA cannot be proven from the supplied artifact. The release therefore records the archive SHA-256, retrieval date, included paths, exclusions, individual file checksums, and upstream URL in [`vendor/ED-318/PROVENANCE.yaml`](vendor/ED-318/PROVENANCE.yaml).

The imported project is MIT licensed. Its licence is retained at [`vendor/ED-318/LICENSE`](vendor/ED-318/LICENSE).

## Build verification

Run:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

At v0.2.1 the test harness is expected to report zero tests. That is intentional for this documentation/provenance release.

## Development sequence

The next planned stages are:

1. JSON Schema validator.
2. Exact ED-318 Rust wire model.
3. Normalized internal model.
4. Country reference baselines.
5. `inspect`, `validate`, `normalize`, `query`, and `compare` CLI commands.
6. Canadian geo-awareness research profile.

See [`ROADMAP.md`](ROADMAP.md) for release gates and acceptance criteria.

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) before changing source material, schemas, provenance records, or country classifications.

## Licence

Project-authored material is licensed under the MIT License. Third-party material remains under its original licence and is identified through provenance records. See [`LICENSE`](LICENSE).
