use serde::{Deserialize, Serialize};

/// Represents a single aircraft as returned by the adsb.lol API.
/// All fields are optional because the API omits keys when data is unavailable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aircraft {
    /// 24-bit ICAO address (hex). May start with '~' for non-ICAO addresses.
    pub hex: String,

    /// Type/source of the best available data (e.g. "adsb_icao", "mlat", "mode_s")
    #[serde(rename = "type")]
    pub source_type: Option<String>,

    /// Flight / callsign
    pub flight: Option<String>,

    /// Aircraft registration (tail number)
    pub r: Option<String>,

    /// ICAO aircraft type code (e.g. "B738")
    pub t: Option<String>,

    /// Aircraft description (e.g. "L2J")
    pub desc: Option<String>,

    /// Squawk code
    pub squawk: Option<String>,

    // ── Position ────────────────────────────────────────────────────────────
    /// Latitude in decimal degrees
    pub lat: Option<f64>,

    /// Longitude in decimal degrees
    pub lon: Option<f64>,

    /// Barometric altitude in feet, or "ground"
    pub alt_baro: Option<serde_json::Value>,

    /// Geometric (GNSS) altitude in feet
    pub alt_geom: Option<i32>,

    /// Navigation Integrity Category
    pub nic: Option<u8>,

    /// Radius of Containment (meters)
    pub rc: Option<u32>,

    /// Seconds since position was last updated
    pub seen_pos: Option<f64>,

    // ── Velocity ────────────────────────────────────────────────────────────
    /// Ground speed in knots
    pub gs: Option<f64>,

    /// Indicated Air Speed in knots
    pub ias: Option<u32>,

    /// True Air Speed in knots
    pub tas: Option<u32>,

    /// Mach number
    pub mach: Option<f64>,

    /// Track over ground in degrees (0–360)
    pub track: Option<f64>,

    /// True heading in degrees
    pub true_heading: Option<f64>,

    /// Barometric vertical rate in ft/min
    pub baro_rate: Option<i32>,

    /// Geometric vertical rate in ft/min
    pub geom_rate: Option<i32>,

    // ── Signal / Receiver ───────────────────────────────────────────────────
    /// RSSI of the most recent message (dBFS)
    pub rssi: Option<f64>,

    /// Number of receivers that contributed to MLAT
    pub mlat: Option<Vec<String>>,

    /// Number of receivers that contributed via TISB
    pub tisb: Option<Vec<String>>,

    /// Total messages received from this aircraft
    pub messages: Option<u64>,

    /// Seconds since any message was last received
    pub seen: Option<f64>,

    // ── Navigation / Autopilot ──────────────────────────────────────────────
    /// Navigation altitude mode ("MCP"/"FMS"/"approach"/"alt hold"/"vnav")
    pub nav_altitude_mode: Option<String>,

    /// MCP/FCU selected altitude in feet
    pub nav_altitude_mcp: Option<i32>,

    /// FMS selected altitude in feet
    pub nav_altitude_fms: Option<i32>,

    /// Selected heading in degrees
    pub nav_heading: Option<f64>,

    /// Navigation modes (autopilot, VNAV, LNAV, TCAS, etc.)
    pub nav_modes: Option<Vec<String>>,

    // ── ADS-B Version & Accuracy ────────────────────────────────────────────
    /// ADS-B version number (0, 1 or 2)
    pub version: Option<u8>,

    /// NIC supplement B
    pub nic_baro: Option<u8>,

    /// Navigation Accuracy Category – position
    pub nac_p: Option<u8>,

    /// Navigation Accuracy Category – velocity
    pub nac_v: Option<u8>,

    /// Source Integrity Level
    pub sil: Option<u8>,

    /// SIL supplement
    pub sil_type: Option<String>,

    /// Geometric Vertical Accuracy
    pub gva: Option<u8>,

    /// System Design Assurance
    pub sda: Option<u8>,

    // ── Database flags (bitfield) ───────────────────────────────────────────
    /// Bitfield: military=1, interesting=2, PIA=4, LADD=8
    #[serde(rename = "dbFlags")]
    pub db_flags: Option<u32>,

    /// Aircraft owner / operator from database
    pub own_op: Option<String>,

    /// Year of manufacture
    pub year: Option<String>,

    // ── Emergency ───────────────────────────────────────────────────────────
    /// Emergency / priority status string
    pub emergency: Option<String>,

    /// Category set by the transponder (e.g. "A3")
    pub category: Option<String>,
}

impl Aircraft {
    /// Returns `true` if this aircraft is flagged as military in the database.
    pub fn is_military(&self) -> bool {
        self.db_flags.map_or(false, |f| f & 1 != 0)
    }

    /// Returns `true` if this aircraft is identifiable as German military
    /// by at least one of: ICAO hex range, callsign prefix, or aircraft type.
    pub fn is_german_military(&self) -> bool {
        self.is_military()
            && (self.has_german_military_icao() || self.has_german_military_callsign())
    }

    fn has_german_military_icao(&self) -> bool {
        let hex = self.hex.to_ascii_uppercase();

        if hex == "3F7ECF" {
            println!("hit");
        }
        hex.starts_with("3C")
            || hex.starts_with("3D")
            || hex.starts_with("3E")
            || hex.starts_with("3F")
    }

    fn has_german_military_callsign(&self) -> bool {
        const PREFIXES: &[&str] = &["GAF", "NAV", "CTM", "ASF"];
        self.flight
            .as_deref()
            .map(|f| {
                PREFIXES
                    .iter()
                    .any(|p| f.trim().to_ascii_uppercase().starts_with(p))
            })
            .unwrap_or(false)
    }

    /// Convenience: returns `true` when a valid lat/lon position is present.
    pub fn has_position(&self) -> bool {
        self.lat.is_some() && self.lon.is_some()
    }

    /*
    pub fn registration_country(&self) -> Option<&'static str> {
        self.r.as_deref().and_then(country_from_registration)
    }
    */
}

/// Top-level response envelope returned by `/v2/` endpoints.
#[derive(Debug, Deserialize)]
pub struct AdsbResponse {
    /// List of aircraft currently tracked.
    #[serde(rename = "ac")]
    pub aircraft: Vec<Aircraft>,
}
