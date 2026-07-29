# Semantic Audit Report

## Project

- Name: `ed318-canada-lab`
- Version: `0.1.0`
- Created: 2026-07-29
- Baseline: New project; no earlier code baseline exists.

## Reviewed files

- `Cargo.toml`
- `src/main.rs`
- `samples/canada-learning-zones.json`
- `README.md`

## Intended behaviour

The tool reads a local JSON dataset, deserializes it into a learning-oriented geo-zone model, validates basic semantic constraints, prints summaries, and evaluates whether a supplied position is horizontally, vertically, and temporally inside a zone.

## Safety and scope controls

- The README and sample explicitly state that the project is not compliant or authoritative.
- Unknown dataset, metadata, zone, and authority fields are preserved using flattened JSON maps.
- Parsing does not command an aircraft or modify an autopilot fence.
- Altitude references are not silently converted.
- AGL and AMSL are treated as different references.
- Invalid datasets stop the query operation.

## Geometry review

- Polygon and MultiPolygon are supported.
- Exterior-ring containment uses ray casting.
- Interior rings are treated as holes.
- Validation checks ring size, closure, latitude range, and longitude range.
- Boundary-point behaviour is not yet formally defined and must be addressed against ED-318 requirements.

## Vertical review

- Each zone may contain multiple vertical layers.
- A query matches if any vertical layer contains the altitude.
- Unit handling is currently limited to metres (`M`).
- Unlimited limits are represented with `null` values.
- No AGL/AMSL conversion is attempted.

## Temporal review

- Fixed RFC 3339 UTC start/end periods are supported.
- A zone with no time periods is treated as continuously active.
- Recurring daily periods, weekdays, sunrise, and sunset are not implemented.

## Known implementation risk

This environment did not contain `rustc` or `cargo`, so compilation and test execution could not be performed here. The code was reviewed textually, but a successful local `cargo test` is still required before treating v0.1.0 as a working baseline.

## Compliance conclusion

This version is suitable only as a learning scaffold. It does not claim ED-318, ED-318A, Transport Canada, NAV CANADA, Standard 922, or operational geo-awareness compliance.
