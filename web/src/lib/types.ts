export interface AircraftSummary {
  icao: string;
  callsign: string | null;
  aircraft_type: string | null;
  registration: string | null;
  first_seen: number;
  last_seen: number;
  position_count: number;
}

export interface TrackPoint {
  lat: number;
  lon: number;
  alt_baro: number | null;
  gs: number | null;
  mach: number | null;
  source_type: string | null;
  mlat_count: number | null;
  ts: number;
  segment: number;
}

export interface Stop {
  lat: number;
  lon: number;
  from_ts: number;
  to_ts: number;
  duration_s: number;
}

export interface MlatStats {
  min: number;
  max: number;
  avg: number;
  samples: number;
}

export interface SourceTypeCount {
  source_type: string;
  count: number;
}

export interface SegmentStats {
  segment: number | null;
  point_count: number;
  source_types: SourceTypeCount[];
  mlat_receivers: MlatStats | null;
  max_gs: number | null;
  max_mach: number | null;
  start_ts: number | null;
  end_ts: number | null;
}

export interface Track {
  points: TrackPoint[];
  stops: Stop[];
  segments: SegmentStats[];
  overall: SegmentStats;
}
