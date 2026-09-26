export interface GhostPoint {
  t_ms: number;
  cents: number;
}

export interface GhostSpan {
  t0_ms: number;
  t1_ms: number;
  note: string;
  cents: number;
  /** Gesture on this note: cut, tap, roll, short_roll, and the later ones. */
  mark?: string;
}

export interface GhostTrace {
  points: GhostPoint[];
  spans: GhostSpan[];
}

const CAP = 64;

export function showGhost(trace: GhostTrace | null | undefined, evidence: string): boolean {
  if (!trace || evidence === "couldnt_hear" || evidence === "abstain") return false;
  return trace.points.length >= 2 && trace.points.length <= CAP;
}

/** Letter as on the holes. The second octave takes a prime. C stays plain: it is still the low one. */
export function ghostNoteLabel(note: string): string {
  const match = /^([A-G])(#?)(\d)$/.exec(note.trim());
  if (!match) return note;
  const name = match[1] + (match[2] ? "♯" : "");
  if (Number(match[3]) >= 5 && match[1] !== "C") return `${name}′`;
  return name;
}

/**
 * A quiet line under the remark. Each expected note is named.
 * A cut sits near the start of its note, a tap later in it. A roll is both.
 * No cents, and nothing while an attempt is open.
 */
export function ghostMarkup(trace: GhostTrace | null | undefined, evidence: string): string {
  if (!showGhost(trace, evidence) || !trace) return "";
  const points = trace.points;
  const t0 = points[0].t_ms;
  const t1 = points[points.length - 1].t_ms;
  const span = Math.max(1, t1 - t0);
  const x = (t: number) => ((t - t0) / span) * 568 + 36;
  const y = (cents: number) => {
    const c = Math.max(-700, Math.min(700, cents));
    return 52 - (c / 700) * 24;
  };
  const played = points
    .map((p, i) => `${i === 0 ? "M" : "L"}${x(p.t_ms).toFixed(1)} ${y(p.cents).toFixed(1)}`)
    .join(" ");
  const expected = expectedPath(trace.spans, x, y);
  const moments = trace.spans.flatMap((s) => gestureMoments(s));
  const gestureRow = fitted(
    gestureLabels(trace.spans, x),
    "ghost-gesture",
    16,
  );
  const noteRow = fitted(
    trace.spans.map((s) => ({
      x: labelX(x((s.t0_ms + s.t1_ms) / 2)),
      text: ghostNoteLabel(s.note),
    })),
    "ghost-note",
    94,
  );
  const ticks = moments
    .map((m) => {
      const px = x(m.t).toFixed(1);
      const py = y(m.cents);
      const tip = Math.min(py - 1, Math.max(py - 11, 24));
      return `<line class="ghost-tick" x1="${px}" y1="${py.toFixed(1)}" x2="${px}" y2="${tip.toFixed(1)}"/>`;
    })
    .join("");
  return `<svg class="ghost-line" viewBox="0 0 640 104" role="img" aria-label="${esc(describe(trace.spans))}"><path class="ghost-expected" d="${expected}"/>${ticks}<path class="ghost-played" d="${played}"/>${gestureRow}${noteRow}</svg>`;
}

function labelX(px: number): number {
  return Math.min(604, Math.max(36, px));
}

function halfWidth(text: string): number {
  return 3 + text.length * 3.6;
}

function gestureLabels(
  spans: GhostSpan[],
  x: (t: number) => number,
): Array<{ x: number; text: string }> {
  const items: Array<{ x: number; text: string }> = [];
  for (const span of spans) {
    const moments = gestureMoments(span);
    if (moments.length === 0) continue;
    const placed = moments.map((m) => ({ x: labelX(x(m.t)), text: m.word }));
    const crowded =
      placed.length === 2 &&
      Math.abs(placed[0].x - placed[1].x) < halfWidth(placed[0].text) + halfWidth(placed[1].text) + 6;
    if (crowded) {
      items.push({
        x: labelX(x((span.t0_ms + span.t1_ms) / 2)),
        text: placed.map((p) => p.text).join(" "),
      });
    } else {
      items.push(...placed);
    }
  }
  return items;
}

function fitted(items: Array<{ x: number; text: string }>, cls: string, yText: number): string {
  const kept: Array<{ x: number; text: string; half: number }> = [];
  for (const item of items) {
    if (!item.text) continue;
    const half = halfWidth(item.text);
    if (kept.some((p) => Math.abs(p.x - item.x) < p.half + half + 4)) continue;
    kept.push({ ...item, half });
  }
  return kept
    .map(
      (item) =>
        `<text class="${cls}" x="${item.x.toFixed(1)}" y="${yText}" text-anchor="middle">${esc(item.text)}</text>`,
    )
    .join("");
}

function describe(spans: GhostSpan[]): string {
  const notes = spans.map((s) => ghostNoteLabel(s.note)).filter(Boolean);
  const gestures: string[] = [];
  for (const s of spans) {
    const note = ghostNoteLabel(s.note);
    for (const word of gestureWords(s.mark)) gestures.push(`${word} on ${note}`);
  }
  const notePart = notes.length ? ` Notes ${notes.join(" ")}.` : "";
  const gesturePart = gestures.length ? ` ${gestures.join(", ")}.` : "";
  return `The phrase, against what you played.${notePart}${gesturePart}`;
}

/** Where a gesture sits on its note. A cut is early. A tap is later. A roll is both. */
function gestureMoments(span: GhostSpan): Array<{ t: number; word: string; cents: number }> {
  const words = gestureWords(span.mark);
  const dur = span.t1_ms - span.t0_ms;
  if (words.length === 0 || dur <= 0) return [];
  const fracs = gestureFracs((span.mark ?? "").trim(), words.length);
  return words.map((word, i) => ({
    t: span.t0_ms + dur * fracs[i],
    word,
    cents: span.cents,
  }));
}

function gestureFracs(mark: string, count: number): number[] {
  if (count === 2 && mark === "short_roll") return [0.28, 0.62];
  if (count === 2 && mark === "roll") return [0.18, 0.7];
  if (count === 2 && mark === "double_tap") return [0.4, 0.72];
  if (mark === "cut") return [0.18];
  if (mark === "tap") return [0.62];
  if (mark === "slide") return [0.14];
  return Array.from({ length: count }, () => 0.5);
}

function gestureWords(mark: string | undefined): string[] {
  switch ((mark ?? "").trim()) {
    case "":
      return [];
    case "cut":
      return ["cut"];
    case "tap":
      return ["tap"];
    case "roll":
    case "short_roll":
      return ["cut", "tap"];
    case "double_tap":
      return ["tap", "tap"];
    case "cran":
      return ["cran"];
    case "slide":
      return ["slide"];
    case "triplet":
      return ["triplet"];
    default:
      return [(mark ?? "").trim().replaceAll("_", " ")];
  }
}

function esc(s: string): string {
  return s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function expectedPath(
  spans: GhostSpan[],
  x: (t: number) => number,
  y: (cents: number) => number,
): string {
  if (spans.length === 0) return "M36 52 L604 52";
  return spans
    .map((span, i) => {
      const a = `${x(span.t0_ms).toFixed(1)} ${y(span.cents).toFixed(1)}`;
      const b = `${x(span.t1_ms).toFixed(1)} ${y(span.cents).toFixed(1)}`;
      return i === 0 ? `M${a} L${b}` : `M${a} L${b}`;
    })
    .join(" ");
}
