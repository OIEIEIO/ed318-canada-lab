# ed318-canada-lab

`ed318-canada-lab` is an auditable reference and learning laboratory for studying how the ED-318 geographical-zone standard is represented, interpreted, published, and maintained by national aviation authorities.

## Release status

This repository is **v0.3.0 — ED-318 Implementation and Strategy Reference Library**.

The project is no longer framed primarily as a parser or validator project. Version 0.3.0 establishes the evidence and terminology needed before software design begins. It compares the reference schemas and examples with real national publication strategies.

The Rust executable remains a placeholder. This release adds no parser, validator, converter, normalized model, geo-awareness engine, or aircraft integration.

## Project question

The central question is:

> How do different countries implement and publish geographical-zone information around the same ED-318 standard, and what can Canada learn from those strategies?

## Evidence layers

The repository distinguishes four evidence layers:

1. **ED-318 standard** — the published standard itself. The standard text is not redistributed here.
2. **Reference schemas** — the machine-readable JSON Schema files retained under `schemas/ed-318/`.
3. **Reference examples** — synthetic examples retained under `vendor/ED-318/examples/`.
4. **National implementation profiles** — real public datasets and publishing practices studied under `implementations/`.

These layers must not be described as interchangeable.

## Current implementation profiles

- **Ireland** — a real national GeoJSON publication that closely resembles the reference property model.
- **Spain** — three real ENAIRE publications demonstrating both nested and flattened ED-318-shaped layouts.
- **Czech Republic** — a documented future study of a mature AIRAC-based publication ecosystem. No Czech sample is included in this release.
- **Canada** — a future implementation strategy study, not an official national ED-318 publication.

## Repository map

- `reference/` — terminology and navigation for the standard, schemas, and examples.
- `schemas/ed-318/` — unchanged machine-readable reference schemas.
- `vendor/ED-318/` — unchanged imported GitHub material and examples.
- `implementations/ireland/` — profile, provenance, and verified representative sample.
- `implementations/spain/` — profile, provenance, and verified representative samples.
- `implementations/czech-republic/` — publication-system study placeholder.
- `canada/` — questions and design considerations for a future Canadian strategy.
- `docs/IMPLEMENTATION_COMPARISON.md` — cross-country comparison.
- `docs/TERMINOLOGY.md` — controlled project vocabulary.
- `datasets/` — policy for external datasets and derived samples.
- `reports/` — release audit and diff records.

## Dataset policy

Large national datasets are not committed directly. The repository stores:

- small deterministic samples derived from supplied source files;
- SHA-256 checksums of the complete source files;
- publisher and source information where known;
- exact sample-selection rules;
- transformation disclosures;
- download manifests for reacquiring complete datasets.

A derived sample is never represented as an unchanged official publication.

## Safety and authority boundary

This repository is not EUROCAE, an aviation authority, a flight-authorization service, or an operational navigation source. Primary publications and current legal information always take precedence.

## Start here

1. Read [`docs/TERMINOLOGY.md`](docs/TERMINOLOGY.md).
2. Read [`reference/README.md`](reference/README.md).
3. Compare Ireland and Spain in [`docs/IMPLEMENTATION_COMPARISON.md`](docs/IMPLEMENTATION_COMPARISON.md).
4. Review the national profiles under `implementations/`.
5. Review the Canadian questions in [`canada/README.md`](canada/README.md).
