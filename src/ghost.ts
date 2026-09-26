export interface GhostPoint {
  t_ms: number;
  cents: number;
}

export interface GhostSpan {
  t0_ms: number;
  t1_ms: number;
  note: string;
  cents: number;
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

/** A quiet line under the remark. No cents, no marks, nothing while an attempt is open. */
export function ghostMarkup(trace: GhostTrace | null | undefined, evidence: string): string {
  if (!showGhost(trace, evidence) || !trace) return "";
  const points = trace.points;
  const t0 = points[0].t_ms;
  const t1 = points[points.length - 1].t_ms;
  const span = Math.max(1, t1 - t0);
  const x = (t: number) => ((t - t0) / span) * 300 + 10;
  const y = (cents: number) => {
    const c = Math.max(-700, Math.min(700, cents));
    return 36 - (c / 700) * 28;
  };
  const played = points
    .map((p, i) => `${i === 0 ? "M" : "L"}${x(p.t_ms).toFixed(1)} ${y(p.cents).toFixed(1)}`)
    .join(" ");
  const expected = expectedPath(trace.spans, x, y);
  return `<svg class="ghost-line" viewBox="0 0 320 72" role="img" aria-label="The phrase, against what you played"><path class="ghost-expected" d="${expected}"/><path class="ghost-played" d="${played}"/></svg>`;
}

function expectedPath(
  spans: GhostSpan[],
  x: (t: number) => number,
  y: (cents: number) => number,
): string {
  if (spans.length === 0) return "M10 36 L310 36";
  return spans
    .map((span, i) => {
      const a = `${x(span.t0_ms).toFixed(1)} ${y(span.cents).toFixed(1)}`;
      const b = `${x(span.t1_ms).toFixed(1)} ${y(span.cents).toFixed(1)}`;
      return i === 0 ? `M${a} L${b}` : `M${a} L${b}`;
    })
    .join(" ");
}
