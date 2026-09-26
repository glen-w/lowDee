import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import abcjs from "abcjs";
import { appendNameRows, pictureModel, renderPicture, showSolfege } from "./picture";
import { aboutHtml } from "./about";
import {
  NODE_COPY,
  remarkFor,
  type AppStore,
  type Background,
  type CatalogView,
  type Evidence,
  type ListenState,
  type Pack,
  type PackNode,
  type Reads,
  type WhistleProfile,
} from "./types";
import {
  WARMUP,
  exerciseIds,
  paintWarm,
  stepNumber,
  warmupStats,
  warmDoneHtml,
  warmIntroHtml,
  warmRunHtml,
  type WarmBeat,
} from "./warmup";
import { glossaryCountLabel, glossaryListHtml, glossaryShellHtml } from "./glossary";
import { ghostMarkup, type GhostTrace } from "./ghost";
import { CNAT_ALT, withAltC } from "./cnat";
import { deskHtml } from "./desk";
import {
  PATH_NOTE,
  practiceNavHtml,
  resumeIndex as firstUnsettledIndex,
  stepNavHtml,
  stepsForNode,
  type StepChoice,
} from "./path";
import "./styles.css";

interface FrameDto {
  t_ms: number;
  hz: number | null;
  rms: number;
  confidence: number;
  near_target: boolean;
  early_break: boolean;
  expected_note: string | null;
  phrase_index: number;
  hold_ratio: number;
  leak_hole: number | null;
}

class App {
  pack!: Pack;
  store!: AppStore;
  profile: WhistleProfile | null = null;
  listenState: ListenState = "idle";
  nodeIndex = 0;
  stairIndex = 0;
  phraseIndex = 0;
  hidePictures = false;
  heardPhrase = false;
  playedPhrase = false;
  remark = "";
  pollTimer: number | null = null;
  holdRatio = 0;
  hearAudio: HTMLAudioElement | null = null;
  hearUrl: string | null = null;
  hearToken = 0;
  frozenTarget: number | null = null;
  liveFrame: FrameDto | null = null;
  wantOctave = false;
  warmed = false;
  warmStarted = false;
  warmAt = 0;
  warmPaused = false;
  warmBeats: WarmBeat[] = WARMUP;
  beatStart = 0;
  beatElapsed = 0;
  warmTimer: number | null = null;
  warmView: "intro" | "run" | "done" | null = null;
  page: "sitting" | "glossary" | "shelf" | "about" | "desk" = "sitting";
  catalog: CatalogView = { shelf_open: false, desk_open: false, packs: [], desk: [], refused: [] };
  ghost: GhostTrace | null = null;
  warmWhistle = false;
  cnatAlt = false;
  recalibrating = false;
  hideWords = false;
  glossaryQuery = "";
  glossaryPausedWarm = false;

  get nodeId(): string {
    return this.pack.manifest.node_ids[this.nodeIndex] ?? "first_sound";
  }

  async init(): Promise<void> {
    if (import.meta.env.DEV && new URLSearchParams(location.search).get("preview") === "picture") {
      const { mountPreview } = await import("./preview.ts");
      mountPreview(document.querySelector("#app")!);
      return;
    }
    if (import.meta.env.DEV && new URLSearchParams(location.search).has("glossary")) {
      this.page = "glossary";
      this.render();
      (document.querySelector("#glossary-q") as HTMLInputElement | null)?.focus();
      return;
    }
    if (import.meta.env.DEV && new URLSearchParams(location.search).has("nav")) {
      const { mountPathPreview } = await import("./path-preview.ts");
      mountPathPreview(document.querySelector("#app")!);
      return;
    }
    if (import.meta.env.DEV && new URLSearchParams(location.search).has("warmup")) {
      this.profile = {
        profile_id: "preview",
        label: "this horn",
        break_hz: 0,
        rms_floor: 0,
        cal_as_of: "",
        reads: "no",
        background: "none",
      };
      this.render();
      return;
    }
    this.store = await invoke<AppStore>("get_store");
    this.catalog = await invoke<CatalogView>("get_catalog");
    if (this.store.active_profile_id) {
      this.profile =
        this.store.profiles.find((p) => p.profile_id === this.store.active_profile_id) ??
        null;
    }
    await this.openPack(this.resumePackId(), false);
    this.nodeIndex = this.resumeIndex();
    this.render();
  }

  resumePackId(): string {
    const open = this.catalog.packs.filter((p) => p.playable && p.open);
    return open.find((p) => !p.settled)?.id ?? open.at(-1)?.id ?? "may-morning-dew";
  }

  async openPack(id: string, renderAfter = true): Promise<void> {
    await invoke("open_pack", { packId: id });
    this.pack = await invoke<Pack>("get_pack");
    this.catalog = await invoke<CatalogView>("get_catalog");
    this.nodeIndex = this.resumeIndex();
    this.stairIndex = 0;
    this.phraseIndex = 0;
    this.heardPhrase = false;
    this.playedPhrase = false;
    this.hidePictures = false;
    this.hideWords = false;
    this.listenState = "idle";
    this.remark = "";
    this.ghost = null;
    this.cnatAlt = false;
    this.recalibrating = false;
    this.page = "sitting";
    if (renderAfter) this.render();
  }

  node(): PackNode | undefined {
    return this.pack?.manifest.nodes?.find((n) => n.id === this.nodeId);
  }

  mode(): string {
    return this.node()?.mode ?? "first_sound";
  }

  resumeIndex(): number {
    return firstUnsettledIndex(this.pack.manifest.node_ids, this.ownedProgress());
  }

  isCalibrated(): boolean {
    return !!this.profile && this.profile.break_hz > 0;
  }

  render(): void {
    const root = document.querySelector("#app")!;
    if (this.page === "glossary") {
      root.innerHTML = glossaryShellHtml(this.glossaryQuery);
      this.bindGlossary();
      return;
    }
    if (this.page === "about") {
      root.innerHTML = aboutHtml();
      document.querySelector("#about-back")?.addEventListener("click", () => {
        this.page = "sitting";
        this.render();
      });
      return;
    }
    if (this.page === "shelf") {
      root.innerHTML = this.shelfHtml();
      this.bindShelf();
      return;
    }
    if (this.page === "desk") {
      root.innerHTML = deskHtml(this.catalog.desk, this.catalog.refused);
      this.bindDesk();
      return;
    }
    if (!this.profile) {
      root.innerHTML = this.objectCardHtml();
      this.bindObjectCard();
      this.paintCardWhistle();
      return;
    }
    if (!this.warmed) {
      this.renderWarm();
      return;
    }
    if (!this.pack) {
      root.innerHTML = `<div class="app-shell"><p class="lede">Warm-up preview. The whistle screen needs the app.</p></div>`;
      return;
    }
    root.innerHTML = this.practiceHtml();
    this.bindPractice();
    this.paintHoles();
    this.paintStaff();
  }

