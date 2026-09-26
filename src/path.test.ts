import assert from "node:assert/strict";
import { test } from "node:test";
import {
  practiceNavHtml,
  resumeIndex,
  stepNavHtml,
  stepsForNode,
  type ProgressMark,
} from "./path.ts";

const ids = [
  "first_sound",
  "staircase",
  "breath_octave",
  "hedwig",
  "on_the_breath",
  "air_bare",
  "orn_cut",
  "orn_tap",
  "orn_roll",
  "air_may_morning_dew",
];

const titles = Object.fromEntries(ids.map((id) => [id, id]));

test("a new sitting opens on the first unsettled part", () => {
  const progress: ProgressMark[] = [
    { node_id: "first_sound", state_reached: "settled" },
    { node_id: "staircase", state_reached: "started" },
    { node_id: "orn_cut", state_reached: "settled" },
  ];
  assert.equal(resumeIndex(ids, progress), 1);
  assert.equal(resumeIndex(ids, []), 0);
  assert.equal(
    resumeIndex(
      ids,
      ids.map((node_id) => ({ node_id, state_reached: "settled" })),
    ),
    ids.length - 1,
  );
});

test("every part stays open when earlier parts are unsettled", () => {
  const html = practiceNavHtml({
    nodeIds: ids,
    titles,
    current: 0,
    progress: [],
  });
  assert.equal(html.includes("disabled"), false);
  for (const id of ids) assert.match(html, new RegExp(`aria-label="${id}, not yet"`));
  assert.match(html, /data-node-index="6"/);
  assert.match(html, /data-node-index="9"/);
});

test("settled and started are marks on the button, and the current part stays pressable", () => {
  const html = practiceNavHtml({
    nodeIds: ids,
    titles,
    current: 6,
    progress: [
      { node_id: "first_sound", state_reached: "settled" },
      { node_id: "staircase", state_reached: "started" },
    ],
  });
  assert.match(html, /class="node-jump done"[^>]*aria-label="first_sound, settled"/);
  assert.match(html, /class="node-jump started"[^>]*aria-label="staircase, started"/);
  assert.match(html, /class="node-jump current"[^>]*data-node-index="6"/);
  assert.equal(html.includes("disabled"), false);
});

test("staircase notes, the octave, and air phrases can each be opened", () => {
  const notes = stepsForNode({
    nodeId: "staircase",
    staircaseNotes: ["D4", "B4"],
    noteLabel: (n) => n,
    chunkLabels: [],
  });
  assert.deepEqual(notes.map((s) => s.label), ["D4", "B4"]);
  const stair = stepNavHtml(notes, 1);
  assert.match(stair, /data-step-index="1"[^>]*aria-current="true"/);
  assert.equal(stair.includes("disabled"), false);

  const octave = stepsForNode({
    nodeId: "breath_octave",
    staircaseNotes: [],
    noteLabel: (n) => n,
    chunkLabels: [],
  });
  assert.deepEqual(octave.map((s) => s.label), ["Low D", "Octave"]);

  const air = stepNavHtml(
    stepsForNode({
      nodeId: "air_bare",
      staircaseNotes: [],
      noteLabel: (n) => n,
      chunkLabels: ["Opening", "Close"],
    }),
    0,
  );
  assert.match(air, />Close</);
  assert.equal(stepsForNode({
    nodeId: "hedwig",
    staircaseNotes: [],
    noteLabel: (n) => n,
    chunkLabels: ["Opening"],
  }).length, 0);
  assert.equal(stepNavHtml([{ label: "Only" }], 0), "");
});
