import type { PackSummary } from "./types";

export interface PackRefusal {
  folder: string;
  reason: string;
}

export function deskHtml(desk: PackSummary[], refused: PackRefusal[]): string {
  const tunes = desk
    .map((pack) => {
      const open = pack.open
        ? `<button type="button" class="primary" data-open-pack="${escapeAttr(pack.id)}">Open</button>`
        : `<p class="meta">After The May Morning Dew.</p>`;
      const source = pack.source ? `<p class="meta">${escapeText(pack.source)}</p>` : "";
      return `<article class="shelf-card"><h2>${escapeText(pack.title)}</h2>${source}<div class="row">${open}</div></article>`;
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
      : `<p class="lede">Drop one folder in teacher/. It needs a named public-domain source, and it has to fit this whistle.</p>`;
  return `
    <div class="app-shell">
      <div class="topbar">
        <p class="eyebrow">Desk</p>
        <button type="button" class="ghost" id="desk-back">Back</button>
      </div>
      <h1>A tune on the table</h1>
      <p class="lede">One folder at a time, after the first air. Nothing here is uploaded.</p>
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
