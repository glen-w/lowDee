import assert from "node:assert/strict";
import { test } from "node:test";
import {
  GLOSSARY_SECTIONS,
  glossaryCountLabel,
  glossaryEntries,
  glossaryListHtml,
  glossaryShellHtml,
} from "./glossary.ts";

const entries = glossaryEntries();

test("ids are unique and every section has words", () => {
  const ids = new Set(entries.map((item) => item.id));
  assert.equal(ids.size, entries.length);
  for (const section of GLOSSARY_SECTIONS) {
    assert.ok(entries.some((item) => item.section === section), section);
  }
});

test("whistle gestures and the common tune types are all named", () => {
  const terms = new Set(entries.map((item) => item.term));
  for (const term of [
    "Low D",
    "Fipple",
    "Cut",
    "Tap",
    "Roll",
    "Cran",
    "Air",
    "Reel",
    "Jig",
    "Double jig",
    "Single jig",
    "Slip jig",
    "Hop jig",
    "Hornpipe",
    "Polka",
    "March",
    "Waltz",
    "Mazurka",
    "Strathspey",
    "Barndance",
    "Galop",
    "Set",
    "Planxty",
  ]) {
    assert.ok(terms.has(term), term);
  }
  const slides = entries.filter((item) => item.term === "Slide");
  assert.equal(slides.length, 2);
  assert.equal(new Set(slides.map((item) => item.section)).size, 2);
});

test("search leads with the word itself and still finds both slides", () => {
  const reel = glossaryListHtml("reel");
  const first = reel.indexOf("<h3>");
  assert.equal(reel.slice(first, first + "<h3>Reel</h3>".length), "<h3>Reel</h3>");
  const slides = glossaryListHtml("slide");
  assert.equal(slides.split("<h3>Slide</h3>").length - 1, 2);
  assert.match(slides, /Ornaments/);
  assert.match(slides, /Kinds of tune/);
  assert.equal(reel.split("<h3>").length - 1, 1);
  assert.match(glossaryListHtml("6/8"), /Double jig/);
});

test("sharp letters and feadóg fold into the search", () => {
  assert.match(glossaryListHtml("f#"), /<h3>F♯<\/h3>/);
  assert.match(glossaryListHtml("feadog"), /Tin whistle/);
  assert.equal(glossaryCountLabel("xyzzy"), "No word under that.");
  assert.match(glossaryCountLabel(""), /^\d+ words$/);
});

test("the page shell has a search box, the groups, and a way back", () => {
  const shell = glossaryShellHtml();
  assert.match(shell, /id="glossary-q"/);
  assert.match(shell, /id="glossary-back"/);
  assert.match(shell, /Kinds of tune/);
  assert.match(shell, /data-gloss-jump="gloss-ornaments"/);
  assert.match(glossaryListHtml(""), /<h2>Ornaments<\/h2>/);
});
