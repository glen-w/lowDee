import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { noteHz, ornamentEvents, phraseEvents, songToneEvents } from "./tones.ts";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

const D4 = 293.66;

test("the opening keeps the pack's notes and the ABC lengths", () => {
  const events = phraseEvents("D2 E | F2 G | A3", ["D4", "E4", "F#4", "G4", "A4"], D4);
  assert.equal(events.length, 5);
  assert.ok(Math.abs(events[0].hz - D4) < 0.01);
  assert.ok(events[2].hz > events[1].hz);
  assert.ok(Math.abs(events[0].seconds / events[1].seconds - 2) < 0.001);
  assert.ok(Math.abs(events[4].seconds / events[1].seconds - 3) < 0.001);
});

test("c in the C-natural line stays the pack's C", () => {
  const events = phraseEvents("G2 A c | B2 A2", ["G4", "A4", "C5", "B4", "A4"], D4);
  const expected = noteHz("C5", D4);
  assert.ok(Math.abs(events[2].hz - expected) < 0.01);
  assert.ok(expected > noteHz("B4", D4));
});

test("a high d follows the pack's note list", () => {
  const events = phraseEvents("D2 F A d | A2 F D", ["D4", "F#4", "A4", "D5", "A4", "F#4", "D4"], D4);
  assert.ok(Math.abs(events[3].hz - noteHz("D5", D4)) < 0.01);
});

test("the whole song plays every line, with a breath between them", () => {
  const events = songToneEvents(
    [
      { abc: "D2 E | F2 G | A3", notes: ["D4", "E4", "F#4", "G4", "A4"] },
      { abc: "A2 F | G2 E | D3", notes: ["A4", "F#4", "G4", "E4", "D4"] },
    ],
    D4,
  );
  const sounded = events.filter((event) => event.hz > 0);
  const rests = events.filter((event) => event.hz === 0);
  assert.equal(sounded.length, 10);
  assert.equal(rests.length, 1);
  assert.ok(rests[0].seconds > 0);
});

test("a mismatched line does not invent pitches", () => {
  assert.deepEqual(phraseEvents("D2 E", ["D4"], D4), []);
});

test("every pack line with ABC plays one tone per note", () => {
  for (const id of readdirSync(join(root, "pack"))) {
    const path = join(root, "pack", id, "phrases.json");
    let phrases: {
      chunks?: Array<{ id: string; notes: string[]; abc: string }>;
      on_the_breath?: { notes: string[]; abc: string };
    };
    try {
      phrases = JSON.parse(readFileSync(path, "utf8"));
    } catch {
      continue;
    }
    const lines = [...(phrases.chunks ?? [])];
    if (phrases.on_the_breath?.abc) {
      lines.push({ id: "on_the_breath", ...phrases.on_the_breath });
    }
    for (const line of lines) {
      if (!line.abc?.trim()) continue;
      const sounded = phraseEvents(line.abc, line.notes, D4).filter((event) => event.hz > 0);
      assert.equal(sounded.length, line.notes.length, `${id} ${line.id}`);
    }
  }
});

test("the close of the air is four notes and the last D is the long one", () => {
  const phrases = JSON.parse(
    readFileSync(join(root, "pack/may-morning-dew/phrases.json"), "utf8"),
  );
  const close = phrases.chunks.find((chunk: { id: string }) => chunk.id === "phrase_4");
  const events = phraseEvents(close.abc, close.notes, D4);
  assert.equal(events.length, 4);
  assert.ok(Math.abs(events[3].hz - D4) < 0.01);
  assert.ok(events[3].seconds > events[1].seconds);
});

test("a cut rises and comes back, and a slide is silent", () => {
  const cut = ornamentEvents("cut", 440);
  assert.ok(cut[1].hz > cut[0].hz);
  assert.ok(Math.abs(cut[2].hz - cut[0].hz) < 0.01);
  assert.ok(cut[1].seconds < 0.07);
  const tap = ornamentEvents("tap", 440);
  assert.ok(tap[1].hz < tap[0].hz);
  assert.deepEqual(ornamentEvents("slide", 440), []);
});