  renderWarm(): void {
    const root = document.querySelector("#app")!;
    const view: "intro" | "run" | "done" = !this.warmStarted
      ? "intro"
      : this.warmAt >= this.warmBeats.length
        ? "done"
        : "run";
    if (view !== this.warmView) {
      this.warmView = view;
      if (view === "intro") root.innerHTML = warmIntroHtml(warmupStats().steps, warmupStats().seconds);
      else if (view === "done") root.innerHTML = warmDoneHtml();
      else root.innerHTML = warmRunHtml();
      this.bindWarm(view);
    }
    if (view === "run") this.paintWarmNow();
  }

  bindWarm(view: "intro" | "run" | "done"): void {
    this.bindGlossaryOpen();
    if (view === "intro") {
      document.querySelector("#warm-begin")?.addEventListener("click", () => this.beginWarm());
      document.querySelector("#warm-skip")?.addEventListener("click", () => this.skipWarmup());
      return;
    }
    if (view === "done") {
      document.querySelector("#warm-done")?.addEventListener("click", () => this.finishWarm());
      return;
    }
    document.querySelector("#warm-next")?.addEventListener("click", () => this.skipWarmExercise());
    document.querySelector("#warm-pause")?.addEventListener("click", () => this.toggleWarmPause());
    document.querySelector("#warm-skip")?.addEventListener("click", () => this.skipWarmup());
  }

  beginWarm(): void {
    this.warmStarted = true;
    this.warmAt = 0;
    this.warmPaused = false;
    this.beatElapsed = 0;
    this.beatStart = performance.now();
    this.warmView = null;
    this.ensureWarmTick();
    this.render();
  }

  ensureWarmTick(): void {
    if (this.warmTimer != null) return;
    this.warmTimer = window.setInterval(() => this.onWarmTick(), 100);
  }

  stopWarmTick(): void {
    if (this.warmTimer != null) {
      window.clearInterval(this.warmTimer);
      this.warmTimer = null;
    }
  }

  onWarmTick(): void {
    if (!this.warmStarted || this.warmed) return;
    if (this.warmPaused) {
      this.paintWarmNow();
      return;
    }
    const beat = this.warmBeats[this.warmAt];
    if (!beat) {
      this.stopWarmTick();
      this.render();
      return;
    }
    const now = performance.now();
    if (now - this.beatStart >= beat.seconds * 1000) {
      this.warmAt += 1;
      this.beatStart = now;
      this.beatElapsed = 0;
      if (this.warmAt >= this.warmBeats.length) {
        this.stopWarmTick();
        this.render();
        return;
      }
    }
    this.paintWarmNow();
  }

  warmElapsedMs(): number {
    if (this.warmPaused) return this.beatElapsed;
    return performance.now() - this.beatStart;
  }

  paintWarmNow(): void {
    const beat = this.warmBeats[this.warmAt];
    const root = document.querySelector("#app");
    if (!beat || !root || !root.querySelector("#warm-mark")) return;
    const elapsedMs = Math.min(this.warmElapsedMs(), beat.seconds * 1000);
    const prior = this.warmBeats.slice(0, this.warmAt).reduce((n, b) => n + b.seconds, 0) * 1000;
    paintWarm(root, {
      beat,
      elapsedMs,
      paused: this.warmPaused,
      step: stepNumber(this.warmBeats, this.warmAt),
      steps: exerciseIds(this.warmBeats).length,
      overallElapsedMs: prior + elapsedMs,
      overallMs: warmupStats(this.warmBeats).seconds * 1000,
    });
  }

  skipWarmExercise(): void {
    const id = this.warmBeats[this.warmAt]?.exercise;
    if (!id) return;
    while (this.warmAt < this.warmBeats.length && this.warmBeats[this.warmAt].exercise === id) {
      this.warmAt += 1;
    }
    this.warmPaused = false;
    this.beatElapsed = 0;
    this.beatStart = performance.now();
    if (this.warmAt >= this.warmBeats.length) {
      this.stopWarmTick();
      this.render();
      return;
    }
    this.paintWarmNow();
  }

  toggleWarmPause(): void {
    const now = performance.now();
    if (this.warmPaused) {
      this.beatStart = now - this.beatElapsed;
      this.warmPaused = false;
    } else {
      this.beatElapsed = now - this.beatStart;
      this.warmPaused = true;
    }
    this.paintWarmNow();
  }

  skipWarmup(): void {
    this.stopWarmTick();
    this.warmed = true;
    this.warmView = null;
    this.render();
  }

  finishWarm(): void {
    this.stopWarmTick();
    this.warmed = true;
    this.warmView = null;
    this.render();
  }

  onGlossaryKey = (event: KeyboardEvent): void => {
    if (event.key !== "Escape" || this.page !== "glossary") return;
    const field = document.querySelector("#glossary-q") as HTMLInputElement | null;
    if (field && document.activeElement === field && field.value) {
      field.value = "";
      this.glossaryQuery = "";
      this.paintGlossary();
      return;
    }
    this.closeGlossary();
  };

  bindGlossaryOpen(): void {
    document.querySelector("#glossary-open")?.addEventListener("click", () => this.openGlossary());
  }

  openGlossary(): void {
    this.glossaryPausedWarm = false;
    if (this.warmStarted && !this.warmed && !this.warmPaused) {
      this.toggleWarmPause();
      this.glossaryPausedWarm = true;
    }
    this.glossaryQuery = "";
    this.page = "glossary";
    this.render();
    (document.querySelector("#glossary-q") as HTMLInputElement | null)?.focus();
  }

  closeGlossary(): void {
    document.removeEventListener("keydown", this.onGlossaryKey);
    const resumeWarm = this.glossaryPausedWarm;
    this.glossaryPausedWarm = false;
    this.glossaryQuery = "";
    this.page = "sitting";
    this.warmView = null;
    if (resumeWarm && this.warmPaused) this.toggleWarmPause();
    this.render();
  }

  paintGlossary(): void {
    const list = document.querySelector("#glossary-list");
    const count = document.querySelector("#glossary-count");
    const jumps = document.querySelector("#glossary-jumps") as HTMLElement | null;
    if (list) list.innerHTML = glossaryListHtml(this.glossaryQuery);
    if (count) count.textContent = glossaryCountLabel(this.glossaryQuery);
    if (jumps) jumps.hidden = this.glossaryQuery.trim().length > 0;
  }

