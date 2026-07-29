# Provenance Specification

## Purpose

Provenance records make national implementation studies reproducible without requiring large source datasets to be committed directly.

## Required source fields

Record, where available:

- national implementation profile;
- publisher or authority;
- authoritative source URL;
- retrieval date;
- supplied filename;
- byte size;
- SHA-256 checksum;
- top-level document type;
- feature count;
- internal issue and validity metadata;
- licence or redistribution terms;
- known uncertainty.

A missing value must be represented as unknown or null rather than guessed.

## Required sample fields

Every derived sample records:

- repository path;
- selection method;
- selected zero-based feature indices;
- selected identifiers where available;
- serialization method;
- sample byte size;
- sample SHA-256 checksum;
- explicit derived-sample status.

## Source identity

Filename alone does not establish source identity. Byte size and SHA-256 together identify the supplied snapshot. A later download with a different checksum is a new snapshot even when the filename is unchanged.

## Transformation record

For v0.3.0 samples, the approved transformation is:

1. parse the complete source as JSON;
2. retain listed top-level fields;
3. select listed features by zero-based index;
4. serialize as UTF-8 JSON with two-space indentation and a final newline.

This changes container serialization and produces a project-derived artifact. Feature content is not intentionally normalized or corrected.

## Integrity algorithms

SHA-256 is the repository integrity algorithm. Publisher-provided MD5 values may also be recorded as publication metadata, but do not replace SHA-256 for project snapshots.

## Authority and licence

Provenance proves which file was studied; it does not grant redistribution rights. Licence and terms must be verified separately. Where terms are unknown, commit only the smallest justified derived sample and state the uncertainty.
