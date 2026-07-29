# Dataset Provenance Specification

## Purpose

Every external or generated data artifact must be traceable to its origin and transformation history.

## Required fields

A provenance record should include:

- `record_version` — version of this provenance format;
- `artifact_id` — stable project identifier;
- `artifact_type` — schema, example, dataset, licence, source archive, or generated output;
- `country` when applicable;
- `publisher` or `authority`;
- `source_url`;
- `retrieved_at` in ISO 8601 UTC;
- `source_version` and `commit_sha` when available;
- `source_archive` and its SHA-256 when acquisition used an archive;
- `license` and retained licence path;
- `sha256` for each stored artifact;
- `official_source` as true, false, or unknown;
- `modified` as true or false;
- `transformations` for generated or normalized output;
- `notes` and unresolved uncertainty.

## Integrity rules

1. Calculate checksums before interpretation or transformation.
2. Do not replace original files with normalized output.
3. Do not call a file unchanged if line endings, encoding, ordering, or content changed.
4. Name excluded archive entries such as `.DS_Store` files.
5. Mark unavailable immutable identifiers as unavailable; never infer a commit SHA from a branch-named ZIP.

## Storage rules

- Original authoritative data: `datasets/original/<country>/<source>/`.
- Normalized output: `datasets/normalized/<country>/<profile>/`.
- Synthetic fixtures: `datasets/generated/<purpose>/`.
- Imported standards/reference repositories: `vendor/<project>/` and, where needed by tools, unchanged schema copies under `schemas/`.

## Transformation records

A normalized or generated artifact must record the source checksum, tool name and version, command/options, generation time, transformation profile, validation result, and output checksum.

## Verification

Release audits should verify that stored checksums match files and that imported source material has not changed unexpectedly.