  bindGlossary(): void {
    document.removeEventListener("keydown", this.onGlossaryKey);
    document.addEventListener("keydown", this.onGlossaryKey);
    const field = document.querySelector("#glossary-q") as HTMLInputElement | null;
    field?.addEventListener("input", () => {
      this.glossaryQuery = field.value;
      this.paintGlossary();
    });
    document.querySelector("#glossary-back")?.addEventListener("click", () => this.closeGlossary());
    document.querySelectorAll<HTMLButtonElement>("[data-gloss-jump]").forEach((button) => {
      button.addEventListener("click", () => {
        const id = button.dataset.glossJump;
        if (!id) return;
        document.getElementById(id)?.scrollIntoView({ behavior: "smooth", block: "start" });
      });
    });
  }

  objectCardHtml(): string {
    return `
      <div class="app-shell">
        <div class="topbar">
          <p class="eyebrow">This horn</p>
          <span class="row">
            <button type="button" class="ghost" id="about-open">This tube</button>
            <button type="button" class="ghost" id="glossary-open">Glossary</button>
          </span>
        </div>
        <h1>Name the whistle</h1>
        <p class="lede">
          Some low Ds are too big for some hands. A smaller-holed polymer low D is a fine instrument to learn on.
          There is no microphone test here — only a name, then a warm-up for the hands and the breath, then how to hold it.
        </p>
        ${this.hornListHtml()}
        <div class="card">
          <div id="card-whistle"></div>
          <div class="field">
            <label for="label">Nickname</label>
            <input id="label" placeholder="this horn" value="this horn" />
          </div>
          <div class="field">
            <label for="reads">Do you read music?</label>
            <select id="reads">
              <option value="no">No</option>
              <option value="some">A little</option>
              <option value="yes">Yes</option>
            </select>
          </div>
          <div class="field">
            <label for="background">Musical background</label>
            <select id="background">
              <option value="none">New to music</option>
              <option value="wind">Wind instrument (flute, recorder, …)</option>
              <option value="other">Other instrument or singer</option>
              <option value="high_d">Already play high D whistle</option>
            </select>
          </div>
          <div class="row">
            <button class="primary" id="begin">Hands and breath</button>
          </div>
        </div>
      </div>`;
  }

  bindObjectCard(): void {
    this.bindGlossaryOpen();
    document.querySelector("#about-open")?.addEventListener("click", () => {
      this.page = "about";
      this.render();
    });
    document.querySelectorAll<HTMLButtonElement>("[data-horn]").forEach((btn) => {
      btn.addEventListener("click", () => {
        void this.useHorn(btn.dataset.horn ?? "");
      });
    });
    document.querySelector("#begin")?.addEventListener("click", async () => {
      const label = (document.querySelector("#label") as HTMLInputElement).value.trim() || "this horn";
      const reads = (document.querySelector("#reads") as HTMLSelectElement).value as Reads;
      const background = (document.querySelector("#background") as HTMLSelectElement)
        .value as Background;
      this.profile = await invoke<WhistleProfile>("save_profile", {
        args: { label, reads, background },
      });
      this.store = await invoke<AppStore>("get_store");
      this.render();
    });
  }

  practiceHtml(): string {
    const node = this.node();
    const fallback = NODE_COPY[this.nodeId] ?? { title: this.nodeId, body: "" };
    const copy = node
      ? { title: node.title, body: node.body, body_high_d: node.high_d_body || undefined }
      : fallback;
    const body =
      this.profile?.background === "high_d" && copy.body_high_d
        ? copy.body_high_d
        : copy.body;
    const titles = Object.fromEntries(
      this.pack.manifest.node_ids.map((id) => [
        id,
        this.pack.manifest.nodes?.find((n) => n.id === id)?.title ?? NODE_COPY[id]?.title ?? id,
      ]),
    );
    const nodes = practiceNavHtml({
      nodeIds: this.pack.manifest.node_ids,
      titles,
      current: this.nodeIndex,
      progress: this.ownedProgress(),
    });

    const page = this.isPage();
    const stateCls = this.listenState;
    const words = this.wordLine();
    const next = this.nextPack();
    return `
      <div class="app-shell">
        <div class="topbar">
          <p class="eyebrow">${escapeHtml(this.profile?.label ?? "this horn")} · ${escapeHtml(this.pack.manifest.title)}</p>
          <span class="row">
            ${this.hornSelectHtml()}
            ${this.catalog.desk_open ? `<button type="button" class="ghost" id="desk-open">A tune on the table</button>` : ""}
            ${this.onTheDesk() ? `<button type="button" class="ghost" id="path-back">The path</button>` : ""}
            ${this.catalog.shelf_open ? `<button type="button" class="ghost" id="shelf-open">Shelf</button>` : ""}
            <button type="button" class="ghost" id="about-open">This tube</button>
            <button type="button" class="ghost" id="glossary-open">Glossary</button>
          </span>
        </div>
        ${nodes}
        <p class="meta path-note">${PATH_NOTE}</p>
        <h1>${copy.title}</h1>
        <p class="lede">${body}</p>
        <div class="card">
          ${stepNavHtml(this.stepChoices(), this.stepIndex())}
          ${
            page
              ? `<p class="meta">${this.mode() === "vibrato" ? "Named, and almost never a remark." : "Letters you can already find. On the page, use the key of D."}</p>
          <div id="holes"></div>
          ${this.mode() === "vibrato" ? "" : `<div class="letter-row">${this.letterChips()}</div>`}
          <p class="remark" id="remark">${escapeHtml(this.remark)}</p>
          <div class="row" style="margin-top:1.25rem">
            ${this.pack.manifest.pages?.[this.nodeId]?.url ? `<button class="primary" id="page-btn">Letter notes in D</button>` : ""}
            <button id="play-btn">${this.mode() === "vibrato" ? "I’ve heard it" : "I’ve played the opening"}</button>
          </div>`
              : `<div class="row" style="justify-content:space-between">
            <span class="state-pill ${stateCls}"><i class="dot"></i>${this.listenState}</span>
            <span class="meta" id="target-meta">${this.targetMeta()}</span>
          </div>
          <div id="holes"></div>
          <div id="staff" class="staff" hidden></div>
          ${words && (this.heardPhrase || this.playedPhrase) && !this.hideWords ? `<p class="words">${escapeHtml(words)}</p>` : ""}
          <p class="remark" id="remark">${escapeHtml(this.remark)}</p>
          <div id="ghost">${this.listenState === "feedback" ? ghostMarkup(this.ghost, this.ghostEvidence) : ""}</div>
          ${this.cnatAlt ? `<p class="meta">This fingering did not speak. The bottom two holes close. Not a half-hole.</p>` : ""}
          <div class="hold-meter" ${this.showHold() ? "" : "hidden"}><i id="hold-bar" style="width:${Math.min(100, this.holdRatio * 100)}%"></i></div>
          <div class="row" style="margin-top:1.25rem">
            <button class="primary" id="play-btn">${this.primaryLabel()}</button>
            <button id="hear-btn" hidden>${this.hearAudio ? "Stop" : "Hear"}</button>
            <button id="hear-slow" hidden>Slower</button>
            ${(this.pack.manifest.pulse_beats ?? 0) > 0 ? `<button class="ghost" id="pulse-btn">Hear the pulse</button>` : ""}
            <button class="ghost" id="step-btn">Couldn’t hear — continue</button>
            ${node?.hide_pictures ? `<button class="ghost" id="hide-btn">${this.hidePictures ? "Show pictures" : "Hide pictures"}</button>` : ""}
            ${words ? `<button class="ghost" id="words-btn">${this.hideWords ? "Show words" : "Hide words"}</button>` : ""}
          </div>
          ${this.answerRow()}
          ${next && this.packSettled() ? `<div class="row"><button class="primary" id="next-pack">Next: ${escapeHtml(next.title)}</button></div>` : ""}
          <p class="meta" style="margin-top:1rem">Use headphones so the app doesn’t hear itself.</p>`
          }
        </div>
      </div>`;
  }

