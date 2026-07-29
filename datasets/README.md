# Datasets

## `original/`

Unmodified datasets obtained from official or authoritative publishers.

Each dataset should include a provenance manifest recording:

- country;
- publishing authority;
- source URL;
- retrieval date;
- dataset version;
- licence;
- SHA-256 checksum;
- format;
- ED-318 compatibility;
- notes.

## `normalized/`

Outputs produced by this project's parser or normalization pipeline.

Normalized files must identify the original source dataset and parser version.

## `generated/`

Synthetic datasets, test fixtures, demonstrations, and generated edge cases.

Generated data must never be described as official national data.
