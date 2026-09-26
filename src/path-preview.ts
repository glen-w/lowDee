import fingeringJson from "../pack/may-morning-dew/fingering-low-d.json";
import manifest from "../pack/may-morning-dew/manifest.json";
import phrases from "../pack/may-morning-dew/phrases.json";
import {
  PATH_NOTE,
  lessonSetHtml,
  menuKindFor,
  practiceNavHtml,
  resumeIndex,
  stepNavHtml,
  stepsForNode,
  type LessonChoice,
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
  const kinds = Object.fromEntries(
    nodeIds.map((id) => {
      const node = manifest.nodes.find((item) => item.id === id);
      return [
        id,
        menuKindFor({
          id,
          mode: node?.mode,
          title: titles[id] ?? id,
          packTitle: manifest.title,
        }),
      ];
    }),
  );

  const lessonPacks: LessonChoice[] = [
    {
      id: "may-morning-dew",
      title: "The May Morning Dew",
      source: "Teaching setting in D, written for this pack.",
      song: true,
      playable: true,
      open: true,
      rights: "pd",
    },
    {
      id: "salley-gardens",
      title: "Down by the Salley Gardens",
      source: "Traditional air The Mourne Shore, Francis O’Neill.",
      song: true,
      playable: true,
      open: false,
      rights: "pd",
    },
    {
      id: "book-staircase",
      title: "The book’s notes",
      source: "Getting Started.",
      song: false,
      playable: true,
      open: true,
      rights: "pd",
    },
  ];
  let savedLessons: string[] = [];

  const choices = (): StepChoice[] => {
    const id = nodeIds[nodeIndex] ?? "";
    const node = manifest.nodes.find((item) => item.id === id);
    const title = titles[id] ?? id;
    return stepsForNode({
      nodeId: id,
      staircaseNotes: phrases.staircase_notes,
      noteLabel: (note) => fingering.notes[note]?.label ?? note,
      chunkLabels: phrases.chunks.map((chunk) => chunk.label),
      song:
        menuKindFor({
          id,
          mode: node?.mode,
          title,
          packTitle: manifest.title,
        }) === "song",
    });
  };

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
        ${practiceNavHtml({ nodeIds, titles, kinds, current: nodeIndex, progress })}
        ${lessonSetHtml({ packs: lessonPacks, savedIds: savedLessons, currentId: "may-morning-dew" })}
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
    document.querySelector("#lesson-add")?.addEventListener("change", (event) => {
      const select = event.target as HTMLSelectElement;
      const source = select.selectedOptions[0]?.dataset.source ?? "";
      const line = document.querySelector("#lesson-source");
      if (line) line.textContent = source;
    });
    document.querySelector("#lesson-add-btn")?.addEventListener("click", () => {
      const select = document.querySelector("#lesson-add") as HTMLSelectElement | null;
      const id = select?.value;
      if (!id || savedLessons.includes(id)) return;
      savedLessons = [...savedLessons, id];
      draw();
    });
    root.querySelectorAll<HTMLButtonElement>("[data-lesson-off]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const id = btn.dataset.lessonOff;
        if (!id) return;
        savedLessons = savedLessons.filter((pack) => pack !== id);
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