  ghostEvidence = "";

  targetMeta(): string {
    if (!this.isCalibrated()) return "Before calibration: is low D there";
    const t = this.frozenTarget ?? this.profile!.break_hz;
    const line = `Target ${t.toFixed(1)} Hz · break ${this.profile!.break_hz.toFixed(1)} Hz`;
    if (this.warmWhistle && this.listenState !== "sounding" && this.listenState !== "wait") {
      return `${line}. This whistle has warmed`;
    }
    return line;
  }

  answerRow(): string {
    if (!this.profile || this.listenState === "sounding" || this.listenState === "wait") return "";
    const reads = this.profile.reads;
    const background = this.profile.background;
    const opt = (value: string, label: string, current: string) =>
      `<option value="${value}"${value === current ? " selected" : ""}>${label}</option>`;
    const recal =
      this.isCalibrated()
        ? `<button type="button" class="ghost" id="recal">Recalibrate</button>`
        : "";
    return `<div class="row answers">
      <label>Reading <select id="reads" aria-label="Reading">
        ${opt("no", "No", reads)}
        ${opt("some", "A little", reads)}
        ${opt("yes", "Yes", reads)}
      </select></label>
      <label>Background <select id="background" aria-label="Background">
        ${opt("none", "New to music", background)}
        ${opt("wind", "Wind", background)}
        ${opt("other", "Other", background)}
        ${opt("high_d", "High D", background)}
      </select></label>
      ${recal}
    </div>`;
  }

  onTheDesk(): boolean {
    const id = this.pack?.manifest.id;
    return !!id && this.catalog.desk.some((p) => p.id === id);
  }

  letterChips(): string {
    return this.pack.phrases.staircase_notes
      .map((note) => this.pack.fingering.notes[note]?.label ?? note)
      .map((label) => `<span>${escapeHtml(label)}</span>`)
      .join("");
  }

  primaryLabel(): string {
    if (this.hearAudio) return "I’m ready";
    switch (this.listenState) {
      case "idle":
        return "I’m ready";
      case "wait":
      case "sounding":
        return "I’m done";
      case "feedback":
        return "Try again";
    }
  }

  currentNote(): string {
    const mode = this.mode();
    if (mode === "first_sound" || this.nodeId === "first_sound") return "D4";
    if (mode === "staircase" || this.nodeId === "staircase") {
      return this.pack.phrases.staircase_notes[this.stairIndex] ?? "D4";
    }
    if (mode === "breath_octave" || this.nodeId === "breath_octave") {
      return this.wantOctave ? "D5" : "D4";
    }
    if (mode === "on_the_breath" || this.nodeId === "on_the_breath") {
      return this.pack.phrases.on_the_breath.notes[0] ?? "D4";
    }
    if (this.isPhraseNode()) {
      return this.pack.phrases.chunks[this.phraseIndex]?.notes[0] ?? "D4";
    }
    if (this.isOrnament()) {
      return this.node()?.note || this.pack.ornaments.demo_note || "A4";
    }
    return "D4";
  }

  ownedProgress() {
    const id = this.profile?.profile_id ?? this.store?.active_profile_id;
    const packId = this.pack?.manifest.id;
    return (this.store?.progress ?? []).filter(
      (p) =>
        (!p.profile_id || p.profile_id === id) &&
        (!p.pack_id || !packId || p.pack_id === packId),
    );
  }

  cutSettled(): boolean {
    const id = this.profile?.profile_id ?? this.store?.active_profile_id;
    return (this.store?.progress ?? []).some(
      (p) =>
        p.pack_id === "may-morning-dew" &&
        p.node_id === "orn_cut" &&
        p.state_reached === "settled" &&
        (!p.profile_id || p.profile_id === id),
    );
  }

  isPhraseNode(): boolean {
    return (
      this.mode() === "phrase" ||
      this.nodeId === "air_bare" ||
      this.nodeId === "air_may_morning_dew"
    );
  }

  isOrnament(): boolean {
    return this.mode() === "ornament" || this.nodeId.startsWith("orn_");
  }

  isPage(): boolean {
    return this.mode() === "page" || this.mode() === "vibrato" || this.nodeId === "hedwig";
  }

  isStair(): boolean {
    return this.mode() === "staircase" || this.nodeId === "staircase";
  }

  showHold(): boolean {
    if (this.recalibrating) return true;
    const mode = this.mode();
    return mode === "first_sound" || mode === "breath_octave" || mode === "staircase";
  }

  pictureNotes(): string[] {
    const mode = this.mode();
    if (mode === "staircase" || this.nodeId === "hedwig" || mode === "page") {
      return this.pack.phrases.staircase_notes.length
        ? this.pack.phrases.staircase_notes
        : [this.currentNote()];
    }
    if (mode === "on_the_breath" || this.nodeId === "on_the_breath") {
      return this.pack.phrases.on_the_breath.notes;
    }
    if (this.isPhraseNode()) {
      return this.pack.phrases.chunks[this.phraseIndex]?.notes ?? ["D4"];
    }
    return [this.currentNote()];
  }

