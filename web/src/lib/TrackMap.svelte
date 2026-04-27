<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import type { Track } from "./types";
  import { GERMAN_AIRPORTS } from "./airports";

  let {
    track,
    selectedSegment = null,
    showAirports = false,
    onSegmentClick,
  }: {
    track: Track | null;
    selectedSegment?: number | null;
    showAirports?: boolean;
    onSegmentClick?: (segment: number | null) => void;
  } = $props();

  let mapEl: HTMLDivElement;
  let map: any = null;
  let layer: any = null;
  let airportLayer: any = null;
  let L: any = null;

  let mapReady = $state(false);

  const SEGMENT_COLORS = [
    "#3b82f6",
    "#ef4444",
    "#10b981",
    "#f59e0b",
    "#a855f7",
    "#ec4899",
    "#14b8a6",
    "#f97316",
  ];

  onMount(() => {
    void (async () => {
      L = (await import("leaflet")).default;

      map = L.map(mapEl, { worldCopyJump: true }).setView([51, 10], 5);

      // Dedicated pane so airport markers always render above track layers,
      // regardless of when each layer is (re-)added.
      map.createPane("airports");
      map.getPane("airports").style.zIndex = "650";

      L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
        maxZoom: 19,
        attribution:
          '&copy; <a href="https://openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      }).addTo(map);

      L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
        maxZoom: 19,
        attribution: "© OpenStreetMap contributors",
      }).addTo(map);

      // 👉 HIER EINFÜGEN

      mapReady = true;
      if (track) renderTrack(track);
      renderAirports();
    })();
  });

  onDestroy(() => {
    map?.remove();
    map = null;
  });

  let lastTrackId: string | null = null;

  $effect(() => {
    if (!mapReady || !track) return;
    // re-render whenever selection changes too (highlight)
    selectedSegment;
    renderTrack(track);
  });

  $effect(() => {
    if (!mapReady) return;
    showAirports;
    renderAirports();
  });

  function renderAirports() {
    if (!L || !map) return;
    if (airportLayer) {
      airportLayer.remove();
      airportLayer = null;
    }
    if (!showAirports) return;

    const group = L.layerGroup();
    for (const a of GERMAN_AIRPORTS) {
      L.circleMarker([a.lat, a.lon], {
        pane: "airports",
        radius: 5,
        color: "#0f172a",
        weight: 2,
        fillColor: "#ffffff",
        fillOpacity: 1,
      })
        .bindTooltip(`${a.icao} · ${a.name}`, { direction: "top" })
        .addTo(group);
    }
    group.addTo(map);
    airportLayer = group;
  }

  function emitSegment(seg: number | null) {
    onSegmentClick?.(seg);
  }

  function fmtTime(ts: number): string {
    return new Date(ts * 1000).toLocaleString("de-DE", {
      timeZone: "Europe/Berlin",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function fmtDuration(s: number): string {
    if (s < 60) return `${s}s`;
    const m = Math.round(s / 60);
    if (m < 60) return `${m} min`;
    const h = Math.floor(m / 60);
    const r = m % 60;
    return `${h}h ${r}m`;
  }

  function renderTrack(t: Track) {
    if (!L || !map) return;

    if (layer) {
      layer.remove();
      layer = null;
    }

    if (t.points.length === 0) {
      return;
    }

    const group = L.featureGroup();

    // Polylines per segment
    const segments = new Map<number, [number, number][]>();
    for (const p of t.points) {
      if (!segments.has(p.segment)) segments.set(p.segment, []);
      segments.get(p.segment)!.push([p.lat, p.lon]);
    }

    for (const [seg, coords] of segments) {
      const color = SEGMENT_COLORS[seg % SEGMENT_COLORS.length];
      const isSelected = selectedSegment === seg;
      const isDimmed =
        selectedSegment !== null && selectedSegment !== seg;
      const line = L.polyline(coords, {
        color,
        weight: isSelected ? 5 : 3,
        opacity: isDimmed ? 0.25 : isSelected ? 0.55 : 0.85,
      });
      line.on("click", (e: any) => {
        L.DomEvent.stopPropagation(e);
        emitSegment(selectedSegment === seg ? null : seg);
      });
      line.addTo(group);
    }

    // Hover-Tooltip on points (one marker per point would be too many — use circle markers per N).
    const STRIDE = Math.max(1, Math.floor(t.points.length / 200));
    for (let i = 0; i < t.points.length; i += STRIDE) {
      const p = t.points[i];
      const isOnSelected = selectedSegment === p.segment;
      const isOnDimmed =
        selectedSegment !== null && selectedSegment !== p.segment;
      const m = L.circleMarker([p.lat, p.lon], {
        radius: isOnSelected ? 3.5 : 2,
        color: SEGMENT_COLORS[p.segment % SEGMENT_COLORS.length],
        fillColor: SEGMENT_COLORS[p.segment % SEGMENT_COLORS.length],
        fillOpacity: isOnDimmed ? 0.25 : 0.95,
        weight: 0,
      });
      const alt = p.alt_baro != null ? `${Math.round(p.alt_baro)} ft` : "–";
      const gs = p.gs != null ? `${Math.round(p.gs)} kt` : "–";
      m.bindTooltip(`${fmtTime(p.ts)} · ${alt} · ${gs}`, { sticky: true });
      const pointSeg = p.segment;
      m.on("click", (e: any) => {
        L.DomEvent.stopPropagation(e);
        emitSegment(selectedSegment === pointSeg ? null : pointSeg);
      });
      m.addTo(group);
    }

    // Start / End markers
    const first = t.points[0];
    const last = t.points[t.points.length - 1];
    L.circleMarker([first.lat, first.lon], {
      radius: 7,
      color: "#10b981",
      fillColor: "#10b981",
      fillOpacity: 0.9,
      weight: 2,
    })
      .bindPopup(`Start ${fmtTime(first.ts)}`)
      .addTo(group);
    L.circleMarker([last.lat, last.lon], {
      radius: 7,
      color: "#ef4444",
      fillColor: "#ef4444",
      fillOpacity: 0.9,
      weight: 2,
    })
      .bindPopup(`Ende ${fmtTime(last.ts)}`)
      .addTo(group);

    // Stop markers
    for (const s of t.stops) {
      L.circleMarker([s.lat, s.lon], {
        radius: 8,
        color: "#000",
        weight: 1,
        fillColor: "#facc15",
        fillOpacity: 0.95,
      })
        .bindPopup(
          `Zwischenstopp · ${fmtDuration(s.duration_s)}<br/>${fmtTime(s.from_ts)} – ${fmtTime(s.to_ts)}`,
        )
        .addTo(group);
    }

    group.addTo(map);
    layer = group;

    const bounds = group.getBounds();
    if (bounds.isValid()) map.fitBounds(bounds, { padding: [30, 30] });
  }
</script>

<div bind:this={mapEl} class="map"></div>

<style>
  .map {
    width: 100%;
    height: 100%;
    background: #0e1116;
  }
</style>
