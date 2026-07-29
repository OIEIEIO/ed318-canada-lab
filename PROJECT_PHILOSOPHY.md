# Project Philosophy

## Purpose

`ed318-canada-lab` exists to make geographical-zone data engineering inspectable. It provides a place to compare standards, public datasets, schemas, national publication practices, parser behaviour, normalization choices, and proposed Canadian approaches without presenting project conclusions as regulatory fact.

## Authority boundary

This repository is not EUROCAE, Transport Canada, NAV CANADA, EUROCONTROL, the FAA, or another competent authority. It does not create legal requirements, operational permissions, or authoritative interpretations. Primary publications and official guidance always take precedence.

## Evidence before interpretation

Claims about a country, schema, standard, or authority must identify their source. Where evidence is incomplete, the repository must say so. Compatibility conclusions must distinguish direct conformance evidence from engineering inference.

## Immutable source principle

Official or authoritative source files are stored unchanged. Their checksums are recorded at acquisition. Corrections, mappings, and transformations are written to separate files so the original evidence remains recoverable.

## Layer separation

The project keeps five concerns separate:

1. **Source material** — files as published or supplied.
2. **Schema validation** — whether a document satisfies the selected schema.
3. **Exact wire model** — Rust types representing the official serialized structure.
4. **Normalization** — project-defined structures that permit comparison across formats.
5. **Profiles and interpretation** — country-specific or application-specific meaning.

A rule from a Canadian profile must never silently alter the ED-318 schema or exact wire model.

## National neutrality

Not every country publishes geographical-zone information using ED-318. Country material must be classified honestly as an ED-318 implementation, a compatible geographical-zone publication, a non-ED-318 comparison model, or a proposed/future profile.

## Reproducibility

Every imported dataset or reference snapshot should record origin, retrieval time, version information, licence, checksums, and transformation history. Generated outputs should identify the exact source and tool version that produced them.

## Safety and operational scope

The laboratory may eventually support validation and geo-awareness research, but it is not an aircraft command system, flight authorization service, navigation authority, or replacement for current aeronautical information. Operational integrations require separate safety analysis, requirements validation, verification, and regulatory acceptance.

## Change discipline

Each accepted release identifies its baseline, modified files, intended semantic changes, checks performed, known limitations, and unresolved questions. Imported official reference files are treated as read-only snapshots; upstream updates are introduced as new, separately identified snapshots.