  phraseMarks(): Array<string | null> {
    const notes = this.pictureNotes();
    const marks = notes.map(() => null as string | null);
    if (!this.isPhraseNode()) return marks;
    const chunk = this.pack.phrases.chunks[this.phraseIndex];
    if (!chunk) return marks;
    for (const mark of this.pack.ornaments.marks) {
      if (mark.chunk_id === chunk.id && mark.note_index < marks.length) {
        marks[mark.note_index] = mark.gesture.replaceAll("_", " ");
      }
    }
    return marks;
  }

  gradedMarks(): Array<{ note_index: number; gesture: string }> {
    if (!this.node()?.grade_marks) return [];
    const chunk = this.pack.phrases.chunks[this.phraseIndex];
    if (!chunk) return [];
    return this.pack.ornaments.marks.filter((mark) => {
      if (mark.chunk_id !== chunk.id) return false;
      if (
        mark.gesture === "cut" &&
        this.pack.manifest.id !== "may-morning-dew" &&
        !this.cutSettled()
      ) {
        return false;
      }
      return true;
    });
  }

  wordLine(): string | null {
    const chunk = this.pack?.phrases.chunks[this.phraseIndex];
    if (!chunk) return null;
    return this.pack.words?.lines.find((line) => line.chunk_id === chunk.id)?.text ?? null;
  }

  packSettled(): boolean {
    const ids = this.pack.manifest.node_ids;
    if (ids.length === 0) return false;
    return ids.every(
      (id) => this.ownedProgress().find((p) => p.node_id === id)?.state_reached === "settled",
    );
  }

  nextPack() {
    const packs = this.catalog.packs.filter((p) => p.playable);
    const idx = packs.findIndex((p) => p.id === this.pack.manifest.id);
    return packs.slice(idx + 1).find((p) => p.open && !p.settled) ?? null;
  }

  hornListHtml(): string {
    const horns = this.store?.profiles ?? [];
    if (horns.length === 0) return "";
    return `<div class="horns">${horns
      .map(
        (horn) =>
          `<button type="button" class="ghost" data-horn="${escapeHtml(horn.profile_id)}">${escapeHtml(horn.label)}</button>`,
      )
      .join("")}</div>`;
  }

  hornSelectHtml(): string {
    const horns = this.store?.profiles ?? [];
    if (horns.length < 2) {
      return `<button type="button" class="ghost" id="another-horn">Another whistle</button>`;
    }
    const options = horns
      .map(
        (horn) =>
          `<option value="${escapeHtml(horn.profile_id)}"${horn.profile_id === this.profile?.profile_id ? " selected" : ""}>${escapeHtml(horn.label)}</option>`,
      )
      .join("");
    return `<select id="horn-select" aria-label="Whistle">${options}</select><button type="button" class="ghost" id="another-horn">Another whistle</button>`;
  }

  pictureIndex(): number {
    const phrase = this.isPhraseNode() || this.mode() === "on_the_breath";
    if (this.listenState === "sounding" && this.liveFrame && phrase) {
      return this.liveFrame.phrase_index;
    }
    if (this.isStair()) return this.stairIndex;
    return 0;
  }

  paintCardWhistle(): void {
    const el = document.querySelector("#card-whistle") as HTMLElement | null;
    if (!el || !this.pack) return;
    renderPicture(
      el,
      pictureModel({
        fingering: this.pack.fingering,
        notes: ["D4"],
        currentIndex: 0,
        showSolfege: false,
      }),
    );
  }

  paintHoles(): void {
    const el = document.querySelector("#holes") as HTMLElement | null;
    if (!el || !this.pack) return;
    if (this.hidePictures) {
      el.replaceChildren();
      return;
    }
    const sounding = this.listenState === "sounding" && this.liveFrame != null;
    const fingering = this.cnatAlt ? withAltC(this.pack.fingering) : this.pack.fingering;
    renderPicture(
      el,
      pictureModel({
        fingering,
        notes: this.pictureNotes(),
        currentIndex: this.pictureIndex(),
        leakHole: sounding ? this.liveFrame!.leak_hole : null,
        showSolfege: false,
        marks: this.phraseMarks(),
      }),
    );
  }

  paintStaff(): void {
    const el = document.querySelector("#staff") as HTMLElement | null;
    if (!el) return;
    const reads = this.profile?.reads === "some" || this.profile?.reads === "yes";
    const show =
      reads &&
      this.heardPhrase &&
      !this.hidePictures &&
      (this.isPhraseNode() || this.mode() === "on_the_breath");
    el.hidden = !show;
    if (!show) return;
    const meter = this.pack.phrases.meter || "3/4";
    const key = this.pack.phrases.key || "D";
    let abc = `X:1\nM:${meter}\nL:1/8\nK:${key}\n`;
    if (this.mode() === "on_the_breath") abc += this.pack.phrases.on_the_breath.abc;
    else abc += this.pack.phrases.chunks[this.phraseIndex]?.abc ?? "D3";
    el.innerHTML = "";
    abcjs.renderAbc(el, abc, { responsive: "resize", staffwidth: 480 });
    appendNameRows(
      el,
      pictureModel({
        fingering: this.pack.fingering,
        notes: this.pictureNotes(),
        currentIndex: this.pictureIndex(),
        showSolfege: showSolfege(this.profile?.reads ?? "no"),
        marks: this.phraseMarks(),
      }),
    );
  }

  stepChoices(): StepChoice[] {
    return stepsForNode({
      nodeId: this.nodeId,
      mode: this.mode(),
      staircaseNotes: this.pack.phrases.staircase_notes,
      noteLabel: (note) => this.pack.fingering.notes[note]?.label ?? note,
      chunkLabels: this.pack.phrases.chunks.map((chunk) => chunk.label),
    });
  }

  stepIndex(): number {
    if (this.isStair()) return this.stairIndex;
    if (this.mode() === "breath_octave") return this.wantOctave ? 1 : 0;
    if (this.isPhraseNode()) return this.phraseIndex;
    return 0;
  }

