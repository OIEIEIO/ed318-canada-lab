# ED-318 Canada Lab v0.1.0

A small Rust command-line learning tool for exploring RPAS geographical-zone parsing, validation, and geo-awareness queries.

## Important status

This project is **not an ED-318-compliant implementation** and must not be used for flight planning. The complete EUROCAE ED-318 specification and any future Transport Canada/NAV CANADA Canadian application profile are required before conformance can be claimed.

The included dataset is synthetic and deliberately located near Kingsville, Ontario for convenient testing. It is not authoritative airspace information.

## Current learning goals

- Load an ED-318-inspired JSON dataset.
- Preserve unknown fields for future Canadian extensions.
- Validate basic dataset, polygon, altitude, and time rules.
- Inspect dataset contents.
- Query latitude, longitude, altitude, altitude reference, and time.
- Keep parsing separate from legal or flight-control behaviour.

## Build

```bash
cargo build
cargo test
```

## Inspect the sample

```bash
cargo run -- inspect samples/canada-learning-zones.json
```

## Validate the sample

```bash
cargo run -- validate samples/canada-learning-zones.json
```

## Query a point inside the sample zone

```bash
cargo run -- query samples/canada-learning-zones.json \
  --lat 42.0500 \
  --lon -82.7300 \
  --alt 60 \
  --alt-ref AGL \
  --at 2026-08-01T16:00:00Z
```

## Query a point outside the sample zone

```bash
cargo run -- query samples/canada-learning-zones.json \
  --lat 42.0500 \
  --lon -82.9000 \
  --alt 60 \
  --alt-ref AGL \
  --at 2026-08-01T16:00:00Z
```

## Current limitations

- Uses a learning model, not the official ED-318 schema.
- Supports Polygon and MultiPolygon only.
- Supports fixed UTC time intervals only.
- Altitude queries require the caller and zone to use the same reference.
- No terrain model is available for AGL/AMSL conversion.
- No signature, trust-chain, download, update, or anti-tamper logic.
- No Transport Canada visualization profile.
- No ArduPilot or DroneCAN integration.

## Planned progression

1. Obtain and map the complete ED-318 data model and controlled vocabularies.
2. Import real public ED-318 test datasets from European providers.
3. Add a provider adapter so source JSON can be normalized without losing fields.
4. Add recurring schedules and event-relative periods.
5. Add altitude datum conversion with a terrain/elevation source.
6. Add dataset freshness, provenance, hashes, and update handling.
7. Add the Canadian application profile after TC/NAV CANADA publishes it.
8. Add map visualization and route-intersection tests.

## Canadian design rule

The ED-318 core parser, Canadian profile interpretation, geo-awareness evaluator, user interface, and any flight-controller integration should remain separate modules. This prevents a small Canadian profile change from forcing a rewrite of the whole system.
