Type: PRODUCT
Authority: What the build does today, and what [CONCEPT.md](CONCEPT.md) still leaves for later. It does not define the path, the listen states, or the storage rules.

# Features

Status words: **in the build**, **partial** (the screen or the file exists; a piece of the behaviour does not), **ahead** (described in the concept, not in this tree).

The door is `may-morning-dew`, track `beginner`, ten nodes. `hedwig` is a page, not a listen target. After that pack is settled, the book’s chain opens, one pack at a time: the book’s staircase, *Down by the Salley Gardens*, C natural, *As I Roved Out*, two First Tones, two dance tunes, two roll tunes, and more ornament. A shelf of those packs, plus titles that stay on the page only, appears after Salley Gardens is settled. Practice names the whistle in hand when more than one profile is stored.

```mermaid
flowchart LR
  card[Name the whistle]
  warm[Hands and breath]
  n1[first_sound]
  n2[staircase]
  n3[breath_octave]
  n4[hedwig]
  n5[on_the_breath]
  n6[air_bare]
  n7[orn_cut]
  n8[orn_tap]
  n9[orn_roll]
  n10[air_may_morning_dew]

  card --> warm --> n1 --> n2 --> n3 --> n4 --> n5 --> n6 --> n7 --> n8 --> n9 --> n10
```

A later sitting skips the card. It starts at the warm-up, then the first unsettled part, unless Settings says to skip the warm-up on launch. The rail opens any part. On the staircase, the octave, and the air, each note or phrase can be opened on its own. Settled and started marks stay on the rail. The player can also step past a part when the loop cannot hear.

## In the build

### Practice

