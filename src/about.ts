/** Beside the profile. Our words. The book’s chapters are not pasted here. */

export function aboutHtml(): string {
  return `
    <div class="app-shell">
      <div class="topbar">
        <p class="eyebrow">This tube</p>
        <button type="button" class="ghost" id="about-back">Back</button>
      </div>
      <h1>The low D</h1>
      <p class="lede">A tube of its own, an octave below the tin whistle. The stretch is wide. The air is slower. Fingerings you know from a small whistle still have to be learned on this one.</p>
      <h2>Other keys</h2>
      <p>Other keys are other whistles. This companion stays on low D. Changing whistle is how a tune in another key is played. The book’s back table is a reason to pick up a different tube. It is not a second instrument inside this one.</p>
      <h2>Names to hear elsewhere</h2>
      <p class="meta">No audio here. These are players to find on your own.</p>
      <p>Davy Spillane. Cormac Breatnach. Joanie Madden. Michael McGoldrick. Mary Bergin, on the small whistle, for the ornament language the low D borrows.</p>
    </div>`;
}
