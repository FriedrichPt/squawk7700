use std::sync::Mutex;

use rusqlite::{Connection, params};
use tracing::debug;

use crate::application::ports::AircraftRepository;
use crate::domain::aircraft::Aircraft;
use crate::domain::error::DomainError;

pub struct SqliteRepository {
    conn: Mutex<Connection>,
}

impl SqliteRepository {
    pub fn new(path: &str) -> Result<Self, DomainError> {
        let conn = Connection::open(path).map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;

             CREATE TABLE IF NOT EXISTS aircraft (
                 icao            TEXT PRIMARY KEY,
                 callsign        TEXT,
                 aircraft_type   TEXT,
                 description     TEXT,
                 owner_operator  TEXT,
                 registration    TEXT,
                 category        TEXT,
                 db_flags        INTEGER,
                 year            TEXT,
                 mode_s_only     INTEGER NOT NULL DEFAULT 0,
                 first_seen      INTEGER NOT NULL
             );

             CREATE TABLE IF NOT EXISTS positions (
                 id          INTEGER PRIMARY KEY AUTOINCREMENT,
                 icao        TEXT NOT NULL REFERENCES aircraft(icao),
                 source_type TEXT,
                 lat         REAL NOT NULL,
                 lon         REAL NOT NULL,
                 alt_baro    REAL,
                 gs          REAL,
                 mach        REAL,
                 mlat_count  INTEGER,
                 timestamp   INTEGER NOT NULL
             );",
        )
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

impl AircraftRepository for SqliteRepository {
    fn insert_aircraft(&self, a: &Aircraft) -> Result<(), DomainError> {
        debug!(icao = %a.hex, "Inserting aircraft");

        let mode_s_only = a.source_type.as_deref() == Some("mode_s");

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO aircraft
                (icao, callsign, aircraft_type, description, owner_operator,
                 registration, category, db_flags, year, mode_s_only, first_seen)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, unixepoch())",
            params![
                a.hex,
                a.flight.as_deref(),
                a.t.as_deref(),
                a.desc.as_deref(),
                a.own_op.as_deref(),
                a.r.as_deref(),
                a.category.as_deref(),
                a.db_flags,
                a.year.as_deref(),
                mode_s_only,
            ],
        )
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    fn insert_position(&self, a: &Aircraft, timestamp: i64) -> Result<(), DomainError> {
        let (lat, lon) = match (a.lat, a.lon) {
            (Some(lat), Some(lon)) => (lat, lon),
            _ => return Ok(()),
        };

        let alt_baro = a.alt_baro.as_ref().and_then(|v| v.as_f64());

        let mlat_count = a.mlat.as_ref().map(|v| v.len() as i64);

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO positions (icao, source_type, lat, lon, alt_baro, gs, mach, mlat_count, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![a.hex, a.source_type, lat, lon, alt_baro, a.gs, a.mach, mlat_count, timestamp],
        )
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
