# v0.3.0 Diff Report

## Comparison

- Baseline: `v0.2.1`
- Target: `v0.3.0`
- Modified files: 18
- Added files: 17
- Removed files: 19
- Unchanged non-report files: 25

## Structural change

The country/profile scaffold was replaced by an evidence-first structure:

- `reference/`
- `implementations/ireland/`
- `implementations/spain/`
- `implementations/czech-republic/`
- `canada/`
- cross-country comparison and terminology documents

Large national source files were not committed. Four deterministic derived samples and provenance manifests were added.

## Software change

The package version and placeholder message changed from v0.2.1 to v0.3.0. No parser, validator, normalizer, or other runtime capability was added.

## Imported material

All files under `schemas/ed-318/` and `vendor/ED-318/` remain byte-for-byte unchanged from v0.2.1.

## Removed concepts

The following v0.2.1 framing was retired:

- validator-first roadmap;
- country-specific schema implication;
- premature Canadian profile hierarchy;
- synthetic Canada sample as a primary reference dataset.

See `MODIFIED_FILES.md` for the complete path inventory and `FULL_DIFF.patch` for the textual patch. The patch intentionally excludes itself to avoid recursive content.
