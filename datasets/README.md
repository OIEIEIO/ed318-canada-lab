# Dataset and Sample Policy

## Large-file policy

Complete national datasets are generally not committed because they may be large, frequently updated, or subject to redistribution conditions.

## What is stored

Each studied source should have:

- publisher or attributed authority;
- source filename;
- complete-file byte size;
- complete-file SHA-256 checksum;
- retrieval date when known;
- source URL when known;
- dataset metadata observed inside the file;
- licence or redistribution status when known;
- deterministic sample-selection method;
- list of selected feature identifiers;
- disclosure of all transformations.

## Derived samples

Samples in `implementations/*/samples/` are project-generated subsets. They preserve selected feature objects semantically, but the containing document is newly serialized and therefore has its own checksum.

A sample must not be described as the original national publication.

## Reproduction

To reproduce a sample:

1. acquire the exact source file identified by filename, size, and SHA-256;
2. parse the top-level `FeatureCollection` as JSON;
3. select the recorded zero-based feature indices;
4. retain the recorded top-level metadata fields;
5. serialize as UTF-8, two-space-indented JSON with a final newline.
