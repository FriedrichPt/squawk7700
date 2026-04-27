# Squawk7700 – German Military Aircraft Tracker

## Projektziel

Erfassung und Analyse der aktiven deutschen Militärluftflotte anhand
öffentlicher ADS-B Transponderdaten. Kernfragen:

- Wie viele deutsche Militärflugzeuge sind aktiv im Dienst?
- Wo sind diese stationiert (Heimatbasis)?
- Wo waren diese im Laufe der Zeit?

## Kernlogik: Heimatbasis-Rekonstruktion

Militärflugzeuge senden nur ein Signal wenn sie in der Luft sind.
Daraus folgt:

- Letzter bekannter Landepunkt = wahrscheinliche Heimatbasis
- Jeder ICAO-Code der je gesichtet wurde = ein real existierendes Flugzeug
- Über viele Flüge akkumuliert entsteht ein Bild der Gesamtflotte

Tracking-Logik:

1. Flugzeug taucht auf (takeoff) → Flug beginnen
2. Flugzeug verschwindet (signal lost) → letzter Punkt = Landung
3. Landepunkt + Timestamp in DB speichern

## Datenbankschema (Zielstruktur)

Drei Kerntabellen:

aircraft → ein Eintrag pro unique ICAO Code

- icao (PK)
- callsign
- aircraft_type
- first_seen
- last_seen

flights → ein Eintrag pro Flug

- id (PK)
- icao (FK → aircraft)
- takeoff_time
- landing_time
- takeoff_lat / takeoff_lon
- landing_lat / landing_lon

positions → historische Positionspunkte

- id (PK)
- icao (FK → aircraft)
- flight_id (FK → flights)
- lat / lon / altitude
- timestamp

## Militär-Erkennung (Deutschland)

Drei Erkennungsmerkmale kombinieren:

1. ICAO Hex-Range (Luftwaffe: 3C-Prefix)
2. Callsign-Muster:
   - GAF = German Air Force
   - NAV = Deutsche Marine
   - CTM, ASF = weitere
3. Aircraft Type (Eurofighter, A400M, Tornado, CH-53)

Nur Flugzeuge die mindestens ein Kriterium erfüllen werden gespeichert.

## Repo-Struktur

- backend/ → Rust-Service (Cargo, Dockerfile)
- web/ → SvelteKit-Frontend (Vite-Dev auf :5173)
- compose.yml → orchestriert das Backend (Build-Context backend/)
- daily.db → DB-Snapshot für die Visualisierung (lokal, an der Wurzel)

## Architektur

Hexagonale Architektur (Ports & Adapters), alles in `backend/src/`:

- domain/ → Kernlogik, keine externen Dependencies
- application/ → Use Cases + Ports (Traits)
- infrastructure/ → konkrete Implementierungen
  - http/ → Outbound (reqwest → adsb.lol)
  - db/ → SQLite-Repository
  - web/ → Inbound HTTP (axum) für die Visualisierungs-API
  - config/ → Konfigurations-Defaults

Neue Features immer von domain/ nach außen denken, nie andersrum.

## API (Read-Only)

Inbound-Adapter unter `infrastructure/web/`. Default `127.0.0.1:8080`,
überschreibbar via `SQUAWK_API_ADDR`. DB-Pfad via `SQUAWK_DB_PATH`,
Default `../daily.db` (relativ zum Backend-Cwd).

- GET /api/health
- GET /api/aircraft → AircraftSummary[]
- GET /api/aircraft/:icao/days → ISO-Dates (Europe/Berlin)
- GET /api/aircraft/:icao/track?date=YYYY-MM-DD → Track {points, stops}

Track-Segmentierung: Gaps > 600s zwischen zwei Positionen werden als Stop
registriert und starten ein neues Segment (im Use Case, nicht in der DB).

## Frontend

SvelteKit + Leaflet unter `web/`. Dev:

```
cd web && npm run dev
```

Vite proxied `/api/*` an `http://127.0.0.1:8080` (überschreibbar via
`VITE_API_PROXY`). Leaflet wird dynamisch im `onMount` geladen, daher
keine SSR-Sonderbehandlung nötig.

## Geplante Ausbaustufen

- [x] Militär-Filter (ICAO + Callsign + Typ)
- [x] SQLite Datenbankanbindung (infrastructure/db/)
- [x] Kontinuierlicher Polling Service
- [x] Read-API + erste Visualisierung (Tagesroute pro ICAO)
- [ ] Flug-Tracking (takeoff/landing detection in DB persistiert)
- [ ] Flottenübersicht (Karte aller bekannten Heimatbasen)
- [ ] Kontext-Agent (Flugmuster + aktuelle Nachrichten)
- [ ] Hetzner Server Deployment

## Coding Regeln

- tracing statt println! für alle Debug-Outputs
- Fehler immer als DomainError wrappen
- Kein unwrap() in produktivem Code
- API-Keys nur über Umgebungsvariablen, niemals hardcoded
- Domain-Typen für API-Antworten dürfen serde::Serialize haben, aber keine
  axum/http-Abhängigkeit
- Vor größeren Änderungen Planning Mode nutzen (Shift+Tab x2)
- Nach jedem abgeschlossenen Feature committen
