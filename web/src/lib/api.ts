import type { AircraftSummary, Track } from "./types";

async function getJson<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) {
    const body = await res.text();
    throw new Error(`${res.status} ${res.statusText}: ${body}`);
  }
  return res.json();
}

export function getAircraft(): Promise<AircraftSummary[]> {
  return getJson("/api/aircraft");
}

export function getDays(icao: string): Promise<string[]> {
  return getJson(`/api/aircraft/${encodeURIComponent(icao)}/days`);
}

export function getTrack(icao: string, date: string): Promise<Track> {
  return getJson(
    `/api/aircraft/${encodeURIComponent(icao)}/track?date=${encodeURIComponent(date)}`,
  );
}
