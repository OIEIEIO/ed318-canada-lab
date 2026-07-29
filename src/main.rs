// File: src/main.rs
// Path: src/main.rs
// Version: 0.1.0
// Purpose: Parse, inspect, validate, and query learning-oriented ED-318-style geo-zone datasets.
// Created: 2026-07-29
// Timestamp: 2026-07-29T12:18:00-04:00

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{fs, path::PathBuf};
use thiserror::Error;

// =============================================================================
// CLI
// =============================================================================

#[derive(Debug, Parser)]
#[command(name = "ed318-canada-lab")]
#[command(version)]
#[command(about = "Learn and experiment with ED-318-style Canadian RPAS geo-zone data")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print dataset and zone summaries.
    Inspect {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Validate dataset structure and basic semantic rules.
    Validate {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Query whether a position lies inside any active geo-zone.
    Query {
        #[arg(value_name = "FILE")]
        file: PathBuf,

        #[arg(long)]
        lat: f64,

        #[arg(long)]
        lon: f64,

        /// Altitude in metres above the reference selected by --alt-ref.
        #[arg(long)]
        alt: f64,

        /// Altitude reference supplied by the caller, such as AGL or AMSL.
        #[arg(long, default_value = "AGL")]
        alt_ref: String,

        /// Evaluation time in RFC 3339 UTC form. Defaults to now.
        #[arg(long)]
        at: Option<DateTime<Utc>>,
    },
}

// =============================================================================
// Error handling
// =============================================================================

#[derive(Debug, Error)]
enum AppError {
    #[error("failed to read {path}: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse JSON in {path}: {source}")]
    ParseJson {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("dataset contains {0} validation error(s)")]
    ValidationFailed(usize),
}

// =============================================================================
// Learning data model
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dataset {
    #[serde(default)]
    metadata: DatasetMetadata,

    #[serde(default)]
    zones: Vec<GeoZone>,

    /// Retain producer-specific or future Canadian properties.
    #[serde(flatten)]
    extensions: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DatasetMetadata {
    #[serde(default)]
    dataset_id: String,

    #[serde(default)]
    standard: String,

    #[serde(default)]
    profile: String,

    #[serde(default)]
    provider: String,

    #[serde(default)]
    issued: Option<DateTime<Utc>>,

    #[serde(default)]
    valid_from: Option<DateTime<Utc>>,

    #[serde(default)]
    valid_to: Option<DateTime<Utc>>,

    #[serde(default)]
    description: String,

    #[serde(flatten)]
    extensions: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeoZone {
    identifier: String,
    country: String,
    name: String,

    #[serde(default)]
    reason: String,

    #[serde(default)]
    message: String,

    geometry: Geometry,

    #[serde(default)]
    vertical_layers: Vec<VerticalLayer>,

    #[serde(default)]
    time_periods: Vec<TimePeriod>,

    #[serde(default)]
    restrictions: Vec<String>,

    #[serde(default)]
    authority: Option<Authority>,

    #[serde(flatten)]
    extensions: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "coordinates")]
enum Geometry {
    Polygon(Vec<Vec<Position>>),
    MultiPolygon(Vec<Vec<Vec<Position>>>),
}

/// Geographic position in GeoJSON order: longitude, latitude.
type Position = [f64; 2];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VerticalLayer {
    lower: VerticalLimit,
    upper: VerticalLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerticalLimit {
    value: Option<f64>,
    reference: String,
    unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimePeriod {
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Authority {
    name: String,

    #[serde(default)]
    service: String,

    #[serde(default)]
    contact_name: String,

    #[serde(default)]
    site_url: String,

    #[serde(default)]
    email: String,

    #[serde(default)]
    phone: String,

    #[serde(flatten)]
    extensions: Map<String, Value>,
}

// =============================================================================
// Validation
// =============================================================================

#[derive(Debug)]
struct ValidationIssue {
    path: String,
    message: String,
}

fn validate_dataset(dataset: &Dataset) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if dataset.metadata.dataset_id.trim().is_empty() {
        issue(&mut issues, "metadata.datasetId", "must not be empty");
    }

    if dataset.metadata.provider.trim().is_empty() {
        issue(&mut issues, "metadata.provider", "must not be empty");
    }

    if let (Some(from), Some(to)) = (dataset.metadata.valid_from, dataset.metadata.valid_to) {
        if to <= from {
            issue(
                &mut issues,
                "metadata.validTo",
                "must be later than metadata.validFrom",
            );
        }
    }

    if dataset.zones.is_empty() {
        issue(&mut issues, "zones", "must contain at least one zone");
    }

    for (zone_index, zone) in dataset.zones.iter().enumerate() {
        let base = format!("zones[{zone_index}]");

        if zone.identifier.trim().is_empty() {
            issue(&mut issues, format!("{base}.identifier"), "must not be empty");
        }

        if zone.country.len() != 2 {
            issue(
                &mut issues,
                format!("{base}.country"),
                "should be a two-letter country code",
            );
        }

        validate_geometry(&zone.geometry, &base, &mut issues);

        if zone.vertical_layers.is_empty() {
            issue(
                &mut issues,
                format!("{base}.verticalLayers"),
                "must contain at least one vertical layer",
            );
        }

        for (layer_index, layer) in zone.vertical_layers.iter().enumerate() {
            let layer_path = format!("{base}.verticalLayers[{layer_index}]");
            validate_vertical_limit(&layer.lower, &format!("{layer_path}.lower"), &mut issues);
            validate_vertical_limit(&layer.upper, &format!("{layer_path}.upper"), &mut issues);

            if layer.lower.reference.eq_ignore_ascii_case(&layer.upper.reference)
                && layer.lower.unit.eq_ignore_ascii_case(&layer.upper.unit)
            {
                if let (Some(lower), Some(upper)) = (layer.lower.value, layer.upper.value) {
                    if upper < lower {
                        issue(
                            &mut issues,
                            layer_path,
                            "upper limit must not be below lower limit",
                        );
                    }
                }
            }
        }

        for (period_index, period) in zone.time_periods.iter().enumerate() {
            if period.end_date_time <= period.start_date_time {
                issue(
                    &mut issues,
                    format!("{base}.timePeriods[{period_index}]"),
                    "endDateTime must be later than startDateTime",
                );
            }
        }
    }

    issues
}

fn validate_vertical_limit(
    limit: &VerticalLimit,
    path: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    if limit.reference.trim().is_empty() {
        issue(issues, format!("{path}.reference"), "must not be empty");
    }

    if limit.unit.trim().is_empty() {
        issue(issues, format!("{path}.unit"), "must not be empty");
    }
}

fn validate_geometry(geometry: &Geometry, base: &str, issues: &mut Vec<ValidationIssue>) {
    match geometry {
        Geometry::Polygon(rings) => validate_polygon(rings, &format!("{base}.geometry"), issues),
        Geometry::MultiPolygon(polygons) => {
            if polygons.is_empty() {
                issue(
                    issues,
                    format!("{base}.geometry.coordinates"),
                    "must contain at least one polygon",
                );
            }
            for (index, polygon) in polygons.iter().enumerate() {
                validate_polygon(
                    polygon,
                    &format!("{base}.geometry.coordinates[{index}]"),
                    issues,
                );
            }
        }
    }
}

fn validate_polygon(rings: &[Vec<Position>], path: &str, issues: &mut Vec<ValidationIssue>) {
    if rings.is_empty() {
        issue(issues, path, "must contain an exterior ring");
        return;
    }

    for (ring_index, ring) in rings.iter().enumerate() {
        let ring_path = format!("{path}.coordinates[{ring_index}]");

        if ring.len() < 4 {
            issue(
                issues,
                &ring_path,
                "must contain at least four positions",
            );
            continue;
        }

        if ring.first() != ring.last() {
            issue(issues, &ring_path, "must be closed");
        }

        for (position_index, [lon, lat]) in ring.iter().copied().enumerate() {
            if !(-180.0..=180.0).contains(&lon) {
                issue(
                    issues,
                    format!("{ring_path}[{position_index}][0]"),
                    "longitude must be between -180 and 180",
                );
            }
            if !(-90.0..=90.0).contains(&lat) {
                issue(
                    issues,
                    format!("{ring_path}[{position_index}][1]"),
                    "latitude must be between -90 and 90",
                );
            }
        }
    }
}

fn issue(
    issues: &mut Vec<ValidationIssue>,
    path: impl Into<String>,
    message: impl Into<String>,
) {
    issues.push(ValidationIssue {
        path: path.into(),
        message: message.into(),
    });
}

// =============================================================================
// Geo-awareness evaluation
// =============================================================================

#[derive(Debug)]
struct QueryMatch<'a> {
    zone: &'a GeoZone,
    horizontal_match: bool,
    vertical_match: bool,
    temporal_match: bool,
}

impl QueryMatch<'_> {
    fn is_match(&self) -> bool {
        self.horizontal_match && self.vertical_match && self.temporal_match
    }
}

fn query_zone<'a>(
    zone: &'a GeoZone,
    lon: f64,
    lat: f64,
    altitude: f64,
    altitude_reference: &str,
    at: DateTime<Utc>,
) -> QueryMatch<'a> {
    QueryMatch {
        zone,
        horizontal_match: geometry_contains(&zone.geometry, lon, lat),
        vertical_match: zone.vertical_layers.iter().any(|layer| {
            layer_contains(layer, altitude, altitude_reference)
        }),
        temporal_match: zone.time_periods.is_empty()
            || zone.time_periods.iter().any(|period| {
                at >= period.start_date_time && at <= period.end_date_time
            }),
    }
}

