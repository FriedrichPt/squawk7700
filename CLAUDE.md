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

## Architektur

Hexagonale Architektur (Ports & Adapters):

- domain/ → Kernlogik, keine externen Dependencies
- application/ → Use Cases + Ports (Traits)
- infrastructure/ → HTTP, DB, Config (konkrete Implementierungen)

Neue Features immer von domain/ nach außen denken, nie andersrum.

## Geplante Ausbaustufen

- [ ] Militär-Filter (ICAO + Callsign + Typ)
- [ ] SQLite Datenbankanbindung (infrastructure/db/)
- [ ] Flug-Tracking (takeoff/landing detection)
- [ ] Kontinuierlicher Polling Service
- [ ] Visualisierung (Karte, Flottenübersicht)
- [ ] Kontext-Agent (Flugmuster + aktuelle Nachrichten)
- [ ] Hetzner Server Deployment

## Coding Regeln

- tracing statt println! für alle Debug-Outputs
- Fehler immer als DomainError wrappen
- Kein unwrap() in produktivem Code
- API-Keys nur über Umgebungsvariablen, niemals hardcoded
- Vor größeren Änderungen Planning Mode nutzen (Shift+Tab x2)
- Nach jedem abgeschlossenen Feature committen
