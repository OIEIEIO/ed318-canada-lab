# National Implementation Comparison

## Scope

This comparison is based on the actual Ireland and Spain source files supplied for the v0.3.0 study. Czech information is limited to the publication inventory supplied by the project owner and is not yet a field-level dataset inspection.

| Aspect | Reference schemas | Ireland | Spain | Czech Republic |
|---|---|---|---|---|
| Evidence in v0.3.0 | Eight imported JSON Schema files | Complete supplied national GeoJSON inspected | Three complete supplied ENAIRE JSON files inspected | Publication inventory only |
| Primary role | Canonical machine-readable comparison | National implementation close to reference property layout | National implementation with two publication layouts | Future study of national publishing ecosystem |
| Top-level form | `FeatureCollection` schema with `type`, `features`, and optional metadata fields | `FeatureCollection` with `name`, `datasetMetadata`, and `features` | Three `FeatureCollection` files with only `type` and `features` | Full ED-318 export, tiled export, and many thematic source products are advertised |
| Feature geometry observed | GeoJSON and LayeredGeoJSON alternatives | `MultiPolygon` | `Polygon` | Not inspected in v0.3.0 |
| Zone properties | Direct feature properties such as `identifier`, `country`, `type`, `variant`, `reason`, and `zoneAuthority` | Direct feature properties closely matching those concepts | Aero/Urbano nest core fields in `UASZone`; Infra flattens about forty fields | Not inspected in v0.3.0 |
| Vertical limits | `geometry.layer` through LayeredGeoJSON reference material | Not present in the inspected feature-property list; geometry is plain `MultiPolygon` | Aero/Urbano use `properties.verticalLayer`; Infra uses flat `lower`, `upper`, references, and `uom` | Not inspected in v0.3.0 |
| Reasons field | `reason` array in the reference property schema | `reason` present | `reasons` string used | Not inspected in v0.3.0 |
| Authorities | `zoneAuthority` array | `zoneAuthority` present | Nested array in Aero/Urbano; authority fields flattened in Infra | Not inspected in v0.3.0 |
| Dataset metadata | Reference collection schema uses fields including provider, issued, validity, description, and technical limitations | Top-level `datasetMetadata` with provider, issued, validFrom, and technicalLimitation | Per-feature `datasetMetadata` in Aero/Urbano; metadata fields flattened in Infra | Publisher inventory includes effective dates, amendments, file sizes, and MD5 checksums |
| Distribution strategy | Repository snapshot | One supplied national file | Three thematic products: Aero, Infra, Urbano | Individual regulatory datasets plus complete and tiled ED-318 products |
| Main lesson | Intended canonical structure | A close real-world implementation is possible | Authorities may preserve concepts while changing serialization layout | Lifecycle, tiling, integrity, and future-effective releases are part of implementation strategy |

## Ireland

The supplied file contains 86 features. Every observed geometry is `MultiPolygon`. Its direct property names closely resemble the reference property schema: `identifier`, `country`, `name`, `type`, `variant`, `restrictionConditions`, `reason`, `otherReasonInfo`, `regulationExemption`, `message`, `region`, and `zoneAuthority`.

This does not by itself prove complete strict schema conformance. The repository currently records structural observations rather than a validator result.

## Spain

The supplied ENAIRE files contain:

- Aero: 1,679 Polygon features;
- Infra: 14,104 Polygon features;
- Urbano: 4 Polygon features.

Aero and Urbano share a nested publication layout containing `UASZone`, `verticalLayer`, `zoneAuthority`, `limitedApplicability`, `dataSource`, and `datasetMetadata`. Infra flattens the equivalent concepts into direct properties. Spain therefore demonstrates that one national publisher may use more than one serialization strategy across thematic products.

## Czech Republic

The supplied publication inventory shows individual LKR products, complete GeoJSON, complete ED-318, tiled ED-318, AIRAC-style effective dates, future-effective releases, amendments, file sizes, and MD5 checksums. No Czech JSON was inspected for v0.3.0, so field-level claims are intentionally deferred.

## Design implication

Future software should not begin by assuming that every national publication is serialized exactly like the reference examples. A later inspector should first describe what is present. Strict validation, tolerant reading, implementation recognition, and conversion should remain separate operations.
