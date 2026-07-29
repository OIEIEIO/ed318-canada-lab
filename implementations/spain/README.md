# Spain National Implementation Profile

## Status

Three real supplied ENAIRE publications inspected for v0.3.0.

| Product | Features | Geometry | Property layout |
|---|---:|---|---|
| ZGUAS_Aero | 1,679 | Polygon | Nested |
| ZGUAS_Infra | 14,104 | Polygon | Flattened |
| ZGUAS_Urbano | 4 | Polygon | Nested |

## Nested Aero and Urbano layout

Observed feature properties include:

- `UASZone`
- `verticalLayer`
- `zoneAuthority`
- `limitedApplicability`
- `dataSource`
- `datasetMetadata`
- GIS-specific fields such as `SHAPE_Area` or `OBJECTID`

Core zone fields such as `identifier`, `country`, `type`, `variant`, and `reasons` are nested inside `UASZone`. Vertical limits are represented in `properties.verticalLayer`.

## Flattened Infra layout

Infra places zone, authority, schedule, metadata, and vertical-limit concepts directly in approximately forty top-level property keys. Examples include `identifier`, `country`, `type`, `reasons`, `lower`, `upper`, `purpose`, `email`, `startTime`, and `name_authority`.

## Differences observed against the reference schemas

- `reasons` is used instead of reference property `reason`.
- Spain stores reason as a string rather than the reference array form.
- Aero and Urbano move core zone fields under `UASZone`.
- Vertical information is stored in properties rather than `geometry.layer`.
- Infra flattens nested concepts.
- Empty strings commonly represent missing values.
- Extra GIS fields are present.
- Messages may contain HTML-like markup.
- Unit values observed include uppercase `M`.

## Why Spain matters

Spain shows that a real authority can preserve ED-318 concepts while adapting serialization to multiple thematic GIS publication pipelines. A later reader must not confuse concept recognition with strict schema conformance.

See `PROVENANCE.yaml` and the three files under `samples/`.
