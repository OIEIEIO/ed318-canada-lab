# Architecture

## v0.3.0 architecture

Version 0.3.0 is an evidence architecture, not a software-processing architecture.

```text
ED-318 standard
        │
        ├── machine-readable reference schemas
        └── synthetic reference examples

Real national publications
        ├── Ireland implementation profile
        ├── Spain implementation profile
        └── Czech publication-strategy study

Comparative analysis
        └── future Canadian implementation questions

Later software design
        ├── inspector
        ├── recognizer
        ├── tolerant reader
        ├── strict validator
        └── converter / normalizer
```

## Separation rules

### Reference layer

Contains the imported schemas and examples. These artifacts are preserved unchanged and are not rewritten to match national publications.

### Source-evidence layer

Contains provenance records for complete national source files. Large source files are not normally committed.

### Derived-sample layer

Contains deterministic small samples. Each sample records its source checksum and feature-selection rule. Samples are project artifacts.

### Implementation-profile layer

Documents observed national structure and publication strategy. Profiles distinguish direct file observations from publisher metadata and inference.

### Canadian-strategy layer

Collects questions and comparative lessons. It must not present proposals as official Canadian policy.

### Future software layer

Software components remain unimplemented. When approved, they should remain separate:

1. inspection;
2. implementation recognition;
3. tolerant parsing;
4. strict schema validation;
5. explicit conversion and normalization.

A tolerant reader must never silently convert a national variant and then report it as strict schema conformance.

## Data preservation

- Reference snapshots remain unchanged.
- Complete source-file checksums are recorded before sampling.
- Derived samples disclose reserialization.
- Unknown fields must be preserved by future tooling unless an explicit conversion contract states otherwise.

## Operational boundary

Nothing in this repository is an operational aeronautical data service or aircraft safety function.
