import assert from "node:assert/strict";
import { test } from "node:test";
import { ghostMarkup, showGhost, type GhostTrace } from "./ghost.ts";

const trace: GhostTrace = {
  points: [
    { t_ms: 0, cents: 0 },
    { t_ms: 200, cents: 180 },
    { t_ms: 400, cents: -20 },
  ],
  spans: [
    { t0_ms: 0, t1_ms: 200, note: "D4", cents: 0 },
    { t0_ms: 200, t1_ms: 400, note: "E4", cents: 200 },
  ],
};

test("a finished phrase draws a line and does not print cents", () => {
  const html = ghostMarkup(trace, "phrase_ok");
  assert.match(html, /ghost-played/);
  assert.match(html, /ghost-expected/);
  assert.doesNotMatch(html, /180/);
  assert.doesNotMatch(html, /cent/i);
});

test("silence and a missed take draw nothing", () => {
  assert.equal(ghostMarkup(trace, "couldnt_hear"), "");
  assert.equal(ghostMarkup(trace, "abstain"), "");
  assert.equal(ghostMarkup(null, "sealed"), "");
  assert.equal(showGhost({ points: [], spans: [] }, "sealed"), false);
});