  bindPractice(): void {
    this.bindGlossaryOpen();
    document.querySelector("#about-open")?.addEventListener("click", () => {
      this.page = "about";
      this.render();
    });
    document.querySelector("#shelf-open")?.addEventListener("click", () => {
      this.page = "shelf";
      this.render();
    });
    document.querySelector("#desk-open")?.addEventListener("click", () => {
      this.page = "desk";
      this.render();
    });
    document.querySelector("#path-back")?.addEventListener("click", () => {
      void this.openPack(this.resumePackId());
    });
    document.querySelector("#reads")?.addEventListener("change", (event) => {
      void this.saveAnswers((event.target as HTMLSelectElement).value, this.profile?.background ?? "none");
    });
    document.querySelector("#background")?.addEventListener("change", (event) => {
      void this.saveAnswers(this.profile?.reads ?? "no", (event.target as HTMLSelectElement).value);
    });
    document.querySelector("#recal")?.addEventListener("click", () => {
      void this.onRecalibrate();
    });
    document.querySelector("#another-horn")?.addEventListener("click", () => {
      this.profile = null;
      this.render();
    });
    document.querySelector("#horn-select")?.addEventListener("change", (event) => {
      void this.useHorn((event.target as HTMLSelectElement).value);
    });
    document.querySelector("#next-pack")?.addEventListener("click", () => {
      const next = this.nextPack();
      if (next) void this.openPack(next.id);
    });
    document.querySelector("#pulse-btn")?.addEventListener("click", () => this.playPulse());
    document.querySelector("#words-btn")?.addEventListener("click", () => {
      this.hideWords = !this.hideWords;
      this.render();
    });
    document.querySelectorAll<HTMLButtonElement>("[data-node-index]").forEach((btn) => {
      btn.addEventListener("click", () => {
        void this.goToNode(Number(btn.dataset.nodeIndex));
      });
    });
    document.querySelectorAll<HTMLButtonElement>("[data-step-index]").forEach((btn) => {
      btn.addEventListener("click", () => {
        void this.goToStep(Number(btn.dataset.stepIndex));
      });
    });
    document.querySelector("#play-btn")?.addEventListener("click", () => this.onPrimary());
    document.querySelector("#page-btn")?.addEventListener("click", () => {
      void this.openLetterNotes();
    });
    document.querySelector("#step-btn")?.addEventListener("click", () => this.onStepPast());
    document.querySelector("#hear-btn")?.addEventListener("click", () => this.onHear(1));
    document.querySelector("#hear-slow")?.addEventListener("click", () => this.onHear(0.75));
    document.querySelector("#hide-btn")?.addEventListener("click", () => {
      this.hidePictures = !this.hidePictures;
      this.render();
    });
    void this.refreshHear();
  }

  async leaveAttempt(): Promise<void> {
    this.hearToken += 1;
    this.stopHear();
    this.stopPoll();
    this.liveFrame = null;
    this.frozenTarget = null;
    this.holdRatio = 0;
    this.remark = "";
    this.ghost = null;
    this.recalibrating = false;
    this.listenState = "idle";
    try {
      await invoke("drop_attempt");
    } catch {
      /* microphone already closed */
    }
  }

  async goToNode(index: number): Promise<void> {
    const ids = this.pack.manifest.node_ids;
    if (!Number.isInteger(index) || index < 0 || index >= ids.length || index === this.nodeIndex) {
      return;
    }
    await this.leaveAttempt();
    this.nodeIndex = index;
    this.stairIndex = 0;
    this.phraseIndex = 0;
    this.wantOctave = false;
    this.cnatAlt = false;
    this.heardPhrase = false;
    this.playedPhrase = false;
    if (!this.node()?.hide_pictures) this.hidePictures = false;
    this.render();
  }

  async goToStep(index: number): Promise<void> {
    const steps = this.stepChoices();
    if (!Number.isInteger(index) || index < 0 || index >= steps.length || index === this.stepIndex()) {
      return;
    }
    await this.leaveAttempt();
    if (this.isStair()) this.stairIndex = index;
    else if (this.mode() === "breath_octave") this.wantOctave = index === 1;
    else this.phraseIndex = index;
    this.heardPhrase = false;
    this.playedPhrase = false;
    this.render();
  }

  async refreshHear(): Promise<void> {
    const btn = document.querySelector("#hear-btn") as HTMLButtonElement | null;
    const slow = document.querySelector("#hear-slow") as HTMLButtonElement | null;
    if (!btn) return;
    const rel = this.refPath();
    if (!rel || this.hearAudio) {
      btn.hidden = !this.hearAudio;
      if (slow) slow.hidden = true;
      return;
    }
    try {
      const ok = await invoke<boolean>("ref_available", { relative: rel });
      btn.hidden = !ok;
      if (slow) slow.hidden = !ok || this.isOrnament();
    } catch {
      btn.hidden = true;
      if (slow) slow.hidden = true;
    }
  }

  refPath(): string | null {
    const mode = this.mode();
    if (mode === "first_sound" || mode === "staircase" || mode === "breath_octave") {
      return `ref/${this.currentNote()}.wav`;
    }
    if (mode === "on_the_breath") return this.pack.phrases.on_the_breath.ref || null;
    if (this.isPhraseNode()) return this.pack.phrases.chunks[this.phraseIndex]?.ref ?? null;
    if (this.isOrnament()) {
      const gesture = this.node()?.gesture || "cut";
      return `ref/${gesture}_demo.wav`;
    }
    return null;
  }

  stopHear(): void {
    const audio = this.hearAudio;
    this.hearAudio = null;
    if (audio) {
      audio.onended = null;
      audio.pause();
      audio.removeAttribute("src");
      audio.load();
    }
    if (this.hearUrl) {
      URL.revokeObjectURL(this.hearUrl);
      this.hearUrl = null;
    }
  }

  async finishHear(token: number): Promise<void> {
    if (token !== this.hearToken) return;
    this.stopHear();
    await invoke("set_grading", { enabled: true });
    if (this.listenState === "wait") {
      this.listenState = "idle";
      this.render();
    }
  }

  async onHear(rate = 1): Promise<void> {
    if (this.hearAudio) {
      this.hearToken += 1;
      this.stopHear();
      await invoke("set_grading", { enabled: true });
      this.listenState = "idle";
      this.render();
      return;
    }
    const rel = this.refPath();
    if (!rel) return;
    if (this.isOrnament()) rate = 1;
    const raw = await invoke<ArrayBuffer | number[]>("read_ref", { relative: rel });
    const bytes = raw instanceof ArrayBuffer ? new Uint8Array(raw) : Uint8Array.from(raw);
    this.hearToken += 1;
    const token = this.hearToken;
    const blob = new Blob([bytes], { type: "audio/wav" });
    const url = URL.createObjectURL(blob);
    const audio = new Audio(url);
    audio.playbackRate = rate;
    this.hearUrl = url;
    this.hearAudio = audio;
    await invoke("set_grading", { enabled: false });
    this.listenState = "wait";
    this.heardPhrase = true;
    this.remark = "";
    this.render();
    audio.onended = () => {
      void this.finishHear(token);
    };
    try {
      await audio.play();
    } catch {
      await this.finishHear(token);
    }
  }

