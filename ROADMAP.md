# Capability Roadmap

The roadmap is staged so each layer can be audited before the next layer depends on it.

## v0.2.1 — Documentation and provenance baseline

**Scope:** professional documentation, immutable ED-318 reference snapshot, provenance manifest, formatted Rust placeholder, semantic audit, and diff report.

**Exit criteria:** repository structure is explained; imported files have checksums and licence records; official and experimental material are visibly separated; Rust quality commands pass; no validator or parser is claimed.

## v0.3.0 — Schema validator

Implement:

```text
ed318-canada-lab validate <dataset>
```

Acceptance criteria:

- load an ED-318 JSON document;
- validate against the selected imported schema snapshot;
- resolve local schema references deterministically;
- report failures with useful JSON paths;
- return documented process exit codes;
- avoid discarding unknown input data before validation;
- test official valid and invalid examples;
- document schema draft and format-checking behaviour.

No Canadian interpretation belongs in this stage.

## v0.4.0 — Exact ED-318 Rust wire model

Create types that represent the imported official JSON structure without normalization or Canadian assumptions. Add round-trip and compatibility tests. Document any schema ambiguity that cannot be represented exactly.

## v0.5.0 — Normalized internal model

Define project-owned structures for cross-country comparison. Every transformation must be explicit, testable, and traceable to its source field. Unsupported or ambiguous values must remain visible rather than being silently coerced.

## v0.6.0 — Country reference baselines

Begin with Spain, Czech Republic, Ireland, United States, and Canada. Classify each source as:

- ED-318 implementation;
- compatible geographical-zone publication;
- non-ED-318 comparison model;
- proposed or future Canadian profile.

## v0.7.0 — CLI inspection and comparison

Provide tested commands:

- `inspect`
- `validate`
- `normalize`
- `query`
- `compare`

Each command must document inputs, outputs, exit codes, error behaviour, and example use.

## v0.8.0 — Canadian geo-awareness research profile

Develop a clearly non-authoritative Canadian research profile based on public evidence. Keep official, proposed, and experimental content separate. Record assumptions and unresolved regulatory questions.

## Later work

Possible later capabilities include spatial indexing, temporal evaluation, altitude-reference handling, visualization, publication comparison, update detection, and integration APIs. These are not commitments and require separate requirements and safety review.
