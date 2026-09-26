import assert from "node:assert/strict";
import test from "node:test";
import { currentVerse, lyricsPanelHtml, PRACTICE_NOTICE, previewSheet } from "./lyrics.ts";

test("the current line marks its verse", () => {
  const verses = ["Down by the salley gardens\nmy love and I did meet", "She passed the salley gardens"];
  assert.equal(currentVerse(verses, "my love and I did meet"), 0);
  assert.equal(currentVerse(verses, null), -1);
});

test("words stay hidden until the line has been heard", () => {
  const sheet = previewSheet();
  const closed = lyricsPanelHtml({
    sheet,
    revealed: false,
    currentLine: "Down by the salley gardens",
    playing: false,
  });
  assert.match(closed, /Hear the line first/);
  assert.doesNotMatch(closed, /snow-white/);
  assert.doesNotMatch(closed, /PRACTICE/);

  const open = lyricsPanelHtml({
    sheet,
    revealed: true,
    currentLine: "Down by the salley gardens",
    playing: false,
  });
  assert.match(open, /snow-white/);
  assert.match(open, /verse current/);
  assert.match(open, new RegExp(PRACTICE_NOTICE.slice(0, 24).replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(open, /not a licence to perform/);
  assert.match(open, /Nothing was fetched/);
  assert.match(open, /Hear this melody/);
});

test("a credit cannot inject markup", () => {
  const sheet = previewSheet();
  sheet.credit = `<script>alert("x")</script>`;
  const html = lyricsPanelHtml({ sheet, revealed: true, currentLine: null, playing: true });
  assert.doesNotMatch(html, /<script>/);
  assert.match(html, /Stop the melody/);
});
