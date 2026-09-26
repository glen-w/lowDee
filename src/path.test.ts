import assert from "node:assert/strict";
import { test } from "node:test";
import {
  advanceMove,
  joinedSong,
  lessonSetHtml,
  lessonSetView,
  menuKindFor,
  practiceNavHtml,
  primaryHears,
  hornHandoff,
  resumeIndex,
  reviewCell,
  stepNavHtml,
  stepsForNode,
  type LessonChoice,
  type ProgressMark,
  type SectionPlace,
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

test("a new whistle warms up at its first unsettled part", () => {
  const fresh = hornHandoff({
    fresh: true,
    warmedThisSitting: true,
    nodeIds: ids,
    progress: [],
  });
  assert.equal(fresh.showWarmup, true);
  assert.equal(fresh.nodeIndex, 0);

  const mid = hornHandoff({
    fresh: true,
    warmedThisSitting: true,
    nodeIds: ids,
    progress: [
      { node_id: "first_sound", state_reached: "settled" },
      { node_id: "staircase", state_reached: "started" },
    ],
  });
  assert.equal(mid.showWarmup, true);
  assert.equal(mid.nodeIndex, 1);

  const kept = hornHandoff({
    fresh: false,
    warmedThisSitting: true,
    nodeIds: ids,
    progress: [{ node_id: "first_sound", state_reached: "settled" }],
  });
  assert.equal(kept.showWarmup, false);
  assert.equal(kept.nodeIndex, 1);

  const cold = hornHandoff({
    fresh: false,
    warmedThisSitting: false,
    nodeIds: ids,
    progress: [{ node_id: "first_sound", state_reached: "settled" }],
  });
  assert.equal(cold.showWarmup, true);
  assert.equal(cold.nodeIndex, 1);
});

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

test("a settled section walks only as far as the setting says", () => {
  const mid: SectionPlace = { step: 0, stepCount: 4, hasNextNode: true, nextIsPage: false };
  const last: SectionPlace = { step: 3, stepCount: 4, hasNextNode: true, nextIsPage: false };
  const alone: SectionPlace = { step: 0, stepCount: 0, hasNextNode: true, nextIsPage: false };
  assert.deepEqual(advanceMove("inside", mid, false), { kind: "stay" });
  assert.deepEqual(advanceMove("off", mid, true), { kind: "stay" });
  assert.deepEqual(advanceMove("inside", mid, true), { kind: "step", index: 1, start: true });
  assert.deepEqual(advanceMove("highlight", mid, true), { kind: "step", index: 1, start: false });
  assert.deepEqual(advanceMove("across", mid, true), { kind: "step", index: 1, start: true });
  assert.deepEqual(advanceMove("inside", last, true), { kind: "stay" });
  assert.deepEqual(advanceMove("highlight", last, true), { kind: "stay" });
  assert.deepEqual(advanceMove("across", last, true), { kind: "node", start: true });
  assert.deepEqual(advanceMove("across", { ...last, nextIsPage: true }, true), { kind: "stay" });
  assert.deepEqual(advanceMove("across", { ...last, hasNextNode: false }, true), { kind: "stay" });
  assert.deepEqual(advanceMove("inside", alone, true), { kind: "stay" });
  assert.deepEqual(advanceMove("across", alone, true), { kind: "node", start: true });
});

test("a song with more than one line ends on the whole song", () => {
  const song = stepsForNode({
    nodeId: "lines",
    mode: "phrase",
    staircaseNotes: [],
    noteLabel: (n) => n,
    chunkLabels: ["Gardens", "We met"],
    song: true,
  });
  assert.deepEqual(song.map((step) => step.label), ["Gardens", "We met", "Whole song"]);
  assert.equal(song.at(-1)?.whole, true);
  const one = stepsForNode({
    nodeId: "lines",
    mode: "phrase",
    staircaseNotes: [],
    noteLabel: (n) => n,
    chunkLabels: ["Opening"],
    song: true,
  });
  assert.deepEqual(one.map((step) => step.label), ["Opening"]);
  const lesson = stepsForNode({
    nodeId: "cnat_phrase",
    mode: "phrase",
    staircaseNotes: [],
    noteLabel: (n) => n,
    chunkLabels: ["In the line", "Again"],
    song: false,
  });
  assert.deepEqual(lesson.map((step) => step.label), ["In the line", "Again"]);
});

test("the whole song joins the lines, and a breath between them stays allowed", () => {
  const open = joinedSong(
    [
      { id: "a", notes: ["D4", "E4"], abc: "D2 E", breaths: [] },
      { id: "b", notes: ["F#4"], abc: "F2", breaths: [0] },
    ],
    [{ chunk_id: "b", note_index: 0, gesture: "cut" }],
  );
  assert.deepEqual(open.notes, ["D4", "E4", "F#4"]);
  assert.equal(open.abc, "D2 E F2");
  assert.deepEqual(open.breaths, []);
  assert.deepEqual(open.marks, [{ note_index: 2, gesture: "cut" }]);

  const closed = joinedSong([
    { id: "a", notes: ["D4", "E4"], abc: "D2 E", breaths: [0] },
    { id: "b", notes: ["F#4", "G4"], abc: "F2 G", breaths: [1] },
  ]);
  assert.deepEqual(closed.breaths, [0, 1, 3]);
});

const modes: Record<string, string> = {
  first_sound: "first_sound",
  staircase: "staircase",
  breath_octave: "breath_octave",
  hedwig: "page",
  on_the_breath: "on_the_breath",
  air_bare: "phrase",
  orn_cut: "ornament",
  air_may_morning_dew: "phrase",
};

test("a sitting reviews the last settled listen step", () => {
  assert.equal(
    reviewCell({
      nodeIds: ids,
      modes,
      progress: [],
      stairCount: 6,
      phraseCount: 4,
    }),
    null,
  );
  assert.equal(
    reviewCell({
      nodeIds: ids,
      modes,
      progress: [{ node_id: "hedwig", state_reached: "settled" }],
      stairCount: 6,
      phraseCount: 4,
    }),
    null,
  );
  const hold = reviewCell({
    nodeIds: ids,
    modes,
    progress: [{ node_id: "first_sound", state_reached: "settled" }],
    stairCount: 6,
    phraseCount: 4,
  });
  assert.deepEqual(hold, { nodeIndex: 0, stepIndex: 0 });
  const stair = reviewCell({
    nodeIds: ids,
    modes,
    progress: [
      { node_id: "first_sound", state_reached: "settled" },
      { node_id: "staircase", state_reached: "settled" },
      { node_id: "hedwig", state_reached: "settled" },
    ],
    stairCount: 6,
    phraseCount: 4,
  });
  assert.deepEqual(stair, { nodeIndex: 1, stepIndex: 5 });
  const air = reviewCell({
    nodeIds: ids,
    modes,
    progress: [
      { node_id: "first_sound", state_reached: "settled" },
      { node_id: "air_bare", state_reached: "settled" },
    ],
    stairCount: 6,
    phraseCount: 4,
  });
  assert.deepEqual(air, { nodeIndex: 5, stepIndex: 3 });
});

test("a phrase titled as the pack is a song, and a drill phrase stays a lesson", () => {
  assert.equal(
    menuKindFor({ id: "hedwig", mode: "page", title: "Hedwig's Theme", packTitle: "The May Morning Dew" }),
    "song",
  );
  assert.equal(
    menuKindFor({ id: "air_bare", mode: "phrase", title: "The air, bare", packTitle: "The May Morning Dew" }),
    "song",
  );
  assert.equal(
    menuKindFor({
      id: "lines",
      mode: "phrase",
      title: "Down by the Salley Gardens",
      packTitle: "Down by the Salley Gardens",
    }),
    "song",
  );
  assert.equal(
    menuKindFor({ id: "cnat_phrase", mode: "phrase", title: "C inside a phrase", packTitle: "C natural" }),
    "lesson",
  );
  assert.equal(
    menuKindFor({ id: "orn_cut", mode: "ornament", title: "Cut", packTitle: "The May Morning Dew" }),
    "lesson",
  );
});

test("lessons and songs sit in separate groups, still in path order", () => {
  const kinds = Object.fromEntries(ids.map((id) => [id, id.startsWith("air_") || id === "hedwig" ? "song" as const : "lesson" as const]));
  const html = practiceNavHtml({
    nodeIds: ids,
    titles,
    current: 3,
    progress: [],
    kinds,
  });
  const lessons = html.indexOf(">Lessons<");
  const songs = html.indexOf(">Songs<");
  assert.ok(lessons >= 0 && songs > lessons);
  const lessonRow = html.slice(lessons, songs);
  const songRow = html.slice(songs);
  assert.ok(lessonRow.indexOf("first_sound") < lessonRow.indexOf("on_the_breath"));
  assert.ok(lessonRow.indexOf("on_the_breath") < lessonRow.indexOf("orn_cut"));
  assert.ok(songRow.indexOf("hedwig") < songRow.indexOf("air_bare"));
  assert.ok(songRow.indexOf("air_bare") < songRow.indexOf("air_may_morning_dew"));
  assert.match(html, /data-node-index="3"[^>]*aria-current="true"|aria-current="true"[^>]*data-node-index="3"/);
  assert.equal(html.includes(">hedwig<") && lessonRow.includes("hedwig"), false);
});

test("a pack of only songs stays one row", () => {
  const html = practiceNavHtml({
    nodeIds: ["lines"],
    titles: { lines: "Skye Boat Song" },
    current: 0,
    progress: [],
    kinds: { lines: "song" },
  });
  assert.equal(html.includes("rail-group"), false);
  assert.match(html, />Skye Boat Song</);
});

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
    source: "Traditional air The Mourne Shore.",
    song: true,
    playable: true,
    open: false,
    rights: "pd",
  },
  {
    id: "amazing-grace",
    title: "Amazing Grace",
    source: "Traditional, public domain.",
    song: true,
    playable: true,
    open: true,
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
  {
    id: "lonesome-boatman",
    title: "The Lonesome Boatman",
    source: "Finbar Furey. In the book. No notes and no audio here.",
    song: false,
    playable: false,
    open: false,
    rights: "page_only",
  },
];