  attemptArgs(): Record<string, unknown> {
    const node_id = this.nodeId;
    if (this.recalibrating) return { node_id, mode: "first_sound", recalibrate: true };
    const mode = this.mode();
    if (mode === "first_sound") return { node_id, mode: "first_sound" };
    if (mode === "staircase") return { node_id, mode: "single_note", note: this.currentNote() };
    if (mode === "breath_octave") {
      return { node_id, mode: "breath_octave", want_octave: this.wantOctave };
    }
    if (mode === "on_the_breath") {
      return { node_id, mode: "on_the_breath", notes: this.pack.phrases.on_the_breath.notes };
    }
    if (mode === "phrase") {
      const chunk = this.pack.phrases.chunks[this.phraseIndex];
      return {
        node_id,
        mode: "phrase",
        notes: chunk?.notes ?? ["D4"],
        breaths: chunk?.breaths ?? [],
        marks: this.gradedMarks(),
      };
    }
    if (mode === "ornament") {
      return {
        node_id,
        mode: "ornament",
        note: this.currentNote(),
        gesture: this.node()?.gesture || "cut",
      };
    }
    return { node_id, mode: "first_sound" };
  }

  async openLetterNotes(): Promise<void> {
    const page = this.pack.manifest.pages?.[this.nodeId];
    if (!page?.url) {
      this.remark = "The letter-note page isn’t in this pack.";
      const el = document.querySelector("#remark");
      if (el) el.textContent = this.remark;
      return;
    }
    try {
      await openUrl(page.url);
    } catch (e) {
      this.remark = "Couldn’t open the letter notes.";
      const el = document.querySelector("#remark");
      if (el) el.textContent = this.remark;
      console.warn(e);
    }
  }

  async settlePage(): Promise<void> {
    await invoke("step_past", { nodeId: this.nodeId });
    this.store = await invoke<AppStore>("get_store");
    this.catalog = await invoke<CatalogView>("get_catalog");
    this.remark = "";
    if (this.nodeIndex + 1 < this.pack.manifest.node_ids.length) {
      this.nodeIndex += 1;
      this.stairIndex = 0;
      this.phraseIndex = 0;
      this.heardPhrase = false;
      this.playedPhrase = false;
      this.hidePictures = false;
    }
    this.listenState = "idle";
    this.render();
  }

  async onPrimary(): Promise<void> {
    if (this.isPage()) {
      await this.settlePage();
      return;
    }
    if (this.hearAudio) {
      this.hearToken += 1;
      this.stopHear();
      await invoke("set_grading", { enabled: true });
      this.listenState = "idle";
    }
    if (this.listenState === "idle" || this.listenState === "feedback") {
      this.remark = "";
      this.ghost = null;
      this.holdRatio = 0;
      this.listenState = "wait";
      const args = this.attemptArgs();
      // Tauri 2: flatten or nest? Our command expects `args: StartAttemptArgs`
      const res = await invoke<{ ok: boolean; mic: boolean; target_hz?: number }>(
        "start_attempt",
        { args },
      );
      if (!res.mic) {
        this.recalibrating = false;
        this.listenState = "idle";
        this.liveFrame = null;
        this.remark = remarkFor(this.pack.remarks, "couldnt_hear", this.profile);
        this.render();
        return;
      }
      this.frozenTarget = res.target_hz ?? this.profile?.break_hz ?? null;
      this.liveFrame = null;
      this.listenState = "sounding";
      this.render();
      this.startPoll();
      return;
    }
    if (this.listenState === "wait" || this.listenState === "sounding") {
      await this.finish();
    }
  }

  startPoll(): void {
    this.stopPoll();
    this.pollTimer = window.setInterval(async () => {
      try {
        const frame = await invoke<FrameDto | null>("poll_frame");
        if (!frame) return;
        this.liveFrame = frame;
        if (frame.hz != null) this.listenState = "sounding";
        if (this.showHold()) {
          this.holdRatio = frame.hold_ratio;
          const bar = document.querySelector("#hold-bar") as HTMLElement | null;
          if (bar) bar.style.width = `${Math.min(100, this.holdRatio * 100)}%`;
        }
        this.paintHoles();
        const holdNode =
          this.mode() === "first_sound" ||
          (this.mode() === "breath_octave" && !this.wantOctave);
        if (frame.early_break && holdNode) {
          this.remark = remarkFor(this.pack.remarks, "early_break", this.profile);
          const el = document.querySelector("#remark");
          if (el) el.textContent = this.remark;
        }
        const pill = document.querySelector(".state-pill");
        if (pill) {
          pill.className = `state-pill ${this.listenState}`;
          pill.innerHTML = `<i class="dot"></i>${this.listenState}`;
        }
      } catch {
        /* ignore */
      }
    }, 120);
  }

  stopPoll(): void {
    if (this.pollTimer != null) {
      clearInterval(this.pollTimer);
      this.pollTimer = null;
    }
  }

  shouldMarkSettled(): boolean {
    if (this.isStair()) {
      return this.stairIndex + 1 >= this.pack.phrases.staircase_notes.length;
    }
    if (this.isPhraseNode()) {
      return this.phraseIndex + 1 >= this.pack.phrases.chunks.length;
    }
    return true;
  }

  async finish(): Promise<void> {
    this.hearToken += 1;
    this.stopHear();
    this.stopPoll();
    const markSettled = this.shouldMarkSettled();
    const wasRecal = this.recalibrating;
    const result = await invoke<{
      evidence: Evidence;
      settled: boolean;
      break_hz: number | null;
      target_hz: number | null;
      remark_note: string | null;
      warm: boolean;
      cnat_disagree: boolean;
      ghost: GhostTrace | null;
    }>("finish_attempt", {
      nodeId: this.nodeId,
      markSettled: wasRecal ? false : markSettled,
      recalibrate: wasRecal,
      cnatFingering:
        this.cnatAlt && this.pack.manifest.id === "c-natural" && this.nodeId === "cnat_hold"
          ? CNAT_ALT
          : undefined,
    });

    // Target must not have drifted
    if (
      this.frozenTarget != null &&
      result.target_hz != null &&
      Math.abs(result.target_hz - this.frozenTarget) > 0.5
    ) {
      console.warn("target drifted", this.frozenTarget, result.target_hz);
    }

    this.remark = remarkFor(
      this.pack.remarks,
      result.evidence,
      this.profile,
      result.remark_note,
    );
    this.ghost = result.ghost;
    this.ghostEvidence = result.evidence;
    this.warmWhistle = wasRecal && result.settled ? false : result.warm;
    if (
      result.cnat_disagree &&
      this.pack.manifest.id === "c-natural" &&
      this.nodeId === "cnat_hold"
    ) {
      this.cnatAlt = true;
    }
    this.recalibrating = false;
    this.listenState = "feedback";
    this.playedPhrase = true;
    this.store = await invoke<AppStore>("get_store");
    this.catalog = await invoke<CatalogView>("get_catalog");
    if (this.profile) {
      const refreshed = this.store.profiles.find(
        (p) => p.profile_id === this.profile!.profile_id,
      );
      if (refreshed) this.profile = refreshed;
    }

    if (result.settled && !wasRecal) {
      await this.advanceAfterSettle();
    }
    this.render();
  }

