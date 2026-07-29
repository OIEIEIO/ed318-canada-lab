# Architecture

## 1. Purpose

The architecture protects provenance and meaning while allowing ED-318 data and other national geographical-zone publications to be studied in one repository.

## 2. Layered model

```text
External source archive or dataset
        ↓ acquisition + checksum
Immutable source snapshot
        ↓
JSON Schema validation
        ↓
Exact ED-318 Rust wire model
        ↓ explicit transformation
Normalized internal model
        ↓
Country profile and interpretation
        ↓
Inspection, query, comparison, or application API
```

Each arrow is a boundary that must be testable and auditable.

## 3. Immutable source layer

External schemas, examples, and authoritative datasets are copied without content edits. The source URL, retrieval date, licence, archive checksum, and per-file checksums are recorded. Operating-system metadata may be excluded, but exclusions must be named.

## 4. Validation layer

Validation answers a narrow question: does the JSON instance satisfy the selected schema and configured format checks? It must not infer Canadian requirements, normalize values, or silently repair invalid documents.

Validation diagnostics should include the instance path, schema path when available, error category, and human-readable explanation. Exit codes must distinguish success, validation failure, input/read failure, schema/configuration failure, and internal failure.

## 5. Exact wire-model layer

The exact model represents the official serialized structure. Field names, optionality, enumerations, arrays, nullability, and extensions should remain faithful to the selected ED-318 snapshot. Unknown or extension content must not be discarded merely because the project does not yet interpret it.

## 6. Normalized internal model

Normalization creates project-owned structures for cross-format comparison. It may standardize names or units only through explicit transformations. Every normalized object must retain source identity and transformation provenance. Ambiguous source values remain marked as ambiguous.

## 7. Country-profile layer

Country profiles describe publication practice and interpretation outside the ED-318 wire format. A profile states its authority status, evidence, assumptions, supported source formats, mappings, and unresolved questions.

Canadian material is divided into `official`, `proposed`, and `experimental`. Code must not merge these status classes implicitly.

## 8. Command-line boundary

Planned commands are `inspect`, `validate`, `normalize`, `query`, and `compare`. CLI output is an interface contract and requires tests. Machine-readable output should be available where practical and separated from diagnostics.

## 9. Error and data-loss policy

- Invalid source data is reported, not silently corrected.
- Unsupported values remain visible.
- Unit and vertical-reference conversion requires an explicit policy.
- AGL, AMSL, and WGS84 are not interchangeable.
- Geometry boundary behaviour must be specified and tested.
- Time-zone, recurrence, sunrise, and sunset rules require explicit implementations.

## 10. Security and safety boundary

Input files are untrusted. Future code should bound memory use, recursion, collection sizes, and diagnostic output. The project does not provide flight authorization or command an aircraft. Operational adoption requires independent requirements, safety, cybersecurity, and regulatory work.

## 11. Current implementation status

At v0.2.1 only the repository foundation, documentation, and imported reference snapshot exist. `src/main.rs` prints a placeholder message. No architecture box beyond source acquisition is implemented.
