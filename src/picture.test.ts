import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { gestureHolesFor, holeStates, pictureModel, showSolfege, solfegeFor } from "./picture.ts";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

const fingering = {
  notes: {
    C5: {
      holes: ["open", "closed", "closed", "open", "open", "half"],
      octave: 1,
      label: "C",
      half: true,
    },
    D4: {
      holes: ["closed", "closed", "closed", "closed", "closed", "closed"],
      octave: 1,
      label: "D",
    },
    D5: {
      holes: ["closed", "closed", "closed", "closed", "closed", "closed"],
      octave: 2,
      label: "D′",
    },
    E4: {
      holes: ["closed", "closed", "closed", "closed", "closed", "open"],
      octave: 1,
      label: "E",
    },
  },
};

test("half-fill stays on the one hole that says half", () => {
  const holes = holeStates(fingering.notes.C5);
  assert.equal(holes.filter((h) => h === "half").length, 1);
  assert.equal(holes[5], "half");
  assert.equal(holes[0], "open");
});

test("a phrase is one column per note and the octave is not a second chart", () => {
  const model = pictureModel({
    fingering,
    notes: ["D4", "E4", "D5"],
    currentIndex: 2,
    showSolfege: false,
  });
  assert.equal(model.columns.length, 3);
  assert.equal(model.columns[2].holes.length, 6);
  assert.equal(model.columns[2].octave, 2);
  assert.deepEqual(model.columns[2].holes, model.columns[0].holes);
  assert.equal(model.whistle.label, "D′");
  assert.equal(model.solfege, null);
  assert.equal(model.columns.filter((c) => c.current).length, 1);
});

test("solfege is a reader layer", () => {
  assert.equal(showSolfege("no"), false);
  assert.equal(showSolfege("some"), true);
  const model = pictureModel({
    fingering,
    notes: ["D4", "E4", "D5"],
    currentIndex: 0,
    showSolfege: true,
  });
  assert.deepEqual(model.solfege, ["Re", "Mi", "Re′"]);
  assert.equal(solfegeFor("F#4"), "Fa♯");
  assert.equal(solfegeFor("G4"), "So");
  assert.equal(solfegeFor("B4"), "Ti");
  assert.equal(solfegeFor("C5"), "Do");
});

const scale = {
  notes: {
    D4: {
      holes: ["closed", "closed", "closed", "closed", "closed", "closed"],
      octave: 1,
      label: "D",
    },
    E4: {
      holes: ["closed", "closed", "closed", "closed", "closed", "open"],
      octave: 1,
      label: "E",
    },
    G4: {
      holes: ["closed", "closed", "closed", "open", "open", "open"],
      octave: 1,
      label: "G",
    },
    A4: {
      holes: ["closed", "closed", "open", "open", "open", "open"],
      octave: 1,
      label: "A",
    },
    B4: {
      holes: ["closed", "open", "open", "open", "open", "open"],
      octave: 1,
      label: "B",
    },
    D5: {
      holes: ["closed", "closed", "closed", "closed", "closed", "closed"],
      octave: 2,
      label: "D′",
    },
  },
};

test("a cut on A is hole 2 and a tap on A is hole 3", () => {
  const cut = pictureModel({
    fingering: scale,
    notes: ["A4"],
    currentIndex: 0,
    showSolfege: true,
    marks: ["cut"],
  });
  assert.deepEqual(cut.whistle.gestures, [{ hole: 1, kind: "cut" }]);
  assert.equal(cut.solfege?.[0], "La");
  const tap = gestureHolesFor(scale, "A4", "tap");
  assert.deepEqual(tap, [{ hole: 2, kind: "tap" }]);
});

test("a cut on low D is hole 6, and low D has no tap", () => {
  assert.deepEqual(gestureHolesFor(scale, "D4", "cut"), [{ hole: 5, kind: "cut" }]);
  assert.deepEqual(gestureHolesFor(scale, "D4", "tap"), []);
});

test("a roll marks both holes and a slide does not guess a hole", () => {
  assert.deepEqual(gestureHolesFor(scale, "A4", "roll"), [
    { hole: 1, kind: "cut" },
    { hole: 2, kind: "tap" },
  ]);
  assert.deepEqual(gestureHolesFor(scale, "A4", "slide"), []);
  assert.deepEqual(gestureHolesFor(fingering, "C5", "cut"), []);
});

test("an octave D uses the same cut as the low one when the chart stops", () => {
  assert.deepEqual(gestureHolesFor(scale, "D5", "cut"), [{ hole: 5, kind: "cut" }]);
});

test("the door's four marks name the moving hole", () => {
  const fingering = JSON.parse(
    readFileSync(join(root, "pack/may-morning-dew/fingering-low-d.json"), "utf8"),
  );
  const phrases = JSON.parse(readFileSync(join(root, "pack/may-morning-dew/phrases.json"), "utf8"));
  const ornaments = JSON.parse(
    readFileSync(join(root, "pack/may-morning-dew/ornaments.json"), "utf8"),
  );
  const seen = ornaments.marks.map(
    (mark: { chunk_id: string; note_index: number; gesture: string }) => {
      const chunk = phrases.chunks.find((item: { id: string }) => item.id === mark.chunk_id);
      const note = chunk.notes[mark.note_index];
      return {
        note,
        gesture: mark.gesture,
        holes: gestureHolesFor(fingering, note, mark.gesture).map((item) => item.hole + 1),
      };
    },
  );
  assert.deepEqual(seen, [
    { note: "D4", gesture: "cut", holes: [6] },
    { note: "A4", gesture: "tap", holes: [3] },
    { note: "A4", gesture: "roll", holes: [2, 3] },
    { note: "D4", gesture: "cut", holes: [6] },
  ]);
});

test("a leak mark is a hole index, not a second row", () => {
  const model = pictureModel({
    fingering,
    notes: ["E4"],
    currentIndex: 0,
    leakHole: 5,
    showSolfege: false,
  });
  assert.equal(model.whistle.leakHole, 5);
  assert.equal(model.whistle.holes.length, 6);
  const ignored = pictureModel({
    fingering,
    notes: ["E4"],
    currentIndex: 0,
    leakHole: 9,
    showSolfege: false,
  });
  assert.equal(ignored.whistle.leakHole, null);
});
