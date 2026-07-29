# Semantic Audit Report

## Release identity

- Project: `ed318-canada-lab`
- Baseline: `v0.2.0-foundation`
- Audited release: `v0.2.1`
- Audit date: 2026-07-29
- Release purpose: documentation and provenance baseline

## Audit scope

Every file in the uploaded `ed318-canada-lab-main.zip` baseline and every file in the uploaded `ED-318-main.zip` reference archive was inventoried and read. JSON files were parsed, imported files were compared byte-for-byte with their archive sources, and a complete textual diff was generated.

## Baseline files reviewed

- `.gitignore`
- `CHANGELOG.md`
- `CONTRIBUTING.md`
- `Cargo.lock`
- `Cargo.toml`
- `LICENSE`
- `PROJECT_PHILOSOPHY.md`
- `README.md`
- `ROADMAP.md`
- `countries/PROVENANCE_TEMPLATE.yaml`
- `countries/README.md`
- `countries/canada/README.md`
- `countries/czech-republic/README.md`
- `countries/ireland/README.md`
- `countries/spain/README.md`
- `countries/united-states/README.md`
- `datasets/README.md`
- `datasets/generated/README.md`
- `datasets/normalized/README.md`
- `datasets/original/README.md`
- `docs/architecture/README.md`
- `docs/canada/README.md`
- `docs/countries/README.md`
- `docs/standards/README.md`
- `profiles/canada/README.md`
- `profiles/canada/experimental/README.md`
- `profiles/canada/official/README.md`
- `profiles/canada/proposed/README.md`
- `reports/DIFF_REPORT.md`
- `reports/README.md`
- `reports/SEMANTIC_AUDIT.md`
- `samples/README.md`
- `samples/canada-learning-zones.json`
- `schemas/README.md`
- `src/main.rs`
- `tests/README.md`
- `tools/README.md`
- `vendor/README.md`

## Upstream archive files reviewed

- `.DS_Store`
- `LICENSE`
- `README.md`
- `examples/.DS_Store`
- `examples/Dockerfile`
- `examples/Example_Collection.json`
- `examples/Example_GeoZone_2_Layers.json`
- `examples/Example_GeoZone_Circle.json`
- `examples/Example_GeoZone_with_extension.json`
- `examples/InvalidExample_GeoZone_2_Layers.json`
- `examples/PartialExample_GeoZoneProperties.json`
- `examples/PartialExample_TimePeriod.json`
- `examples/PartialExample_ZoneAuthority.json`
- `examples/PartialExample_featureGeoJSON.json`
- `examples/requirements.txt`
- `examples/validate_examples.py`
- `schema/Schema_GeoJSONGeometries.json`
- `schema/Schema_GeoZoneAuthority.json`
- `schema/Schema_GeoZoneCollectionMetadata.json`
- `schema/Schema_GeoZoneDataTypes.json`
- `schema/Schema_GeoZoneProperties.json`
- `schema/Schema_GeoZoneTimePeriod.json`
- `schema/Schema_GeoZones.json`
- `schema/Schema_LayeredGeoJSON.json`

The `.DS_Store` entries were identified as operating-system metadata and excluded from the imported release. Their exclusion is recorded in `vendor/ED-318/PROVENANCE.yaml`.

## Intended semantic changes

1. Advance package and release identity from `0.2.0-foundation` to `0.2.1`.
2. Replace placeholder documentation with a professional description of purpose, authority boundaries, architecture, provenance, contribution practice, roadmap, country classification, and release controls.
3. Adopt the MIT License for project-authored material while retaining third-party licences and provenance.
4. Import the supplied ED-318 public schemas, examples, validation helper, Dockerfile, requirements, README, and licence without content changes.
5. Record the source archive SHA-256 and per-file SHA-256 checksums.
6. Format the placeholder `src/main.rs` without changing its output or adding implementation.
7. Correct obsolete v0.1.0 audit and diff reports so they describe the accepted v0.2.0 baseline and v0.2.1 release.

## Explicit non-changes

- No validator was implemented.
- No parser or exact ED-318 Rust model was implemented.
- No normalization model was implemented.
- No geometry, altitude, time, query, compare, or visualization engine was implemented.
- No Canadian regulatory interpretation was encoded.
- No Rust dependencies were added.
- The executable still prints `ed318-canada-lab foundation` and exits normally.
- The synthetic `samples/canada-learning-zones.json` fixture was not modified.

## Imported-reference integrity

- Source artifact: `ED-318-main.zip`
- Commit SHA: unavailable because the supplied ZIP contains no `.git` metadata
- Verification: all imported files match the supplied archive byte-for-byte
- Schema files imported: 8
- Example/helper files imported: 12
- Retained top-level upstream files: README and MIT licence
- Excluded metadata: two `.DS_Store` files

The provenance manifest is `vendor/ED-318/PROVENANCE.yaml`.

## Static checks performed

- Complete baseline and upstream file inventory: **passed**
- Full text review: **completed**
- JSON parsing for 8 schemas, 9 upstream JSON examples/partials, and the retained synthetic sample: **passed**
- Byte-for-byte comparison of imported reference files: **passed**
- Full baseline-to-release diff generation: **passed**
- Rust dependency review: **passed; no dependencies declared**

## Rust execution checks

The release environment did not contain the `cargo` executable. Therefore these checks were **not run in this environment**:

```text
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

`src/main.rs`, `Cargo.toml`, and `Cargo.lock` were reviewed textually. The source is formatted in standard Rust style, but local execution remains required before accepting the release baseline.

## Documentation assessment

The repository now states:

- what the project is and is not;
- who the intended readers are;
- how official sources remain immutable;
- how schema validation, exact models, normalization, and country profiles remain separated;
- how Canadian official, proposed, and experimental material is classified;
- what provenance fields are required;
- what each planned version must deliver before the next layer begins.

## Known limitations and decisions requiring owner awareness

1. The ED-318 source commit SHA cannot be proven from the supplied branch ZIP. The archive checksum is the immutable acquisition identifier for this release.
2. The repository adopts MIT for project-authored material in v0.2.1. Third-party material remains under its retained licence.
3. Country classifications are preliminary placeholders pending primary-source acquisition and review.
4. The imported public schema repository is not equivalent to possessing or proving conformance with the complete EUROCAE standard.
5. No CI workflow was added in this release.

## Audit conclusion

`v0.2.1` meets the intended scope of a documentation and provenance baseline. It is suitable to become the next accepted baseline after the owner runs the four Rust verification commands locally and confirms the licensing decision. Software implementation should begin only in the following version.
