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
        let repo = Self {
            conn: Mutex::new(conn),
        };
        Ok(repo)
    }
}

impl AircraftRepository for SqliteRepository {
    fn insert_aircraft(&self, a: &Aircraft) -> Result<(), DomainError> {
        debug!(icao = %a.hex, "Inserting aircraft");

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO aircraft
                (icao, callsign, aircraft_type, description, owner_operator,
                 registration, category, db_flags, year, first_seen)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, unixepoch())",
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

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO positions (icao, lat, lon, alt_baro, gs, mach, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![a.hex, lat, lon, alt_baro, a.gs, a.mach, timestamp],
        )
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
