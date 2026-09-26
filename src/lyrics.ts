import { noteHz, type ToneEvent } from "./tones.ts";
import type { SongSheet } from "./types.ts";

const QUARTER = 60 / 66;

export const PRACTICE_NOTICE =
  "Personal practice on this machine. These words and this melody are not this pack’s setting. They are not a licence to perform, record, or pass the files on. A transcription can still be in copyright when the song is old. Neither source promises that every item is free to reuse.";

export function normLine(value: string): string {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, " ").trim();
}

export function currentVerse(verses: string[], line: string | null): number {
  if (!line) return -1;
  const wanted = normLine(line);
  if (!wanted) return -1;
  return verses.findIndex((verse) => normLine(verse).includes(wanted));
}

export function melodyEvents(
  melody: Array<{ note: string; beats: number }>,
  breakHz: number,
): ToneEvent[] {
  const events: ToneEvent[] = [];
  for (const item of melody) {
    if (!Number.isFinite(item.beats) || item.beats <= 0) continue;
    const seconds = item.beats * QUARTER;
    if (item.note === "rest") {
      const previous = events[events.length - 1];
      if (previous && previous.hz > 0) previous.join = false;
      events.push({ hz: 0, seconds });
      continue;
    }
    events.push({ hz: noteHz(item.note, breakHz), seconds, join: true });
  }
  const last = events[events.length - 1];
  if (last && last.hz > 0) last.join = false;
  return events;
}

function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

export function lyricsPanelHtml(options: {
  sheet: SongSheet;
  revealed: boolean;
  currentLine: string | null;
  playing: boolean;
}): string {
  if (!options.revealed) {
    return `<aside class="lyrics-panel" id="lyrics-panel"><p class="lyrics-note">Hear the line first. The words are a memory of it.</p></aside>`;
  }
  const current = currentVerse(options.sheet.verses, options.currentLine);
  const verses = options.sheet.verses.length
    ? options.sheet.verses
        .map(
          (verse, index) =>
            `<p class="verse${index === current ? " current" : ""}">${escapeHtml(verse)}</p>`,
        )
        .join("")
    : `<p class="lyrics-note">No words came with this melody.</p>`;
  const melody = options.sheet.playable
    ? `<button type="button" class="ghost" id="lyrics-hear">${options.playing ? "Stop the melody" : "Hear this melody"}</button>`
    : `<p class="lyrics-note">No melody is played from here.</p>`;
  const folded = options.sheet.folded
    ? `<p class="lyrics-note">The pitches were brought into this whistle’s range.</p>`
    : "";
  const trimmed = options.sheet.trimmed
    ? `<p class="lyrics-note">This is the opening of that melody.</p>`
    : "";
  const source = options.sheet.source_url
    ? `<button type="button" class="ghost" id="lyrics-source">Source page</button>`
    : "";
  const also = options.sheet.also_url
    ? `<button type="button" class="ghost" id="lyrics-also">${escapeHtml(options.sheet.also_name || "Other source")}</button>`
    : "";
  return `<aside class="lyrics-panel" id="lyrics-panel">${verses}${melody}${folded}${trimmed}<p class="lyrics-note">${escapeHtml(PRACTICE_NOTICE)}</p><p class="lyrics-note">${escapeHtml(options.sheet.credit)}</p><div class="row">${source}${also}</div></aside>`;
}

/** Dev preview only. The lines are the ones already stored on the Salley Gardens pack. */
export function previewSheet(): SongSheet {
  return {
    pack_id: "salley-gardens",
    title: "Down by the Salley Gardens",
    verses: [
      "Down by the salley gardens\nmy love and I did meet",
      "She passed the salley gardens\non her snow-white feet",
    ],
    credit:
      "Preview words already in this pack. Nothing was fetched. A real sheet comes from songs/library after ingest.",
    licence: "personal",
    source_name: "Preview",
    source_url: "https://www.traditionalmusic.co.uk/bgabout.htm",
    retrieved: "2026-09-26",
    melody: [
      { note: "D4", beats: 1 },
      { note: "F#4", beats: 1 },
      { note: "A4", beats: 1 },
      { note: "D4", beats: 2 },
    ],
    playable: true,
    folded: false,
    trimmed: false,
  };
}
