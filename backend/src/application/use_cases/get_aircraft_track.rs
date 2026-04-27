use std::sync::Arc;

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::Europe::Berlin;

use std::collections::BTreeMap;

use crate::application::ports::AircraftRepository;
use crate::domain::error::DomainError;
use crate::domain::position::{
    MlatStats, Position, SegmentStats, SourceTypeCount, Stop, Track, TrackPoint,
};

/// Two consecutive samples more than this many seconds apart count as a stop
/// (a landing or the aircraft going dark on the ground).
pub const GAP_THRESHOLD_S: i64 = 600;

/// A gap is only treated as a stop if the last sample before the gap looks like
/// the aircraft was on/near the ground. Above this altitude the gap is treated
/// as a coverage loss instead.
pub const AIRBORNE_ALT_FT: f64 = 3000.0;

/// Same idea for ground speed: cruise/climb speeds rule out an imminent landing.
pub const AIRBORNE_GS_KT: f64 = 200.0;

fn likely_airborne(p: &Position) -> bool {
    p.alt_baro.is_some_and(|a| a >= AIRBORNE_ALT_FT)
        || p.gs.is_some_and(|g| g >= AIRBORNE_GS_KT)
}

pub struct GetAircraftTrack {
    repository: Arc<dyn AircraftRepository>,
}

impl GetAircraftTrack {
    pub fn new(repository: Arc<dyn AircraftRepository>) -> Self {
        Self { repository }
    }

    /// `date` is interpreted as a calendar day in Europe/Berlin local time.
    pub fn execute(&self, icao: &str, date: NaiveDate) -> Result<Track, DomainError> {
        let (from_ts, to_ts) = berlin_day_utc_range(date)?;
        let positions = self.repository.list_positions(icao, from_ts, to_ts)?;
        Ok(segment_track(&positions))
    }
}

fn berlin_day_utc_range(date: NaiveDate) -> Result<(i64, i64), DomainError> {
    let start_local = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| DomainError::DataUnavailable("invalid date".into()))?;
    let end_local = (date + Duration::days(1))
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| DomainError::DataUnavailable("invalid date".into()))?;

    let start_utc = Berlin
        .from_local_datetime(&start_local)
        .earliest()
        .ok_or_else(|| DomainError::DataUnavailable("ambiguous local start".into()))?
        .with_timezone(&Utc);
    let end_utc = Berlin
        .from_local_datetime(&end_local)
        .earliest()
        .ok_or_else(|| DomainError::DataUnavailable("ambiguous local end".into()))?
        .with_timezone(&Utc);

    Ok((start_utc.timestamp(), end_utc.timestamp()))
}

fn segment_track(positions: &[Position]) -> Track {
    let mut points = Vec::with_capacity(positions.len());
    let mut stops = Vec::new();
    let mut segment: u32 = 0;

    for (i, p) in positions.iter().enumerate() {
        if i > 0 {
            let prev = &positions[i - 1];
            let gap = p.timestamp - prev.timestamp;
            if gap > GAP_THRESHOLD_S && !likely_airborne(prev) {
                stops.push(Stop {
                    lat: prev.lat,
                    lon: prev.lon,
                    from_ts: prev.timestamp,
                    to_ts: p.timestamp,
                    duration_s: gap,
                });
                segment += 1;
            }
        }
        points.push(TrackPoint {
            lat: p.lat,
            lon: p.lon,
            alt_baro: p.alt_baro,
            gs: p.gs,
            mach: p.mach,
            source_type: p.source_type.clone(),
            mlat_count: p.mlat_count,
            ts: p.timestamp,
            segment,
        });
    }

    let segments = aggregate_segments(&points);
    let overall = aggregate(None, &points);

    Track {
        points,
        stops,
        segments,
        overall,
    }
}

fn aggregate_segments(points: &[TrackPoint]) -> Vec<SegmentStats> {
    let mut by_seg: BTreeMap<u32, Vec<&TrackPoint>> = BTreeMap::new();
    for p in points {
        by_seg.entry(p.segment).or_default().push(p);
    }
    by_seg
        .into_iter()
        .map(|(seg, pts)| aggregate(Some(seg), &pts.into_iter().cloned().collect::<Vec<_>>()))
        .collect()
}