- Object card: the low D with every hole closed, a nickname, reading, and musical background. Hands and breath saves the profile and opens the warm-up. No tuner and no microphone test.
- Before a sitting: a guided pass for the hands and wrists, then a counted breath. One pass, both hands. The pass is not stored. Pause holds the clock. Next skips the current exercise. Ease off skips the rest. Settings can skip the pass the next time the app opens. The default is to show it.
- One practice screen for the listen nodes, and a letter-note card for `hedwig`. The rail names every part and opens it. Settled and started stay marked.
- Shorter copy when background is `high_d`, on the staircase and on the octave.
- Side-view low D, holes numbered 1–6 from the window. Closed is filled, open is a ring, half covers only the hole that says half. A phrase or the staircase is a row of those columns. The letter sits beside the whistle. The second octave is the same holes with a mark by the window.
- The letter and its fixed-do syllable sit on the whistle and on each column (D is Re). Staff after Hear, for readers, on a joined phrase and on a phrase node, with a letter row and the same syllables under that phrase. The staff header uses the pack’s meter and key. A non-reader does not see the staff.
- Hide pictures on a node that asks for it. On a song, words can hide on their own. They show after Hear, or after the line has been played.
- Lyrics, on a phrase, when `songs/library/` has a sheet for this pack. The button is Lyrics. The words stay hidden until the line has been heard. Hear this melody plays the fetched line as plain tones and says it is not this pack’s setting. The panel names the source and says the copy is personal practice only. The files are not in git and not in the bundle. How to fetch them is in [songs/README.md](songs/README.md).
- Hear and Slower. Book recordings play when that step has them in `book/` and the files are on this machine, in the order `tracks.json` names. The door’s cut and tap use the rolls introduction. The roll uses that introduction and both takes on A. The octave, once Octave is selected, uses the all-closed fingering through the two high notes. Skip the talk, in settings, starts at the part this step uses. Play the talk too hears the file from the start. Otherwise a reference wav plays when it is on disk. When neither is there, and the step has ABC or a cut, tap, or roll, Hear plays those notes as plain tones and the card says so. Slower plays a phrase or a held note at three-quarter speed, including the tones. An ornament demo plays at full speed. Grading stays off while either plays. Dance packs add Hear the pulse. Airs do not.
- One remark after an attempt, from `remarks.json`, worded for reading and background. Under it, after feedback, a quiet line of that attempt against the notes the pack expected. Each note is named on the line. A cut or a tap is marked where the pack asked for it; a roll shows both. Couldn’t-hear and an ornament that abstains draw nothing. Hear that plays the take once from memory. It is dropped with the attempt.
- When a book recording or a reference wav is on disk, Hear is the primary action until that file has played once. If neither is there, I’m ready stays available. Slower stays on phrases and held notes. An ornament demo stays at full speed.
- After the warm-up, one review of the last settled listen step in the open pack, when one exists. Onward skips it. It does not write progress.
- Just that note, when a phrase fault names one pitch. That note is held on its own, then the chunk returns. The target does not move.
- Hold bar from the engine’s settle clock on the first sound, the staircase, and the octave.
- An attempt ends when the player stops, or when they press I’m done. After a settled section, Settings can start the next note or phrase in this part, keep going into the next part, move and wait, or stay. The choice is stored on the whistle. A miss stays.
- Low D and Octave on the breath-and-octave part, switching the target between the two.
- Couldn’t hear — continue, which opens the next node.
- Resume on the first unsettled node of the earliest open pack the next time the app opens. Within that pack the rail still opens any part.
- After calibration, the practice card shows the frozen target and this whistle’s break, in hertz. Before that, the line says the listen is whether low D is there.
- Headphones line on the practice card.
- Glossary, from the name card, the warm-up, and practice. Whistle words, ornaments, and the common tune types. Search, and a jump for each group. Escape or Back returns. Opening it pauses a warm-up that is running.
- This tube, from the name card and from practice: a short page in our words for the low D, other keys, and names to hear elsewhere. No audio.
- Shelf, once *Down by the Salley Gardens* is settled. It lists the path, beginner, improver, and players shelves, including titles that stay on the page only. Open returns to that pack. A Session link opens in the browser.
- Lesson set, on the practice screen. A dropdown of public-domain song packs the catalog has already opened. Add saves that pack on this whistle. The source of the selected song is named under the list. A saved song opens that pack. Take off removes it. Drills and page-only titles are not in the list. Adding one does not change the path order.
- Another whistle, from the name card and from practice, when more than one profile is stored.
- Reading and background are asked when the whistle is named, and changed from Settings. They are not on the practice card. They change the pictures and the remark. They do not move the break.
- Recalibrate is on Settings, once a break is stored and this sitting’s warm-up is done. It is the same low-D hold. The target of an attempt already open does not move. When a later low hold sits outside the window, the card says the whistle has warmed. There is no needle.
- A tune on the table, once *The May Morning Dew* is settled. One folder from `teacher/`. Back returns to the path.

### Listening

- On-device microphone via cpal. Mono mix of the input channels. Sample rate is the device rate.
- YIN pitch in a band suited to a low D through the notes this pack uses.
- Search near the note the pack expects, including a half-frequency or double-frequency fold when that lands on the expected note.
- Separate raw pitch so an early octave break can be remarked while the guided tracker would have folded it back down.
- Target frozen for the attempt. Intervals taken from this whistle’s `break_hz` once the first hold has settled; concert D4 only as the stand-in before that.
- RMS as a stand-in for how much air is in the note. The floor is stored with the profile.
- Evidence for a sealed leak, a held low D, an early break, the octave, a note still on D, a note found, a phrase, tongued restarts, a cut that became a note, a missing tap, a breath that chops a line, a high note that fell back down, a short roll that split, a slide that did not arrive, couldn’t hear, and abstain.
- Calibration written when the first sound settles, and again when Recalibrate settles: `break_hz`, `rms_floor`, `cal_as_of`. Samples are not kept. An attempt already open keeps the target it started with.
- If the microphone will not open, the screen stays idle, says it could not hear, and does not mark the node started. A heard pitch that differs by exactly one hole marks that hole. Raw pitch on a note at the break or above can remark that it fell back to the low fingering.

### Content in the door

