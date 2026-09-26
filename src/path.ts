export interface ProgressMark {
  node_id: string;
  state_reached: string;
}

export interface StepChoice {
  label: string;
  whole?: boolean;
}

export const WHOLE_SONG_LABEL = "Whole song";

export interface SongChunk {
  id: string;
  notes: readonly string[];
  abc: string;
  breaths?: readonly number[];
}

export interface SongMark {
  chunk_id: string;
  note_index: number;
  gesture: string;
}

export interface JoinedSong {
  notes: string[];
  abc: string;
  breaths: number[];
  marks: Array<{ note_index: number; gesture: string }>;
}

/** Lines in order. A breath between lines is allowed when every line already names its breaths. */
export function joinedSong(chunks: readonly SongChunk[], marks: readonly SongMark[] = []): JoinedSong {
  const notes = chunks.flatMap((chunk) => [...chunk.notes]);
  const abc = chunks
    .map((chunk) => chunk.abc.trim())
    .filter((line) => line.length > 0)
    .join(" ");
  const open = chunks.some((chunk) => !chunk.breaths || chunk.breaths.length === 0);
  const breaths: number[] = [];
  if (!open) {
    let offset = 0;
    for (let i = 0; i < chunks.length; i++) {
      const chunk = chunks[i];
      for (const breath of chunk.breaths ?? []) breaths.push(breath + offset);
      const count = chunk.notes.length;
      if (i < chunks.length - 1 && count > 0) {
        const end = offset + count - 1;
        if (!breaths.includes(end)) breaths.push(end);
      }
      offset += count;
    }
  }
  let offset = 0;
  const joinedMarks: JoinedSong["marks"] = [];
  for (const chunk of chunks) {
    for (const mark of marks) {
      if (mark.chunk_id !== chunk.id) continue;
      joinedMarks.push({ note_index: mark.note_index + offset, gesture: mark.gesture });
    }
    offset += chunk.notes.length;
  }
  return { notes, abc, breaths, marks: joinedMarks };
}

export const PATH_NOTE = "Any part is open. The marks record what you’ve already done.";

/** First unsettled node. A later settle does not pull the sitting past an earlier gap. */
export function resumeIndex(nodeIds: readonly string[], progress: readonly ProgressMark[]): number {
  if (nodeIds.length === 0) return 0;
  let idx = 0;
  for (let i = 0; i < nodeIds.length; i++) {
    const hit = progress.find((p) => p.node_id === nodeIds[i]);
    if (hit?.state_reached === "settled") idx = Math.min(i + 1, nodeIds.length - 1);
    else break;
  }
  return idx;
}

export function nodeStatus(
  progress: readonly ProgressMark[],
  nodeId: string,
): "settled" | "started" | "open" {
  const hit = progress.find((p) => p.node_id === nodeId);
  if (hit?.state_reached === "settled") return "settled";
  if (hit?.state_reached === "started") return "started";
  return "open";
}

export function stepsForNode(opts: {
  nodeId: string;
  mode?: string;
  staircaseNotes: readonly string[];
  noteLabel: (note: string) => string;
  chunkLabels: readonly string[];
  song?: boolean;
}): StepChoice[] {
  const mode = opts.mode ?? opts.nodeId;
  if (mode === "staircase" || opts.nodeId === "staircase") {
    return opts.staircaseNotes.map((note) => ({ label: opts.noteLabel(note) }));
  }
  if (mode === "breath_octave" || opts.nodeId === "breath_octave") {
    return [{ label: "Low D" }, { label: "Octave" }];
  }
  if (
    mode === "phrase" ||
    opts.nodeId === "air_bare" ||
    opts.nodeId === "air_may_morning_dew"
  ) {
    const steps: StepChoice[] = opts.chunkLabels.map((label) => ({ label }));
    if (opts.song && steps.length > 1) steps.push({ label: WHOLE_SONG_LABEL, whole: true });
    return steps;
  }
  return [];
}

export type MenuKind = "lesson" | "song";