test("the lesson dropdown is open public-domain songs, and a saved one is a pill", () => {
  const empty = lessonSetView(lessonPacks, []);
  assert.deepEqual(
    empty.addable.map((pack) => pack.id),
    ["may-morning-dew", "amazing-grace"],
  );
  assert.deepEqual(empty.saved, []);

  const saved = lessonSetView(lessonPacks, ["amazing-grace", "salley-gardens", "lonesome-boatman", "amazing-grace"]);
  assert.deepEqual(
    saved.saved.map((pack) => pack.id),
    ["amazing-grace"],
  );
  assert.deepEqual(
    saved.addable.map((pack) => pack.id),
    ["may-morning-dew"],
  );

  const html = lessonSetHtml({
    packs: lessonPacks,
    savedIds: ["amazing-grace"],
    currentId: "may-morning-dew",
  });
  assert.match(html, /data-lesson-pack="amazing-grace"/);
  assert.equal(html.includes("salley-gardens"), false);
  assert.equal(html.includes("book-staircase"), false);
  assert.equal(html.includes("lonesome-boatman"), false);
  assert.equal(html.includes('value="amazing-grace"'), false);
  assert.match(html, /value="may-morning-dew"/);
  assert.match(html, /Teaching setting in D/);

  const ordered = lessonSetHtml({
    packs: lessonPacks,
    savedIds: ["amazing-grace", "may-morning-dew"],
    currentId: "may-morning-dew",
  });
  const grace = ordered.indexOf('data-lesson-pack="amazing-grace"');
  const dew = ordered.indexOf('data-lesson-pack="may-morning-dew"');
  assert.ok(grace >= 0 && dew > grace);
  assert.match(ordered, /data-lesson-pack="may-morning-dew"[^>]*aria-current="true"/);
  assert.equal(ordered.includes("<select"), false);
});

