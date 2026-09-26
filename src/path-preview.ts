import fingeringJson from "../pack/may-morning-dew/fingering-low-d.json";
import manifest from "../pack/may-morning-dew/manifest.json";
import phrases from "../pack/may-morning-dew/phrases.json";
import {
  PATH_NOTE,
  practiceNavHtml,
  resumeIndex,
  stepNavHtml,
  stepsForNode,
  type ProgressMark,
  type StepChoice,
} from "./path.ts";
import { NODE_COPY } from "./types.ts";

const fingering = fingeringJson as {
  notes: Record<string, { label?: string }>;
};

export function mountPathPreview(root: HTMLElement): void {
  const nodeIds = manifest.node_ids;
  const progress: ProgressMark[] = [
    { node_id: "first_sound", state_reached: "settled" },
    { node_id: "staircase", state_reached: "started" },
  ];
  let nodeIndex = resumeIndex(nodeIds, progress);
  let stairIndex = 0;
  let phraseIndex = 0;
  let wantOctave = false;
  const titles = Object.fromEntries(nodeIds.map((id) => [id, NODE_COPY[id]?.title ?? id]));

  const choices = (): StepChoice[] =>
    stepsForNode({
      nodeId: nodeIds[nodeIndex] ?? "",
      staircaseNotes: phrases.staircase_notes,
      noteLabel: (note) => fingering.notes[note]?.label ?? note,
      chunkLabels: phrases.chunks.map((chunk) => chunk.label),
    });

  const stepIndex = (): number => {
    const id = nodeIds[nodeIndex];
    if (id === "staircase") return stairIndex;
    if (id === "breath_octave") return wantOctave ? 1 : 0;
    if (id === "air_bare" || id === "air_may_morning_dew") return phraseIndex;
    return 0;
  };

  const draw = () => {
    const id = nodeIds[nodeIndex] ?? "first_sound";
    const copy = NODE_COPY[id] ?? { title: id, body: "" };
    root.innerHTML = `
      <div class="app-shell">
        <p class="eyebrow">Path preview</p>
        ${practiceNavHtml({ nodeIds, titles, current: nodeIndex, progress })}
        <p class="meta path-note">${PATH_NOTE}</p>
        <h1>${copy.title}</h1>
        <p class="lede">${copy.body}</p>
        <div class="card">${stepNavHtml(choices(), stepIndex())}</div>
      </div>`;
    root.querySelectorAll<HTMLButtonElement>("[data-node-index]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const index = Number(btn.dataset.nodeIndex);
        if (!Number.isInteger(index) || index === nodeIndex) return;
        nodeIndex = index;
        stairIndex = 0;
        phraseIndex = 0;
        wantOctave = false;
        draw();
      });
    });
    root.querySelectorAll<HTMLButtonElement>("[data-step-index]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const index = Number(btn.dataset.stepIndex);
        const id = nodeIds[nodeIndex];
        if (!Number.isInteger(index) || index === stepIndex()) return;
        if (id === "staircase") stairIndex = index;
        else if (id === "breath_octave") wantOctave = index === 1;
        else phraseIndex = index;
        draw();
      });
    });
  };

  draw();
}
