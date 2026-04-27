use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, Offset, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::use_cases::GetAircraftTrack;
use crate::domain::error::DomainError;
use crate::domain::position::{AircraftSummary, Track};

use super::error::ApiError;
use super::server::AppState;

pub async fn health() -> Json<Value> {
    Json(json!({ "ok": true }))
}

pub async fn list_aircraft(
    State(state): State<AppState>,
) -> Result<Json<Vec<AircraftSummary>>, ApiError> {
    let summaries = state.repo.list_aircraft_summaries()?;
    Ok(Json(summaries))
}

pub async fn list_days(
    State(state): State<AppState>,
    Path(icao): Path<String>,
) -> Result<Json<Vec<String>>, ApiError> {
    let offset = current_berlin_offset_seconds();
    let days = state.repo.list_active_days(&icao, offset)?;
    Ok(Json(days))
}

#[derive(Deserialize)]
pub struct TrackQuery {
    date: String,
}

pub async fn get_track(
    State(state): State<AppState>,
    Path(icao): Path<String>,
    Query(q): Query<TrackQuery>,
) -> Result<Json<Track>, ApiError> {
    let date = NaiveDate::parse_from_str(&q.date, "%Y-%m-%d").map_err(|e| {
        DomainError::DataUnavailable(format!("invalid date '{}': {e}", q.date))
    })?;

    let use_case = GetAircraftTrack::new(state.repo.clone());
    let track = use_case.execute(&icao, date)?;
    Ok(Json(track))
}

fn current_berlin_offset_seconds() -> i32 {
    let now_utc = Utc::now().naive_utc();
    Berlin.from_utc_datetime(&now_utc).offset().fix().local_minus_utc()
}
