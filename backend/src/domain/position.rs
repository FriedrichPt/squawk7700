use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
    pub alt_baro: Option<f64>,
    pub gs: Option<f64>,
    pub mach: Option<f64>,
    pub source_type: Option<String>,
    pub mlat_count: Option<i64>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AircraftSummary {
    pub icao: String,
    pub callsign: Option<String>,
    pub aircraft_type: Option<String>,
    pub registration: Option<String>,
    pub first_seen: i64,
    pub last_seen: i64,
    pub position_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackPoint {
    pub lat: f64,
    pub lon: f64,
    pub alt_baro: Option<f64>,
    pub gs: Option<f64>,
    pub mach: Option<f64>,
    pub source_type: Option<String>,
    pub mlat_count: Option<i64>,
    pub ts: i64,
    pub segment: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct MlatStats {
    pub min: i64,
    pub max: i64,
    pub avg: f64,
    pub samples: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceTypeCount {
    pub source_type: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SegmentStats {
    /// `None` for the overall aggregate, `Some(i)` for a single segment.
    pub segment: Option<u32>,
    pub point_count: u32,
    pub source_types: Vec<SourceTypeCount>,
    pub mlat_receivers: Option<MlatStats>,
    pub max_gs: Option<f64>,
    pub max_mach: Option<f64>,
    pub start_ts: Option<i64>,
    pub end_ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Stop {
    pub lat: f64,
    pub lon: f64,
    pub from_ts: i64,
    pub to_ts: i64,
    pub duration_s: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub points: Vec<TrackPoint>,
    pub stops: Vec<Stop>,
    pub segments: Vec<SegmentStats>,
    pub overall: SegmentStats,
}