fn layer_contains(layer: &VerticalLayer, altitude: f64, altitude_reference: &str) -> bool {
    if !layer
        .lower
        .reference
        .eq_ignore_ascii_case(altitude_reference)
        || !layer
            .upper
            .reference
            .eq_ignore_ascii_case(altitude_reference)
    {
        return false;
    }

    if !layer.lower.unit.eq_ignore_ascii_case("M")
        || !layer.upper.unit.eq_ignore_ascii_case("M")
    {
        return false;
    }

    let above_lower = layer.lower.value.is_none_or(|lower| altitude >= lower);
    let below_upper = layer.upper.value.is_none_or(|upper| altitude <= upper);
    above_lower && below_upper
}

fn geometry_contains(geometry: &Geometry, lon: f64, lat: f64) -> bool {
    match geometry {
        Geometry::Polygon(rings) => polygon_contains(rings, lon, lat),
        Geometry::MultiPolygon(polygons) => polygons
            .iter()
            .any(|rings| polygon_contains(rings, lon, lat)),
    }
}

fn polygon_contains(rings: &[Vec<Position>], lon: f64, lat: f64) -> bool {
    let Some(exterior) = rings.first() else {
        return false;
    };

    if !ring_contains(exterior, lon, lat) {
        return false;
    }

    !rings
        .iter()
        .skip(1)
        .any(|hole| ring_contains(hole, lon, lat))
}

