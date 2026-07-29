# ED-318 Reference Layer

This directory explains the reference layer used by the laboratory.

## Components

### ED-318 standard

The standard is the normative publication. Its text is not redistributed by this repository.

### Machine-readable reference schemas

The imported JSON Schema files are retained unchanged under `../schemas/ed-318/`. They describe a canonical machine-readable structure used for comparison.

The repository avoids claiming more authority for the GitHub schema snapshot than its documented provenance establishes.

### Reference examples

The files under `../vendor/ED-318/examples/` are synthetic examples supplied with the imported repository. They are useful for understanding intended structures and validation cases, but they are not national operational datasets.

## How the reference is used

The schemas and examples answer:

- What structure does the reference material describe?
- Which properties and enumerations are expected?
- How does LayeredGeoJSON represent vertical information?

National implementation profiles answer:

- What did an authority actually publish?
- How was it organized and distributed?
- Which reference concepts were preserved or adapted?