test("a blank source or a licence that is not public domain stays out of the lesson set", () => {
  const view = lessonSetView(
    [
      {
        id: "unnamed",
        title: "Unnamed",
        source: "  ",
        song: true,
        playable: true,
        open: true,
        rights: "pd",
      },
      {
        id: "brought",
        title: "Brought",
        source: "A file on the desk.",
        song: true,
        playable: true,
        open: true,
        rights: "brought",
      },
    ],
    ["unnamed", "brought"],
  );
  assert.deepEqual(view.saved, []);
  assert.deepEqual(view.addable, []);
});

test("a lesson set with nothing open and nothing saved draws nothing", () => {
  const html = lessonSetHtml({
    packs: lessonPacks.filter((pack) => !pack.open),
    savedIds: [],
    currentId: "may-morning-dew",
  });
  assert.equal(html, "");
});

test("Hear is the primary action until the model has played", () => {
  assert.equal(primaryHears({ hasRef: true, heard: false, playing: false, inAttempt: false }), true);
  assert.equal(primaryHears({ hasRef: false, heard: false, playing: false, inAttempt: false }), false);
  assert.equal(primaryHears({ hasRef: true, heard: true, playing: false, inAttempt: false }), false);
  assert.equal(primaryHears({ hasRef: true, heard: false, playing: false, inAttempt: true }), false);
  assert.equal(primaryHears({ hasRef: true, heard: false, playing: true, inAttempt: false }), true);
});
