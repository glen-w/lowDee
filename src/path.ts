export interface ProgressMark {
  node_id: string;
  state_reached: string;
}

export interface StepChoice {
  label: string;
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
    return opts.chunkLabels.map((label) => ({ label }));
  }
  return [];
}

export function practiceNavHtml(opts: {
  nodeIds: readonly string[];
  titles: Readonly<Record<string, string>>;
  current: number;
  progress: readonly ProgressMark[];
}): string {
  const buttons = opts.nodeIds
    .map((id, i) => {
      const status = nodeStatus(opts.progress, id);
      const current = i === opts.current;
      const mark = status === "settled" ? "done" : status === "started" ? "started" : "";
      const cls = ["node-jump", current ? "current" : "", mark].filter(Boolean).join(" ");
      const title = opts.titles[id] ?? id;
      const word = status === "open" ? "not yet" : status;
      const here = current ? ` aria-current="true"` : "";
      return `<button type="button" class="${cls}" data-node-index="${i}"${here} aria-label="${escapeHtml(title)}, ${word}">${escapeHtml(title)}</button>`;
    })
    .join("");
  return `<nav class="node-rail" aria-label="Practice parts">${buttons}</nav>`;
}

export function stepNavHtml(steps: readonly StepChoice[], current: number): string {
  if (steps.length < 2) return "";
  const buttons = steps
    .map((step, i) => {
      const here = i === current;
      const mark = here ? ` aria-current="true"` : "";
      return `<button type="button" class="step-jump${here ? " current" : ""}" data-step-index="${i}"${mark}>${escapeHtml(step.label)}</button>`;
    })
    .join("");
  return `<nav class="step-rail" aria-label="Inside this part">${buttons}</nav>`;
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
