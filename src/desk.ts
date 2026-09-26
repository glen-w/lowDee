import type { PackSummary } from "./types";

export interface PackRefusal {
  folder: string;
  reason: string;
}

export function rightsHtml(
  pack: {
    rights?: string;
    source?: string;
    author?: string;
    licence?: string;
    site?: string;
    placement?: string;
  },
  includePlacement = true,
): string {
  const lines: string[] = [];
  if (pack.author) lines.push(pack.author);
  if (pack.licence) lines.push(pack.licence);
  if (pack.source) lines.push(pack.source);
  if (includePlacement && pack.placement) lines.push(pack.placement);
  if (pack.rights === "brought") {
    lines.push("Copyright on this file is not established. It stays on this machine.");
  }
  const body = lines.map((line) => `<p class="meta">${escapeText(line)}</p>`).join("");
  const site =
    pack.site && pack.site.startsWith("https://")
      ? `<p class="meta"><a href="${escapeAttr(pack.site)}">${escapeText(pack.site)}</a></p>`
      : "";
  return body + site;
}

export function deskHtml(desk: PackSummary[], refused: PackRefusal[]): string {
  const tunes = desk
    .map((pack) => {
      const open = pack.open
        ? `<button type="button" class="primary" data-open-pack="${escapeAttr(pack.id)}">Open</button>`
        : `<p class="meta">After The May Morning Dew.</p>`;
      return `<article class="shelf-card"><h2>${escapeText(pack.title)}</h2>${rightsHtml(pack)}<div class="row">${open}</div></article>`;
    })
    .join("");
  const blocked = refused
    .map(
      (item) =>
        `<article class="shelf-card"><h2>${escapeText(item.folder)}</h2><p class="meta">${escapeText(item.reason)}</p></article>`,
    )
    .join("");
  const empty =
    tunes || blocked
      ? ""
      : `<p class="lede">A place to look is the <a href="https://www.traditionalmusic.co.uk/song-midis/songs-midis.html">Traditional Music Library</a>. Download the file yourself, and drop the folder here.</p>`;
  return `
    <div class="app-shell">
      <div class="topbar">
        <p class="eyebrow">Desk</p>
        <button type="button" class="ghost" id="desk-back">Back</button>
      </div>
      <h1>A tune on the table</h1>
      <p class="lede">One folder at a time, after the first air. Name the source when a tune is public domain. Name the licence and the author when you have them. When the copyright is not established, the card says so. A piano MIDI names its own pitches, and this whistle moves them into its range. Nothing here is uploaded.</p>
      ${empty}
      <div class="shelf-list">${tunes}${blocked}</div>
    </div>`;
}

function escapeText(s: string): string {
  return s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function escapeAttr(s: string): string {
  return escapeText(s).replaceAll('"', "&quot;");
}
