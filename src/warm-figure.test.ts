import assert from "node:assert/strict";
import { test } from "node:test";
import { exerciseIds, WARMUP } from "./warmup.ts";
import { figureId, warmFigureMarkup, warmIntroFigure } from "./warm-figure.ts";

function mirrors(svg: string): number {
  return svg.split("scale(-1 1)").length - 1;
}

test("every exercise has a figure", () => {
  for (const id of exerciseIds()) {
    const beat = WARMUP.find((b) => b.exercise === id);
    assert.ok(beat, id);
    const svg = warmFigureMarkup(beat);
    assert.match(svg, /^<svg/);
    assert.match(svg, /role="img"/);
    assert.match(svg, /<title id="[^"]+">[^<]+<\/title>/);
  }
  assert.match(warmIntroFigure(), /Open hands/);
});

test("the picture follows the pose, not the count", () => {
  const up = WARMUP.find((b) => b.exercise === "rise" && b.mark === "Up")!;
  const down = WARMUP.find((b) => b.exercise === "rise" && b.mark === "Down")!;
  assert.notEqual(warmFigureMarkup(up), warmFigureMarkup(down));

  const inn = WARMUP.find((b) => b.mark === "In 1")!;
  const innLater = WARMUP.find((b) => b.mark === "In 4")!;
  const out = WARMUP.find((b) => b.mark === "Out 1")!;
  assert.equal(figureId(inn), figureId(innLater));
  assert.notEqual(figureId(inn), figureId(out));
  assert.notEqual(warmFigureMarkup(inn), warmFigureMarkup(out));
});

test("each finger is its own picture", () => {
  for (const exercise of ["fingers", "lifts"]) {
    const marks = [...new Set(WARMUP.filter((b) => b.exercise === exercise).map((b) => b.mark))];
    const svgs = marks.map(
      (mark) => warmFigureMarkup(WARMUP.find((b) => b.exercise === exercise && b.mark === mark)!),
    );
    assert.equal(new Set(svgs).size, marks.length, exercise);
  }
});

test("a left-hand step mirrors the right-hand drawing", () => {
  const right = warmFigureMarkup(
    WARMUP.find((b) => b.exercise === "thumb" && b.line.startsWith("Right"))!,
  );
  const left = warmFigureMarkup(
    WARMUP.find((b) => b.exercise === "thumb" && b.line.startsWith("Left"))!,
  );
  assert.ok(mirrors(left) > mirrors(right));
  assert.match(left, /Left hand/);
  assert.match(right, /Right hand/);
  const shake = warmFigureMarkup(WARMUP.find((b) => b.exercise === "shake")!);
  assert.equal(mirrors(shake), 0);
});
