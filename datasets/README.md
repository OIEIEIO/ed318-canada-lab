# Datasets

This tree separates evidence from project output. See [`PROVENANCE_SPEC.md`](PROVENANCE_SPEC.md) for required records.

## `original/`

Unmodified datasets obtained from official or authoritative publishers. Store provenance and checksums beside each acquisition. Do not manually repair source files.

## `normalized/`

Outputs produced by the future normalization pipeline. Each output must identify its source checksum, parser/normalizer version, transformation profile, generation time, and validation result.

## `generated/`

Synthetic datasets, test fixtures, demonstrations, and generated edge cases. Generated data must never be represented as official national data.

## Reference repositories

Standards-related schemas and examples are kept under `schemas/` and `vendor/`, not mixed with national datasets.
