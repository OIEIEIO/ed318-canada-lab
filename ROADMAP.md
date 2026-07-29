# Roadmap

## v0.3.0 — Implementation and Strategy Reference Library

Status: current release.

Goals:

- establish controlled terminology;
- distinguish the standard, schemas, examples, and national implementations;
- document Ireland and Spain from real supplied datasets;
- store small verified derived samples and provenance manifests;
- document the Czech publication ecosystem as a later data study;
- reposition Canada as a future national implementation-strategy question;
- postpone parser and validator design.

Explicitly out of scope:

- Rust ED-318 parser;
- JSON Schema validator;
- national-profile auto-detection;
- canonical normalized model;
- dataset conversion;
- map viewer;
- geo-awareness query engine;
- operational aircraft use.

## v0.3.x — Reference-library refinement

Possible work:

- add an inspected Czech ED-318 sample;
- verify licences and redistribution terms for each national source;
- expand field-level comparison tables;
- add additional countries only when authoritative evidence is available;
- improve reproducible sample-generation documentation.

## v0.4.0 — Dataset inspector design

Only after the reference library is stable:

- define a read-only dataset inspection model;
- summarize top-level structure, feature counts, geometry types, and property layouts;
- preserve unknown fields;
- avoid claiming strict conformance.

## Later software stages

Potential sequence:

1. dataset inspector;
2. implementation recognizer;
3. tolerant reader;
4. strict reference-schema validator;
5. explicit converter or normalizer;
6. Canadian profile experiments;
7. geo-awareness research.

Each stage requires a separate reviewed design decision.
