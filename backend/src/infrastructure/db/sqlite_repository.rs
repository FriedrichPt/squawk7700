use std::sync::Mutex;

use rusqlite::{Connection, params};
use tracing::debug;

use crate::application::ports::AircraftRepository;
use crate::domain::aircraft::Aircraft;
use crate::domain::error::DomainError;
use crate::domain::position::{AircraftSummary, Position};

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
             );

             CREATE INDEX IF NOT EXISTS idx_positions_icao_ts
                 ON positions(icao, timestamp);",
        )
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

fn db_err(e: impl std::fmt::Display) -> DomainError {
    DomainError::DatabaseError(e.to_string())
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
        .map_err(db_err)?;

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
        .map_err(db_err)?;

        Ok(())
    }

    fn list_aircraft_summaries(&self) -> Result<Vec<AircraftSummary>, DomainError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT a.icao,
                        a.callsign,
                        a.aircraft_type,
                        a.registration,
                        a.first_seen,
                        MAX(p.timestamp) AS last_seen,
                        COUNT(p.id)      AS position_count
                 FROM aircraft a
                 INNER JOIN positions p ON p.icao = a.icao
                 GROUP BY a.icao
                 ORDER BY position_count DESC, a.icao ASC",
            )
            .map_err(db_err)?;

        let rows = stmt
            .query_map([], |row| {
                let trim = |s: Option<String>| s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
                Ok(AircraftSummary {
                    icao: row.get(0)?,
                    callsign: trim(row.get(1)?),
                    aircraft_type: trim(row.get(2)?),
                    registration: trim(row.get(3)?),
                    first_seen: row.get(4)?,
                    last_seen: row.get(5)?,
                    position_count: row.get(6)?,
                })
            })
            .map_err(db_err)?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(db_err)?);
        }
        Ok(out)
    }

    fn list_positions(
        &self,
        icao: &str,
        from_ts: i64,
        to_ts: i64,
    ) -> Result<Vec<Position>, DomainError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT lat, lon, alt_baro, gs, mach, source_type, mlat_count, timestamp
                 FROM positions
                 WHERE icao = ?1 AND timestamp >= ?2 AND timestamp < ?3
                 ORDER BY timestamp ASC",
            )
            .map_err(db_err)?;

        let rows = stmt
            .query_map(params![icao, from_ts, to_ts], |row| {
                Ok(Position {
                    lat: row.get(0)?,
                    lon: row.get(1)?,
                    alt_baro: row.get(2)?,
                    gs: row.get(3)?,
                    mach: row.get(4)?,
                    source_type: row.get(5)?,
                    mlat_count: row.get(6)?,
                    timestamp: row.get(7)?,
                })
            })
            .map_err(db_err)?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(db_err)?);
        }
        Ok(out)
    }

    fn list_active_days(
        &self,
        icao: &str,
        tz_offset_seconds: i32,
    ) -> Result<Vec<String>, DomainError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT DISTINCT strftime('%Y-%m-%d', timestamp + ?2, 'unixepoch') AS d
                 FROM positions
                 WHERE icao = ?1
                 ORDER BY d DESC",
            )
            .map_err(db_err)?;

        let rows = stmt
            .query_map(params![icao, tz_offset_seconds], |row| row.get::<_, String>(0))
            .map_err(db_err)?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(db_err)?);
        }
        Ok(out)
    }
}