/** Drills stay lessons. A page, the air, or a phrase titled as the pack is a song. */
export function menuKindFor(opts: {
  id: string;
  mode?: string;
  title: string;
  packTitle: string;
}): MenuKind {
  const mode = opts.mode || opts.id;
  if (mode === "page") return "song";
  if (mode === "phrase" && (opts.id.startsWith("air_") || opts.title === opts.packTitle)) {
    return "song";
  }
  return "lesson";
}

export function practiceNavHtml(opts: {
  nodeIds: readonly string[];
  titles: Readonly<Record<string, string>>;
  current: number;
  progress: readonly ProgressMark[];
  kinds?: Readonly<Record<string, MenuKind>>;
}): string {
  const parts = opts.nodeIds.map((id, i) => {
    const status = nodeStatus(opts.progress, id);
    const current = i === opts.current;
    const mark = status === "settled" ? "done" : status === "started" ? "started" : "";
    const cls = ["node-jump", current ? "current" : "", mark].filter(Boolean).join(" ");
    const title = opts.titles[id] ?? id;
    const word = status === "open" ? "not yet" : status;
    const here = current ? ` aria-current="true"` : "";
    const html = `<button type="button" class="${cls}" data-node-index="${i}"${here} aria-label="${escapeHtml(title)}, ${word}">${escapeHtml(title)}</button>`;
    return { html, kind: opts.kinds?.[id] ?? "lesson" };
  });
  const songs = parts.filter((part) => part.kind === "song");
  const lessons = parts.filter((part) => part.kind === "lesson");
  if (songs.length === 0 || lessons.length === 0) {
    return `<nav class="node-rail" aria-label="Practice parts">${parts.map((part) => part.html).join("")}</nav>`;
  }
  const group = (label: string, items: typeof parts) =>
    `<div class="rail-group"><p class="rail-kicker">${label}</p><div class="rail-pills">${items.map((part) => part.html).join("")}</div></div>`;
  return `<nav class="node-rail grouped" aria-label="Practice parts">${group("Lessons", lessons)}${group("Songs", songs)}</nav>`;
}

export interface LessonChoice {
  id: string;
  title: string;
  source: string;
  song: boolean;
  playable: boolean;
  open: boolean;
  rights: string;
}

/** Playable public-domain songs the catalog has already opened. */
export function lessonCandidates(packs: readonly LessonChoice[]): LessonChoice[] {
  return packs.filter(
    (pack) =>
      pack.song &&
      pack.playable &&
      pack.open &&
      pack.rights === "pd" &&
      pack.source.trim().length > 0,
  );
}

export interface LessonSetView {
  saved: LessonChoice[];
  addable: LessonChoice[];
}

/** Saved ids stay in the order given. One that is no longer a choice drops out of both lists. */
export function lessonSetView(packs: readonly LessonChoice[], savedIds: readonly string[]): LessonSetView {
  const allowed = lessonCandidates(packs);
  const byId = new Map(allowed.map((pack) => [pack.id, pack]));
  const saved: LessonChoice[] = [];
  for (const id of savedIds) {
    const pack = byId.get(id);
    if (pack && !saved.some((kept) => kept.id === pack.id)) saved.push(pack);
  }
  const have = new Set(saved.map((pack) => pack.id));
  return { saved, addable: allowed.filter((pack) => !have.has(pack.id)) };
}

