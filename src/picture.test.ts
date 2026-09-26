import assert from "node:assert/strict";
import { test } from "node:test";
import { holeStates, pictureModel, showSolfege, solfegeFor } from "./picture.ts";

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
  assert.equal(solfegeFor("G4"), "Sol");
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