fn aggregate(segment: Option<u32>, points: &[TrackPoint]) -> SegmentStats {
    let mut source_counts: BTreeMap<String, u32> = BTreeMap::new();
    let mut mlat_min: Option<i64> = None;
    let mut mlat_max: Option<i64> = None;
    let mut mlat_sum: i64 = 0;
    let mut mlat_samples: u32 = 0;
    let mut max_gs: Option<f64> = None;
    let mut max_mach: Option<f64> = None;
    let mut start_ts: Option<i64> = None;
    let mut end_ts: Option<i64> = None;

    for p in points {
        if let Some(st) = &p.source_type {
            *source_counts.entry(st.clone()).or_insert(0) += 1;
        }
        if let Some(c) = p.mlat_count {
            mlat_min = Some(mlat_min.map_or(c, |m| m.min(c)));
            mlat_max = Some(mlat_max.map_or(c, |m| m.max(c)));
            mlat_sum += c;
            mlat_samples += 1;
        }
        if let Some(g) = p.gs {
            max_gs = Some(max_gs.map_or(g, |m| m.max(g)));
        }
        if let Some(m) = p.mach {
            max_mach = Some(max_mach.map_or(m, |x| x.max(m)));
        }
        start_ts = Some(start_ts.map_or(p.ts, |s| s.min(p.ts)));
        end_ts = Some(end_ts.map_or(p.ts, |e| e.max(p.ts)));
    }

    let mut source_types: Vec<SourceTypeCount> = source_counts
        .into_iter()
        .map(|(source_type, count)| SourceTypeCount { source_type, count })
        .collect();
    source_types.sort_by(|a, b| b.count.cmp(&a.count));

    let mlat_receivers = match (mlat_min, mlat_max, mlat_samples) {
        (Some(min), Some(max), n) if n > 0 => Some(MlatStats {
            min,
            max,
            avg: mlat_sum as f64 / n as f64,
            samples: n,
        }),
        _ => None,
    };

    SegmentStats {
        segment,
        point_count: points.len() as u32,
        source_types,
        mlat_receivers,
        max_gs,
        max_mach,
        start_ts,
        end_ts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(ts: i64) -> Position {
        pos_with(ts, Some(10_000.0), Some(420.0))
    }

    fn pos_with(ts: i64, alt_baro: Option<f64>, gs: Option<f64>) -> Position {
        Position {
            lat: 52.5,
            lon: 13.4,
            alt_baro,
            gs,
            mach: None,
            source_type: None,
            mlat_count: None,
            timestamp: ts,
        }
    }

    #[test]
    fn splits_on_long_gap_when_grounded() {
        let positions = vec![
            pos_with(1_000, Some(500.0), Some(20.0)),
            pos_with(1_060, Some(0.0), Some(0.0)),
            pos_with(1_060 + 20 * 60, Some(0.0), Some(0.0)),
            pos_with(1_060 + 20 * 60 + 60, Some(500.0), Some(40.0)),
        ];

        let track = segment_track(&positions);

        assert_eq!(track.points.len(), 4);
        assert_eq!(track.stops.len(), 1);

        assert_eq!(track.points[0].segment, 0);
        assert_eq!(track.points[1].segment, 0);
        assert_eq!(track.points[2].segment, 1);
        assert_eq!(track.points[3].segment, 1);

        let stop = &track.stops[0];
        assert_eq!(stop.from_ts, 1_060);
        assert_eq!(stop.to_ts, 1_060 + 20 * 60);
        assert_eq!(stop.duration_s, 20 * 60);
    }

    #[test]
    fn coverage_gap_during_cruise_does_not_split() {
        // Long gap, but the aircraft was clearly cruising before the gap —
        // this is a receiver/coverage hole, not a landing.
        let positions = vec![
            pos(1_000),
            pos(1_060),
            pos(1_060 + 20 * 60),
            pos(1_060 + 20 * 60 + 60),
        ];

        let track = segment_track(&positions);

        assert_eq!(track.points.len(), 4);
        assert_eq!(track.stops.len(), 0);
        assert!(track.points.iter().all(|p| p.segment == 0));
    }

    #[test]
    fn high_speed_overrides_low_altitude() {
        // No altitude data, but ground speed clearly says airborne.
        let positions = vec![
            pos_with(1_000, None, Some(300.0)),
            pos_with(1_060, None, Some(310.0)),
            pos_with(1_060 + 20 * 60, None, Some(280.0)),
        ];

        let track = segment_track(&positions);
        assert_eq!(track.stops.len(), 0);
        assert!(track.points.iter().all(|p| p.segment == 0));
    }

    #[test]
    fn no_gaps_means_single_segment() {
        let positions = vec![pos(0), pos(60), pos(120), pos(180)];
        let track = segment_track(&positions);
        assert_eq!(track.stops.len(), 0);
        assert!(track.points.iter().all(|p| p.segment == 0));
    }

    #[test]
    fn empty_input_yields_empty_track() {
        let track = segment_track(&[]);
        assert!(track.points.is_empty());
        assert!(track.stops.is_empty());
    }
}