fn ring_contains(ring: &[Position], lon: f64, lat: f64) -> bool {
    if ring.len() < 4 {
        return false;
    }

    let mut inside = false;
    let mut previous = ring.len() - 1;

    for current in 0..ring.len() {
        let [current_lon, current_lat] = ring[current];
        let [previous_lon, previous_lat] = ring[previous];

        let crosses_latitude = (current_lat > lat) != (previous_lat > lat);
        let intersection_lon = (previous_lon - current_lon) * (lat - current_lat)
            / (previous_lat - current_lat)
            + current_lon;

        if crosses_latitude && lon < intersection_lon {
            inside = !inside;
        }

        previous = current;
    }

    inside
}

// =============================================================================
// I/O and command execution
// =============================================================================

fn load_dataset(path: &PathBuf) -> Result<Dataset, AppError> {
    let text = fs::read_to_string(path).map_err(|source| AppError::ReadFile {
        path: path.clone(),
        source,
    })?;

    serde_json::from_str(&text).map_err(|source| AppError::ParseJson {
        path: path.clone(),
        source,
    })
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Command::Inspect { file } => {
            let dataset = load_dataset(&file)?;
            println!("Dataset: {}", dataset.metadata.dataset_id);
            println!("Standard: {}", dataset.metadata.standard);
            println!("Profile: {}", dataset.metadata.profile);
            println!("Provider: {}", dataset.metadata.provider);
            println!("Zones: {}", dataset.zones.len());

            for zone in &dataset.zones {
                println!(
                    "- {} | {} | {} | {} vertical layer(s) | {} time period(s)",
                    zone.identifier,
                    zone.country,
                    zone.name,
                    zone.vertical_layers.len(),
                    zone.time_periods.len()
                );
            }
        }
        Command::Validate { file } => {
            let dataset = load_dataset(&file)?;
            let issues = validate_dataset(&dataset);

            if issues.is_empty() {
                println!("VALID: no learning-model validation errors found");
            } else {
                for issue in &issues {
                    eprintln!("ERROR {}: {}", issue.path, issue.message);
                }
                return Err(AppError::ValidationFailed(issues.len()));
            }
        }
        Command::Query {
            file,
            lat,
            lon,
            alt,
            alt_ref,
            at,
        } => {
            let dataset = load_dataset(&file)?;
            let issues = validate_dataset(&dataset);
            if !issues.is_empty() {
                for issue in &issues {
                    eprintln!("ERROR {}: {}", issue.path, issue.message);
                }
                return Err(AppError::ValidationFailed(issues.len()));
            }

            let evaluation_time = at.unwrap_or_else(Utc::now);
            println!(
                "Query: lat={lat}, lon={lon}, altitude={alt} m {alt_ref}, time={evaluation_time}"
            );

            let matches: Vec<_> = dataset
                .zones
                .iter()
                .map(|zone| query_zone(zone, lon, lat, alt, &alt_ref, evaluation_time))
                .filter(QueryMatch::is_match)
                .collect();

            if matches.is_empty() {
                println!("CLEAR: no matching active learning-model geo-zone");
            } else {
                for result in matches {
                    println!(
                        "MATCH: {} | {} | restrictions: {} | message: {}",
                        result.zone.identifier,
                        result.zone.name,
                        result.zone.restrictions.join(", "),
                        result.zone.message
                    );
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Geometry {
        Geometry::Polygon(vec![vec![
            [-82.80, 42.00],
            [-82.70, 42.00],
            [-82.70, 42.10],
            [-82.80, 42.10],
            [-82.80, 42.00],
        ]])
    }

    #[test]
    fn point_inside_polygon_is_detected() {
        assert!(geometry_contains(&square(), -82.75, 42.05));
    }

    #[test]
    fn point_outside_polygon_is_rejected() {
        assert!(!geometry_contains(&square(), -82.90, 42.05));
    }

    #[test]
    fn altitude_layer_requires_matching_reference() {
        let layer = VerticalLayer {
            lower: VerticalLimit {
                value: Some(0.0),
                reference: "AGL".into(),
                unit: "M".into(),
            },
            upper: VerticalLimit {
                value: Some(120.0),
                reference: "AGL".into(),
                unit: "M".into(),
            },
        };

        assert!(layer_contains(&layer, 60.0, "AGL"));
        assert!(!layer_contains(&layer, 60.0, "AMSL"));
        assert!(!layer_contains(&layer, 150.0, "AGL"));
    }
}

// =============================================================================
// End of file: src/main.rs
// =============================================================================
