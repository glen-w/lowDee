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

test("a finished phrase names the notes and does not print cents", () => {
  const html = ghostMarkup(trace, "phrase_ok");
  assert.match(html, /ghost-played/);
  assert.match(html, /ghost-expected/);
  assert.match(html, /class="ghost-note"[^>]*>D</);
  assert.match(html, /class="ghost-note"[^>]*>E</);
  assert.doesNotMatch(html, /ghost-gesture/);
  assert.doesNotMatch(html, /D4|E4/);
  assert.doesNotMatch(html, /180/);
  assert.doesNotMatch(html, /cent/i);
});

test("a cut and a tap are marked on the notes they belong to", () => {
  const marked: GhostTrace = {
    points: [
      { t_ms: 0, cents: 0 },
      { t_ms: 500, cents: 40 },
      { t_ms: 1000, cents: 680 },
    ],
    spans: [
      { t0_ms: 0, t1_ms: 500, note: "D4", cents: 0, mark: "cut" },
      { t0_ms: 500, t1_ms: 1000, note: "A4", cents: 700, mark: "tap" },
    ],
  };
  const html = ghostMarkup(marked, "phrase_ok");
  assert.match(html, /class="ghost-gesture"[^>]*>cut</);
  assert.match(html, /class="ghost-gesture"[^>]*>tap</);
  assert.match(html, /ghost-tick/);
  assert.match(html, /cut on D, tap on A/);
});

test("a roll marks the cut and the tap, and the octave takes a prime", () => {
  const marked: GhostTrace = {
    points: [
      { t_ms: 0, cents: 700 },
      { t_ms: 800, cents: 1100 },
    ],
    spans: [
      { t0_ms: 0, t1_ms: 400, note: "A4", cents: 700, mark: "roll" },
      { t0_ms: 400, t1_ms: 800, note: "D5", cents: 1200, mark: "" },
    ],
  };
  const html = ghostMarkup(marked, "phrase_ok");
  assert.match(html, />cut</);
  assert.match(html, />tap</);
  assert.match(html, />A</);
  assert.match(html, />D′</);
  assert.doesNotMatch(html, /D5/);
});

test("silence and a missed take draw nothing", () => {
  assert.equal(ghostMarkup(trace, "couldnt_hear"), "");
  assert.equal(ghostMarkup(trace, "abstain"), "");
  assert.equal(ghostMarkup(null, "sealed"), "");
  assert.equal(showGhost({ points: [], spans: [] }, "sealed"), false);
});