export function lessonSetHtml(opts: {
  packs: readonly LessonChoice[];
  savedIds: readonly string[];
  currentId: string;
}): string {
  const view = lessonSetView(opts.packs, opts.savedIds);
  if (view.saved.length === 0 && view.addable.length === 0) return "";
  const pills = view.saved
    .map((pack) => {
      const here = pack.id === opts.currentId;
      const current = here ? " current" : "";
      const aria = here ? ` aria-current="true"` : "";
      return `<span class="lesson-pill"><button type="button" class="node-jump${current}" data-lesson-pack="${escapeHtml(pack.id)}"${aria}>${escapeHtml(pack.title)}</button><button type="button" class="ghost lesson-off" data-lesson-off="${escapeHtml(pack.id)}" aria-label="Take ${escapeHtml(pack.title)} off the lesson set">Take off</button></span>`;
    })
    .join("");
  const options = view.addable
    .map(
      (pack) =>
        `<option value="${escapeHtml(pack.id)}" data-source="${escapeHtml(pack.source)}">${escapeHtml(pack.title)}</option>`,
    )
    .join("");
  const first = view.addable[0];
  const source = first ? `<p class="meta" id="lesson-source">${escapeHtml(first.source)}</p>` : "";
  const chooser = view.addable.length
    ? `<div class="row lesson-add"><label class="field lesson-field">Add a song<select id="lesson-add" aria-label="Songs you can add">${options}</select></label><button type="button" id="lesson-add-btn">Add</button></div>${source}`
    : "";
  const pillRow = pills ? `<div class="rail-pills">${pills}</div>` : "";
  return `<section class="lesson-set" aria-label="Lesson set"><p class="rail-kicker">Lesson set</p>${pillRow}${chooser}</section>`;
}

export function stepNavHtml(steps: readonly StepChoice[], current: number): string {
  if (steps.length < 2) return "";
  const buttons = steps
    .map((step, i) => {
      const here = i === current;
      const mark = here ? ` aria-current="true"` : "";
      const whole = step.whole ? " whole" : "";
      return `<button type="button" class="step-jump${whole}${here ? " current" : ""}" data-step-index="${i}"${mark}>${escapeHtml(step.label)}</button>`;
    })
    .join("");
  return `<nav class="step-rail" aria-label="Inside this part">${buttons}</nav>`;
}

export type AutoAdvance = "inside" | "across" | "highlight" | "off";

export interface SectionPlace {
  step: number;
  stepCount: number;
  hasNextNode: boolean;
  nextIsPage: boolean;
}

export type AdvanceMove =
  | { kind: "stay" }
  | { kind: "step"; index: number; start: boolean }
  | { kind: "node"; start: boolean };

/** Where a settled section goes. A miss stays. A page is not a listen part. */
export function advanceMove(scope: AutoAdvance, place: SectionPlace, settled: boolean): AdvanceMove {
  if (!settled || scope === "off") return { kind: "stay" };
  const nextStep = place.step + 1;
  if (nextStep < place.stepCount) {
    return { kind: "step", index: nextStep, start: scope !== "highlight" };
  }
  if (scope === "across" && place.hasNextNode && !place.nextIsPage) {
    return { kind: "node", start: true };
  }
  return { kind: "stay" };
}

const LISTEN_MODES = new Set([
  "first_sound",
  "staircase",
  "breath_octave",
  "on_the_breath",
  "phrase",
  "ornament",
]);

export interface ReviewCell {
  nodeIndex: number;
  stepIndex: number;
}

/** Last settled listen step in this pack. Pages are not a review. */
export function reviewCell(opts: {
  nodeIds: readonly string[];
  modes: Readonly<Record<string, string>>;
  progress: readonly ProgressMark[];
  stairCount: number;
  phraseCount: number;
}): ReviewCell | null {
  let found: ReviewCell | null = null;
  for (let i = 0; i < opts.nodeIds.length; i++) {
    const id = opts.nodeIds[i];
    const hit = opts.progress.find((mark) => mark.node_id === id);
    if (hit?.state_reached !== "settled") continue;
    const mode = opts.modes[id] ?? id;
    if (!LISTEN_MODES.has(mode)) continue;
    let stepIndex = 0;
    if (mode === "staircase") stepIndex = Math.max(0, opts.stairCount - 1);
    else if (mode === "phrase") stepIndex = Math.max(0, opts.phraseCount - 1);
    found = { nodeIndex: i, stepIndex };
  }
  return found;
}

/** Hear is the primary action until the model has played, when a recording exists. */
export function primaryHears(opts: {
  hasRef: boolean;
  heard: boolean;
  playing: boolean;
  inAttempt: boolean;
}): boolean {
  if (opts.heard || opts.inAttempt) return false;
  if (opts.playing) return true;
  return opts.hasRef;
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