  async saveAnswers(reads: string, background: string): Promise<void> {
    if (!this.profile) return;
    const profile = await invoke<WhistleProfile>("update_profile_answers", {
      profileId: this.profile.profile_id,
      reads,
      background,
    });
    this.profile = profile;
    const stored = this.store.profiles.find((p) => p.profile_id === profile.profile_id);
    if (stored) {
      stored.reads = profile.reads;
      stored.background = profile.background;
    }
    this.render();
  }

  async onRecalibrate(): Promise<void> {
    if (this.listenState === "sounding" || this.listenState === "wait") return;
    this.recalibrating = true;
    this.listenState = "idle";
    await this.onPrimary();
  }

  async advanceAfterSettle(): Promise<void> {
    if (this.isStair()) {
      if (this.stairIndex + 1 < this.pack.phrases.staircase_notes.length) {
        this.stairIndex += 1;
        this.listenState = "idle";
        this.remark = remarkFor(this.pack.remarks, "note_found", this.profile, this.currentNote());
        return;
      }
    }
    if (this.isPhraseNode()) {
      if (this.phraseIndex + 1 < this.pack.phrases.chunks.length) {
        this.phraseIndex += 1;
        this.heardPhrase = false;
        this.playedPhrase = false;
        this.listenState = "idle";
        return;
      }
    }
    if (this.nodeIndex + 1 < this.pack.manifest.node_ids.length) {
      this.nodeIndex += 1;
      this.stairIndex = 0;
      this.phraseIndex = 0;
      this.heardPhrase = false;
      this.playedPhrase = false;
      this.hidePictures = !!this.node()?.hide_pictures && this.hidePictures;
      this.listenState = "idle";
    }
  }

  async onStepPast(): Promise<void> {
    this.hearToken += 1;
    this.stopHear();
    this.stopPoll();
    await invoke("step_past", { nodeId: this.nodeId });
    this.store = await invoke<AppStore>("get_store");
    this.catalog = await invoke<CatalogView>("get_catalog");
    this.remark = "";
    await this.advanceAfterSettle();
    // force node advance if still on same (advanceAfterSettle may only bump phrase)
    if (this.ownedProgress().find((p) => p.node_id === this.nodeId)?.state_reached === "settled") {
      if (this.nodeIndex + 1 < this.pack.manifest.node_ids.length) {
        this.nodeIndex += 1;
        this.stairIndex = 0;
        this.phraseIndex = 0;
      }
    }
    this.listenState = "idle";
    this.render();
  }

  async useHorn(profileId: string): Promise<void> {
    if (!profileId) return;
    await invoke("select_profile", { profileId });
    this.store = await invoke<AppStore>("get_store");
    this.catalog = await invoke<CatalogView>("get_catalog");
    this.profile = this.store.profiles.find((p) => p.profile_id === profileId) ?? null;
    await this.openPack(this.resumePackId());
  }

  playPulse(): void {
    const beats = this.pack.manifest.pulse_beats ?? 0;
    if (beats < 1) return;
    const ctx = new AudioContext();
    const gap = 0.32;
    for (let i = 0; i < beats * 2; i++) {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.frequency.value = i % beats === 0 ? 660 : 440;
      gain.gain.value = 0.06;
      osc.connect(gain);
      gain.connect(ctx.destination);
      const start = ctx.currentTime + i * gap;
      osc.start(start);
      osc.stop(start + 0.05);
    }
  }

  shelfHtml(): string {
    const groups = ["path", "beginner", "improver", "players"];
    const blocks = groups
      .map((shelf) => {
        const packs = this.catalog.packs.filter((p) => p.shelf === shelf);
        if (packs.length === 0) return "";
        const cards = packs
          .map((pack) => {
            const aka = pack.aka.length ? `<p class="meta">${escapeHtml(pack.aka.join(" · "))}</p>` : "";
            const ref = pack.book_ref ? `<p class="meta">${escapeHtml(pack.book_ref)}</p>` : "";
            const open = pack.playable && pack.open
              ? `<button type="button" class="primary" data-open-pack="${escapeHtml(pack.id)}">Open</button>`
              : "";
            const session = pack.session
              ? `<button type="button" class="ghost" data-session="${escapeHtml(pack.session)}">On The Session</button>`
              : "";
            const note = pack.playable ? "" : `<p class="meta">In the book. No notes and no audio here.</p>`;
            return `<article class="shelf-card"><h2>${escapeHtml(pack.title)}</h2>${ref}${aka}${note}<div class="row">${open}${session}</div></article>`;
          })
          .join("");
        return `<h2>${escapeHtml(shelf)}</h2>${cards}`;
      })
      .join("");
    return `
      <div class="app-shell">
        <div class="topbar">
          <p class="eyebrow">Shelf</p>
          <button type="button" class="ghost" id="shelf-back">Back</button>
        </div>
          <h1>After the second air</h1>
        <p class="lede">An index of packs. The door is still The May Morning Dew.</p>
        ${this.catalog.desk_open ? `<p><button type="button" class="ghost" id="desk-open">A tune on the table</button></p>` : ""}
        <div class="shelf-list">${blocks}</div>
      </div>`;
  }

  bindShelf(): void {
    document.querySelector("#shelf-back")?.addEventListener("click", () => {
      this.page = "sitting";
      this.render();
    });
    document.querySelector("#desk-open")?.addEventListener("click", () => {
      this.page = "desk";
      this.render();
    });
    document.querySelectorAll<HTMLButtonElement>("[data-open-pack]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const id = btn.dataset.openPack;
        if (id) void this.openPack(id);
      });
    });
    document.querySelectorAll<HTMLButtonElement>("[data-session]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const url = btn.dataset.session;
        if (url) void openUrl(url);
      });
    });
  }

  bindDesk(): void {
    document.querySelector("#desk-back")?.addEventListener("click", () => {
      this.page = "sitting";
      this.render();
    });
    document.querySelectorAll<HTMLButtonElement>("[data-open-pack]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const id = btn.dataset.openPack;
        if (id) void this.openPack(id);
      });
    });
  }
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

const app = new App();
app.init().catch((e) => {
  document.querySelector("#app")!.innerHTML = `<div class="app-shell"><p class="remark">Couldn’t start: ${escapeHtml(String(e))}</p></div>`;
});