- *The May Morning Dew*, teaching setting in D, 3/4, slow. Four chunks: Opening, Home again, Second strain, Close.
- Staircase D, E, F♯, G, A, B — the notes those chunks use.
- On-the-breath line D–E–F♯–G.
- Fingerings for D4 through D5, including a C natural picture (oxxooo) and C♯ (all open). The chunks do not ask for C. The loader refuses the pack if those holes or intervals disagree with the engine’s scale table, or if a note is not on this whistle.
- Ornament descriptions and marks: cut and tap on the opening, a roll on the return, a cut on the close. Demo note A4. The last node draws those marks and grades them.
- Remark sentences for each evidence id. Node copy lives on the manifest.
- Held-note recordings D4–B4 and D5, for Hear on the first sound, the staircase, and the octave. Provenance is in `pack/may-morning-dew/ref/README.md`.
- `hedwig` opens [Hedwig’s Theme letter notes](https://www.irish-folk-songs.com/hedwigs-theme-piano-flute-and-tin-whistle-notes.html) in the key of D. The card shows the staircase holes, letters, and syllables. I’ve played the opening, or Not yet — continue, writes `settled` with `via` `stepped` and moves on. The melody is not in the pack.

### Content after the door

Same folder shape. Each shipped pack names a public-domain source, or a licence and an author when that is what it has. Chapter names are pointers. Hear plays a wav when one is added, and the pack’s notes as plain tones until then.

- `book-staircase` — B, A, G, F♯, E, D, C♯, octave D′. No new air.
- `salley-gardens` — *Down by the Salley Gardens*, with the Mourne Shore names in `aka`. Words after the line is heard. One cut mark.
- `c-natural` — the `oxxooo` hold, then a short phrase. If that hold is not the interval and not a one-hole leak, the picture becomes `oxxoxx` (the bottom two holes closed, not a half-hole). A heard settle writes `cnat_fingering`. Stepping past does not.
- `as-i-roved-out` — A dorian. C natural in the line. One phrase through E′.
- `amazing-grace`, `skye-boat-song` — slow tunes in D, with a breath mark on the line.
- `drops-of-brandy`, `winster-gallop` — slip jig and galop, slowed, with a pulse the player can hear.
- `green-gates`, `melancholy-martin` — a roll, then a short roll, each on a long note before the tune. Sources are named in the manifests.
- `more-ornament` — slide, cran on low D, double tap, triplet, and a vibrato page.
- Page-only titles from the rights ledger, on the players shelf, with no notes and no audio.

Later packs that use the second octave carry fingerings through B5. The door’s chart stops at D5.

### On the device

- Several profiles can be stored. Saving from the object card activates the new one. Practice can switch the one in hand.
- Progress is stored per profile and per pack: `started` or `settled`, with `via` `heard` or `stepped`, and a time.
- No account, no upload, no score on the sounding screen.

### Bench

- `listen fixtures`, `listen grade`, `listen gate`, `listen grade-take`. `grade` prints `steady_low_d`, `early_break`, or `noise`. `grade-take` scores a wav named by a sidecar. Two synthetic sidecars live in `bench/takes/`. `fixtures` also writes the wave 2 traces. On those traces a short cut comes back `abstain` and settled; a cut that lasts does not.
- `bench/score_traces.py` checks those traces.
- Human protocol in `bench/HUMAN_GATE.md`.
- `corpus/` for local labeled takes. The scorer does not read them.

## Partial

| Feature | What is present | What is not |
| --- | --- | --- |
| Ear before playing | Hear plays a wav when one is there, otherwise the pack’s notes as plain tones, and then the staff may appear. When the wav is there, Hear is the primary action until it has played once. When the book files are on this machine, the door’s cut, tap, roll, and octave play those recordings ahead of the wav | Phrase wavs and cut, tap, and roll demos are not in `ref/` yet, so without the book those steps stay playable without a whistled model. The hole picture still shows. |
| Ornaments on the air | Marks are drawn on the phrase and passed into the attempt when the node grades them. The moving hole is marked on that note. A cut on a later pack waits until the door’s cut has settled. | Phrase wavs and demos are still absent, so Hear plays the notes. |
| Several whistles | The practice screen can switch the whistle in hand, and the name card can pick one already stored. Reading and background can be changed from Settings. | A new whistle is still named on the object card. |
| Early-break remark during the hold | The pill can show it as soon as a frame latches | The attempt finishes when they stop, or on I’m done. The target does not move. |
| `content_hash` | The loader checks sha256 of the manifest identity, the nodes, and the content files that are present | A mismatch refuses the pack. |
| Tests | Pitch, pack load and hash, a foreign key, an unknown note, rights, a page-only tune, a brought file the app will not ship, a piano MIDI shifted onto the whistle, the chain and the shelf, a bad teacher folder that does not drop the door, store round-trip, `pack_id` migration, C natural on one whistle, a corrupt file set aside, a heard settle kept, early-break target freeze, phrase order, breath gap, one-hole leak, ornament abstain, a short cut that settles, a phrase ghost, a warm hold, a mark on a phrase, a cracked octave E, and the two `bench/takes/` sidecars. Picture, path, glossary, and ghost tests in the webview. The gate binary asserts the synthetic protocol. | No automated test drives the Tauri window: object card, warm-up, or practice. |

## Ahead

Reference recordings for the phrases, the songs, and the ornament demos are still not in `ref/`. Until a wav is there, Hear plays the pack’s notes as plain tones. Words on a song appear after Hear, or after the player has played the line. Slower is for phrases and held notes, including those tones. An ornament demo plays at full speed.

F natural, half-holing as a menu, and the book’s later accidentals are not in the scale. The desk screen is in the build. A folder under `teacher/` can be a pack, or one MIDI file plus an optional `notice.json`. A piano MIDI is moved onto this whistle. Nothing in that folder is uploaded.

## Node by node

| Node | Player sees | Listen mode | Settles when |
| --- | --- | --- | --- |
| `first_sound` | All holes closed, hold bar | Hold low D | About 10 s near the target, and that result is allowed to mark the node |
| `staircase` | Columns for D through B, current note marked | Single note | Each note held, then the node when B is done |
| `breath_octave` | Same picture; octave mark when Octave is selected | Low D or octave D | Low hold, or the octave hold |
| `hedwig` | Staircase holes, letters, and syllables, and a button that opens the D letter notes | None | I’ve played the opening, or Not yet — continue |
| `on_the_breath` | A column for each note of the line, staff after Hear | Joined D–E–F♯–G | Phrase heard, or the join remark |
| `air_bare` | Current chunk | Phrase of that chunk | Each chunk, then the node on the last |
| `orn_cut` | A, holes for A, hole 2 marked | Cut on A4 | Abstain, or step past |
| `orn_tap` | A, hole 3 marked | Tap on A4 | Abstain, or step past |
| `orn_roll` | A, holes 2 and 3 marked | Roll on A4 | Abstain, or step past |
| `air_may_morning_dew` | Same chunks; pictures can hide; cut, tap, and roll where marked, on the hole that moves | Phrase, and those marks | Same chunk walk |

A cut that lasts, a missing tap, a leak, an early break, or a phrase that never arrives leaves the node where it is. Try again starts another attempt against the same frozen target.

## Reference audio the screen looks for

| Step | Path |
| --- | --- |
| First sound, staircase, octave | `ref/D4.wav` … `ref/B4.wav`, `ref/D5.wav` |
| On the breath | `ref/on_the_breath.wav` |
| Air chunks | `ref/phrase_1.wav` … `ref/phrase_4.wav` |
| Cut, tap, roll | `ref/cut_demo.wav`, `ref/tap_demo.wav`, `ref/roll_demo.wav` |

Only the held notes are in the repository today. Until a phrase or demo wav is added, Hear plays the notes, unless the book files are on this machine. Then the door’s octave, cut, tap, and roll play the tracks named above. Checklists for the missing files are in each pack’s `ref/README.md`.
