<script lang="ts">
  import { onMount } from "svelte";
  import TrackMap from "$lib/TrackMap.svelte";
  import { getAircraft, getDays, getTrack } from "$lib/api";
  import type { AircraftSummary, SegmentStats, Track } from "$lib/types";

  let aircraft = $state<AircraftSummary[]>([]);
  let icaoInput = $state<string>("");
  let selectedIcao = $state<string>("");
  let days = $state<string[]>([]);
  let selectedDate = $state<string>("");
  let track = $state<Track | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let selectedSegment = $state<number | null>(null);
  let showAirports = $state<boolean>(false);

  const selected = $derived(
    aircraft.find((a) => a.icao === selectedIcao) ?? null,
  );

  const activeStats = $derived<SegmentStats | null>(
    track == null
      ? null
      : selectedSegment === null
        ? track.overall
        : (track.segments.find((s) => s.segment === selectedSegment) ?? null),
  );

  function fmtNum(n: number, digits = 0): string {
    return n.toLocaleString("de-DE", {
      minimumFractionDigits: digits,
      maximumFractionDigits: digits,
    });
  }

  function fmtClock(ts: number): string {
    return new Date(ts * 1000).toLocaleTimeString("de-DE", {
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

  onMount(async () => {
    try {
      aircraft = await getAircraft();
    } catch (e) {
      error = `Konnte Aircraft-Liste nicht laden: ${(e as Error).message}`;
    }
  });

  async function applyIcao() {
    days = [];
    selectedDate = "";
    track = null;
    selectedSegment = null;
    error = null;
    const normalized = icaoInput.trim().toLowerCase();
    selectedIcao = normalized;
    if (!normalized) return;
    if (aircraft.length > 0 && !aircraft.some((a) => a.icao === normalized)) {
      error = `ICAO ${normalized} nicht in der Flotte gefunden.`;
      return;
    }
    try {
      days = await getDays(normalized);
      if (days.length > 0) {
        selectedDate = days[0];
        await loadTrack();
      } else {
        error = `Keine Tage für ${normalized} verfügbar.`;
      }
    } catch (e) {
      error = `Tage konnten nicht geladen werden: ${(e as Error).message}`;
    }
  }

  function onIcaoKey(e: KeyboardEvent) {
    if (e.key === "Enter") applyIcao();
  }

  async function loadTrack() {
    if (!selectedIcao || !selectedDate) return;
    loading = true;
    error = null;
    selectedSegment = null;
    try {
      track = await getTrack(selectedIcao, selectedDate);
    } catch (e) {
      error = `Track konnte nicht geladen werden: ${(e as Error).message}`;
      track = null;
    } finally {
      loading = false;
    }
  }

  function onSegmentClick(seg: number | null) {
    selectedSegment = seg;
  }
</script>

<div class="app">
  <header>
    <h1>squawk7700</h1>
    <div class="controls">
      <label>
        ICAO
        <input
          type="text"
          bind:value={icaoInput}
          onkeydown={onIcaoKey}
          placeholder="z. B. 3f4567"
          spellcheck="false"
          autocapitalize="off"
          autocomplete="off"
        />
      </label>
      <button type="button" onclick={applyIcao}>Anzeigen</button>

      <label>
        Tag
        <select
          bind:value={selectedDate}
          onchange={loadTrack}
          disabled={days.length === 0}
        >
          <option value="">— wählen —</option>
          {#each days as d}
            <option value={d}>{d}</option>
          {/each}
        </select>
      </label>

      <button
        type="button"
        class="toggle"
        class:active={showAirports}
        onclick={() => (showAirports = !showAirports)}
        title="Deutsche Flughäfen ein-/ausblenden"
      >
        Flughäfen {showAirports ? "an" : "aus"}
      </button>

      {#if loading}<span class="status">lädt …</span>{/if}
      {#if error}<span class="status err">{error}</span>{/if}
      {#if track && !loading && !error}
        <span class="status">
          {track.points.length} Punkte · {track.stops.length} Stopps
        </span>
      {/if}
    </div>
    {#if selected}
      <div class="meta">
        {selected.callsign ?? "—"} · {selected.icao}
        {#if selected.aircraft_type}· {selected.aircraft_type}{/if}
        {#if selected.registration}· {selected.registration}{/if}
      </div>
    {/if}
  </header>

  <main>
    <div class="map-wrap">
      <TrackMap {track} {selectedSegment} {showAirports} {onSegmentClick} />
    </div>
    <aside class="panel">
      {#if !track}
        <p class="muted">Keinen Track geladen.</p>
      {:else if activeStats == null}
        <p class="muted">Segment {selectedSegment} nicht gefunden.</p>
      {:else}
        <header class="panel-head">
          {#if selectedSegment === null}
            <h2>Gesamter Tag</h2>
            <p class="muted">{track.segments.length} Segment(e) · auf Linie klicken zum Filtern</p>
          {:else}
            <h2>Segment {selectedSegment}</h2>
            <button
              type="button"
              class="link"
              onclick={() => (selectedSegment = null)}
            >
              ← zurück zum Gesamttag
            </button>
          {/if}
        </header>

        <section>
          <h3>Zeitraum</h3>
          {#if activeStats.start_ts != null && activeStats.end_ts != null}
            <p>
              {fmtClock(activeStats.start_ts)} – {fmtClock(activeStats.end_ts)}
              <span class="muted">
                · {fmtDuration(activeStats.end_ts - activeStats.start_ts)}
              </span>
            </p>
          {:else}
            <p class="muted">keine Angabe</p>
          {/if}
        </section>

        <section>
          <h3>Punkte</h3>
          <p>{fmtNum(activeStats.point_count)}</p>
        </section>

        <section>
          <h3>Quelle der Ortung</h3>
          {#if activeStats.source_types.length === 0}
            <p class="muted">keine Angabe</p>
          {:else}
            <ul class="kv">
              {#each activeStats.source_types as st}
                <li>
                  <span class="k">{st.source_type}</span>
                  <span class="v">{fmtNum(st.count)}</span>
                </li>
              {/each}
            </ul>
          {/if}
        </section>

        <section>
          <h3>MLAT-Receiver</h3>
          {#if activeStats.mlat_receivers}
            <ul class="kv">
              <li><span class="k">min</span><span class="v">{activeStats.mlat_receivers.min}</span></li>
              <li><span class="k">max</span><span class="v">{activeStats.mlat_receivers.max}</span></li>
              <li><span class="k">ø</span><span class="v">{fmtNum(activeStats.mlat_receivers.avg, 1)}</span></li>
              <li><span class="k">Samples</span><span class="v">{fmtNum(activeStats.mlat_receivers.samples)}</span></li>
            </ul>
          {:else}
            <p class="muted">keine MLAT-Daten</p>
          {/if}
        </section>

        <section>
          <h3>Geschwindigkeit</h3>
          <ul class="kv">
            <li>
              <span class="k">max GS</span>
              <span class="v">
                {activeStats.max_gs != null ? `${fmtNum(activeStats.max_gs, 0)} kt` : "–"}
              </span>
            </li>
            <li>
              <span class="k">max Mach</span>
              <span class="v">
                {activeStats.max_mach != null ? fmtNum(activeStats.max_mach, 2) : "–"}
              </span>
            </li>
          </ul>
        </section>
      {/if}
    </aside>
  </main>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  header {
    padding: 0.6rem 1rem;
    background: #1a1f2b;
    border-bottom: 1px solid #2a3042;
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }
  h1 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    letter-spacing: 0.02em;
  }
  .controls {
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
  }
  label {
    display: flex;
    flex-direction: column;
    font-size: 0.75rem;
    color: #93a4be;
  }
  select,
  input {
    background: #0e1116;
    color: #e6e6e6;
    border: 1px solid #2a3042;
    border-radius: 4px;
    padding: 0.35rem 0.5rem;
    font-size: 0.9rem;
    min-width: 240px;
  }
  input {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    text-transform: lowercase;
  }
  button {
    background: #2a3042;
    color: #e6e6e6;
    border: 1px solid #3a4258;
    border-radius: 4px;
    padding: 0.4rem 0.9rem;
    font-size: 0.85rem;
    cursor: pointer;
    align-self: flex-end;
  }
  button:hover {
    background: #34394d;
  }
  .toggle.active {
    background: #e6e6e6;
    border-color: #ffffff;
    color: #0f172a;
  }
  .toggle.active:hover {
    background: #ffffff;
  }
  .status {
    font-size: 0.85rem;
    color: #93a4be;
  }
  .status.err {
    color: #f87171;
  }
  .meta {
    font-size: 0.8rem;
    color: #93a4be;
  }
  main {
    flex: 1;
    min-height: 0;
    display: grid;
    grid-template-columns: 1fr 320px;
  }
  .map-wrap {
    min-width: 0;
    min-height: 0;
  }
  .panel {
    border-left: 1px solid #2a3042;
    background: #141822;
    color: #e6e6e6;
    padding: 0.9rem 1rem;
    overflow-y: auto;
    font-size: 0.85rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .panel-head {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    border-bottom: 1px solid #2a3042;
    padding-bottom: 0.6rem;
  }
  .panel h2 {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 600;
  }
  .panel h3 {
    margin: 0 0 0.35rem;
    font-size: 0.7rem;
    font-weight: 600;
    color: #93a4be;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .panel p {
    margin: 0;
  }
  .muted {
    color: #93a4be;
    font-size: 0.8rem;
  }
  .kv {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }
  .kv li {
    display: flex;
    justify-content: space-between;
    gap: 0.5rem;
    font-variant-numeric: tabular-nums;
  }
  .kv .k {
    color: #93a4be;
  }
  .link {
    background: none;
    border: none;
    color: #93a4be;
    padding: 0;
    font-size: 0.8rem;
    text-align: left;
    cursor: pointer;
    align-self: flex-start;
  }
  .link:hover {
    color: #e6e6e6;
  }
</style>
