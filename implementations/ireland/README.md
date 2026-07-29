# Ireland National Implementation Profile

## Status

Real supplied national GeoJSON inspected for v0.3.0.

## Observed source

- Top-level type: `FeatureCollection`
- Feature count: 86
- Top-level keys: `type`, `name`, `datasetMetadata`, `features`
- Geometry type observed: `MultiPolygon`
- Dataset provider recorded internally: Irish Aviation Authority
- Internal issued and valid-from timestamp: `2026-07-25T13:00:00+00:00`

## Structural relationship to the reference schemas

Ireland uses direct feature properties that closely resemble the reference `Schema_GeoZoneProperties.json` model. Observed fields include:

- `identifier`
- `country`
- `name`
- `type`
- `variant`
- `restrictionConditions`
- `reason`
- `otherReasonInfo`
- `regulationExemption`
- `message`
- `region`
- `zoneAuthority`

The dataset uses plain `MultiPolygon` geometry. This profile does not claim strict schema validation because v0.3.0 contains no validator.

## Why Ireland matters

Ireland provides a real national publication that is structurally close to the reference property model. It is therefore useful for understanding how the reference concepts may appear in an operational national file.

See `PROVENANCE.yaml` and `samples/ireland-representative-sample.geojson`.
