Type: CONTRACT
Authority: Product decisions — audience, the path, listen states, what is stored, rights, and what is left out. Module layout and the listen implementation live in [ARCHITECTURE.md](ARCHITECTURE.md). What the build actually does today lives in [FEATURES.md](FEATURES.md).

# Low D — concept note

**Date:** 2026-09-26  
**Status:** Decided. The player is new to the whistle. Wave 1 is the door. Wave 2, the book’s path, is in the build: the same listen loop, a chain of local packs, and a shelf that stays shut until *Down by the Salley Gardens* is settled. Wave 3 is the ear and the desk: a phrase ghost after feedback, a recalibration between attempts, and one folder a teacher wrote, after *The May Morning Dew*. Wave 4 is the sitting: hear the model before the first attempt when a recording is there, hear that take back without storing it, review one settled sound after the warm-up, and open the note a phrase fault names. The app does not invent a whistle to copy.  
**Sources:** `brainstorm/2026-09-26-low-d-whistle-p0-spine.md`, `brainstorm/low-d-whistle-concept-and-eng-spike-2026-09-26.md`, `brainstorm/low-d-whistle-learning-software.md`. The ex–high-D track in the spine is withdrawn. This note replaces it.  
**Book on the table:** *The Low Whistle Book* — Stéfán Hannigan and David Ledsam (Soar Valley Music, ISBN 9780952530510). The book stays on the table. The app does not become the book.

---

## Shape

A local practice companion that teaches the Irish low-D whistle from the first sound. The player has not necessarily touched a whistle before. They may read music, a little, or not at all. They may sing, play something else, or be new to music. None of that is required, and none of it is a whistle technique.

One path. The instrument skills are the same for everyone: seal a note, find the next note with one finger, keep the octave on purpose, join notes on the breath, learn one slow air by ear, then put a cut, a tap, and a roll on it. Background and reading change the words and the pictures on that path. They do not create a second course, and they do not let anyone skip the fingers.

The tune at the end of the first pack is the slow air *The May Morning Dew*. Notation is off unless they want it. The microphone stays on the device. Feedback is a short remark, the kind a patient player makes across a table. There is no score, no account, and no network required to play.

After that air, the same listen loop and the same whistle profile open a second wave of local packs. Those packs follow the syllabus of *The Low Whistle Book*. Chapter names in the app are pointers (“Getting Started”, “First Tones”). Settings, diagrams, and prose in the packs are ours. The book’s tracks are not in the repo. When a copy sits in `book/` on this machine, Hear can play the matching file.

The app is a companion to a teacher, to the session, and to the book on the table — not a replacement for any of them. It owns the first months of the low D, then gets out of the way inside the tunes.

---

## Who it is for

A complete beginner to the whistle. Prior whistle is the exception we can tolerate, not the person we design for.

What varies, and what we ask once in plain language:

| They tell us | What we change | What we do not change |
| --- | --- | --- |
| They do not read music | Holes and sound only. Letter names arrive as labels on holes they can already find. No staff, no ABC, no “what is a semitone.” | The notes, the order, the ear. |
| They read, or read a little | The same holes and the same sound, with the staff available as a map of the phrase they just heard. The lesson still starts with the recording. | We do not open on a stave, and reading is not a way to skip playing. |
| New to music | Pulse is clapped and heard. We say “start the note and let it join the next,” not “legato.” | We still ask them to listen and copy. |
| Plays something else (flute, recorder, piano, guitar, fiddle, sings) | Copy can use a word they know. Wind players get an early remark if they tongue every note. Someone who already has tunes in their ear still sees every fingering. | We do not place them out of the staircase. A flute embouchure is not a fipple. |
| Already plays the high D | The staircase copy is shorter, and the octave lesson says the low whistle wants slower air than the small one. | Same nodes. Fingerings match; the air, the stretch, and the break still have to be learned on this tube. |

We do not ask for a level. “Beginner” and “intermediate” do not tell us whether they can read, whether they can keep a pulse, or whether they have ever covered a hole.

What we owe them in the first hour: a low D that seals, the next few notes by lifting one finger at a time, and the opening phrase of the air in the ear with the diagram in front of them. What we owe them by the end of the first pack: that air, slow, bare and then with three ornaments, playable with the page put away. A non-reader and a strong reader both get there.

What we owe them in the second wave: the book’s notes in the book’s order so the page and the holes match, two song airs (*Down by the Salley Gardens*, then *As I Roved Out*), C natural that speaks on this whistle, then the book’s first tones, rolls, and ornaments — still one path, still this tube.

---

## Why this instrument

Bottom note is D4, about 294 Hz, one octave below the usual tin whistle. A usable range is two octaves. The top of the second octave depends on the maker. The first pack asks only for a clean break to the octave D and for the notes inside the air. The second wave asks for C natural inside a song, and for one phrase that crosses the break on purpose.

A new player meets four facts immediately:

- All six holes covered is the home note. Each step up is one finger lifted. Leaks on the low notes are the ordinary fault, because the stretch is wide.
- More air raises the octave on the same fingering. Blowing harder is how a beginner finds the second octave by accident. The lesson is finding it on purpose and coming back down.
- The tongue is for starting a phrase. The notes in between are joined on the breath. Players who have been taught to tongue every attack will do that here until we say otherwise. Players who have never tongued need to hear the join, not a lecture.
- C natural, when a phrase needs it, depends on this whistle. One fingering in the pack, checked against this whistle’s own scale. Half-hole slides, a menu of cross-fingerings, and F natural wait for a later pack.

Pitch rises as the tube warms, and makers do not agree with each other or with concert D. The listening target is this whistle, named and measured once a note will actually hold. Until that hold exists, we do not pretend to tune them.

---

## The hour that has to work

The first session cannot open on a ten-second test. A beginner cannot hold low D yet. Naming the whistle comes first. The calibration is the successful hold at the end of the first lesson, written down once, and then left alone while a phrase is playing.

Before any ornament matters, one new player on a real low D must be able to:

1. Cover the holes and hold low D.
2. Soft-fail an early octave break and hear a single remark (“softer — stay on the low one”), with the target unchanged mid-phrase.
3. Retry and settle the break.
4. Play the first phrase of the air, notes they have already found one at a time, against that same target.

If that loop is unstable, more tunes and a smarter roll detector will not save it. Cut, tap, and roll ship after this gate. They are new gestures, taught by ear on a long note, then marked on the air. The detector may stay silent. It is not an AI score of a roll.

Wave 2 does not reopen that hour. Once low D will hold, a short review walks the book’s staircase so the page and the holes match. The book begins on B because three fingers seal before six do. Wave 1 already contradicted that first note on purpose: low D, all six holes, is the note this tube is for.

---

## What the player meets

### First screen

An object card: a nickname for this whistle, “this horn.” No microphone test and no tuner needle. A sitting opens with a guided pass for the hands and the breath — wrist and finger stretches, a short set of wrist motions, one finger at a time on the table, then a counted hiss — before the whistle comes up. Ease off if it hurts. It is a warm-up, not a treatment, and the pass itself is not stored. Settings can skip that pass the next time the app opens. The next thing after the pass is how to hold it.

When low D will sit still for about ten seconds, that hold is the calibration. Break frequency and breath energy lock, one settled cue, and the profile is stored. Failed early attempts are practice, not a bad score. Calibration audio is not kept in the progress record and is not uploaded.

The card uses the same session-table surface as practice. The profile lives on the device, outside the pack. Someone with two whistles can name both. Most beginners have one.

### Reading and background, stored beside the profile

```
{ reads: no | some | yes, background: none | wind | other | high_d }
```

`high_d` is an answer someone might give. It shortens a few sentences. It is not the default, and it is not a track. The name card asks. After a sitting has started, the practice card does not ask again. A change is Settings, which writes the same fields on the profile.

### Four listen states

| State | What is true |
| --- | --- |
| `idle` | Ready. Mic off or armed and quiet. The whistle, the hole picture, and the air’s name may show. |
| `wait` | The app is silent. The learner plays. Playback has stopped, so the app cannot grade its own speaker. |
| `sounding` | On-device pitch and breath are live. The frame stays clear: no points, no red marks, no running score. |
| `feedback` | One sit-in remark. Optionally, a ghost of the phrase against what just happened. Never a letter grade. |

If the detector is bad, the loop stays in `wait` and does not freeze. Repeated uncertainty can be stepped past. Each new sitting opens on the first part not yet settled. Any part can be opened from the rail, and a note or phrase inside the staircase, the octave, and the air can be opened on its own. A start and a settle are stored on that part.

### The path (wave 1)

Track id: `beginner`. One chain. Hole diagrams carry the early nodes. The staff appears only when `reads` is `some` or `yes`, and only after the phrase has been heard.

| # | Id | What the player is doing | What we will say, if we are sure |
| --- | --- | --- | --- |
| 1 | `first_sound` | Whistle to the mouth, pads on the six holes, a steady low D. The picture shows every hole covered. A hold that settles writes the profile. | “Holes aren’t quite sealed.” “That’s the low one.” |
| 2 | `staircase` | One finger lifts at a time, only through the notes the air uses. Each new note is heard, then found, then played back. | “That’s still D — the bottom finger is down.” “E is there.” |
| 3 | `breath_octave` | Same fingering as low D. The octave comes with air and goes home again. | “Broke early.” “Softer — stay on the low one.” “Octave’s there.” |
| 4 | `hedwig` | *Hedwig’s Theme*, the opening, from letter notes in D on the [Irish folk songs page](https://www.irish-folk-songs.com/hedwigs-theme-piano-flute-and-tin-whistle-notes.html). The key of C on that page is for piano, flute, and recorder. A high mark is the same holes and more air. The tune is not in the pack. | None. They mark the opening tried, or continue without having played it. The loop does not grade it. |
| 5 | `on_the_breath` | A short phrase, notes joined. The tongue starts the line and then stays off. Pulse is the air’s pulse, heard and clapped, not a click track laid over a free melody. | “Each note is restarted.” “Let them join.” |
| 6 | `air_bare` | *The May Morning Dew*, phrase by phrase, no ornaments. Hear it, see the holes, play from the start of the chunk. If a phrase needs C natural, that fingering appears here as a picture, checked against this whistle. | A phrase remark, then the phrase again. |
| 7 | `orn_cut` | A new gesture on one long note: a finger snaps open and shut, too short to hum, on the pulse. Demonstrated before it is asked for. | “The cut became a note.” Or silence if the contour is messy. |
| 8 | `orn_tap` | A finger below the sounding note flicks shut and open. Pitch dips and returns. | Same economy as the cut. |
| 9 | `orn_roll` | A long note: cut near the start, tap later, the note itself still the one they meant. | “The tap didn’t arrive.” Or silence. |
| 10 | `air_may_morning_dew` | The same air, with cut, tap, and roll only where the pack marks them. The hole picture can hide. The staff, if it was on, can hide too. | One remark about the phrase they just played. |

The book’s staircase, the two song airs, C natural, the First Tones, the slowed dance tunes, rolls, and slide, cran, double tap, triplet, and a named vibrato are the second wave, in the pack chain below. Still later: F natural, a menu of C-natural fingerings, and any long stay in the second octave past the one phrase that crosses the break.

Listening runs on `first_sound`, `staircase`, `breath_octave`, `on_the_breath`, and `air_bare`. `hedwig` sits after the octave and before notes are joined: a page, not a listen target. A new player is pointed at the low note first. Settling a part records that it was heard, or that they stepped past it. The other parts stay open.

### Ear, then whatever picture they can use

Every phrase is heard before it is played. Then at most two pictures:

- Six holes, top to bottom. Filled, open, or half covered. The second octave is the same picture with a mark, not a second chart. A cut, tap, or roll uses that same picture: the sounding holes stay as they are, and the hole that moves is marked.
- The letter, and sol-fa for that note (D is Re), sit on the holes.
- If they read: the staff of that phrase, as a map of what they just heard. Under that staff, after the phrase has been heard: the same letters and the same syllables.

Letter names sit on the holes. They are not a quiz. Singing the phrase back is offered and never required; plenty of musical people do not sing, and plenty of beginners should not be stopped for it.

On the air, play resumes from the start of the current chunk. Putting the pictures away is the point of the last node, for readers and non-readers alike.

Ornament glyphs appear only on the nodes that teach them. In the pack they are marks beside the tune, not a cluster of grace notes inside the ABC.

Song packs in wave 2 may show words after the line has been heard, aligned to phrase marks, hideable like the staff. Singing stays offered and ungraded.

A fuller set of words can sit beside that line, and it can be hidden on its own. Those words are not in the pack. `songs/ingest.py` can fetch them, and a melody MIDI, from the [Traditional Music Library](https://www.traditionalmusic.co.uk/song-midis/songs-midis.html) and the [Mudcat Café](https://mudcat.org/) Digital Tradition, into `songs/library/`. That folder stays off git and out of the bundle. The sitting does not call the network. The panel names the page the words came from. It says the copy is for personal practice, that it is not this pack’s setting, and that it is not a licence to perform or to pass the files on. Many of the songs are old. A transcription on either site can still be in copyright, and neither site promises that every file is free to reuse.

### How feedback sounds

One idea, in words that match what they told us, then the loop is open.

- “Softer — stay on the low one.”
- “The low note didn’t speak.”
- “You restarted every note. Start once, and let the rest join.”
- “That cut turned into its own note.”
- “Couldn’t hear. Come a little closer to the mic and play it again.”

A reader may also see the hole that leaked. A non-reader hears the same sentence and sees the same hole, without a note name they have not met. Internal evidence may record that a break was early and a retry settled. The screen never shows a percentage, a streak, or a letter.

If they recalibrate, the target moves once, between attempts, never while a phrase is in flight.

### The sitting (wave 4)

The loop above is unchanged. A sitting uses it in four ways.

When a reference recording is on disk, the first action on a fresh step is Hear. I’m ready comes after that file has played once. Slower stays on phrases and held notes. An ornament demo stays at full speed. A missing recording does not block the step.

After an attempt that was heard, Hear that plays the take once. The remark and the ghost stay. Couldn’t hear, and an ornament that abstains, offer no playback. The samples live in memory, capped at about thirty seconds, and are dropped when the attempt ends or the next one starts. They are not written to the progress record.

After the warm-up, if this profile has a settled listen step in the open pack, the sitting asks for that step once: the last settled hold, or the last settled phrase. Onward skips it. The review does not write progress and does not open a later pack.

When a phrase remark is that the note did not speak, or that it is still low D, Just that note opens that pitch alone, then returns to the same chunk. The target does not move.

---

## Listening

Low D is monophonic. The hazards are a strong second harmonic (the tracker jumps the octave), breath noise, a quiet instrument, and cuts short enough that ordinary note segmentation deletes them. Beginners add a fifth hazard: the note is often not there yet. Silence, a leak, and “too much air” have to be different remarks.

Listening is three measurements, all on device, and only after `first_sound` has settled:

- **Where this whistle speaks.** `break_hz` from the held low D. Later notes are intervals from that, not cents from A440.
- **How much air is in the note.** `rms_floor` as a breath proxy. An early break is “too much air,” not “you are sharp.”
- **Whether a short gesture happened near the pulse,** on the cut, tap, and roll parts. A cut is an upward blip under about 70 ms that returns. A tap is a dip of the same kind. A roll is both, in order, on a note whose body is still the written pitch. We comment on placement and shortness. We do not grade the pitch of the blip.

Until the profile exists, the first node listens only for “a low D is present and steady,” with wide tolerance. It does not scold intonation.

Expected note comes from the pack. The tracker searches around that note. Open transcription is out of scope.

Unstable contour, speaker still playing, or a second instrument in the room: stay in `wait`, say it couldn’t be heard, do not count a failure. Headphones are the setup we describe.

Slow-down is for listening. Gesture checks read the ornament marks, not a time-stretched waveform.

No camera, no sensor whistle, no breath hardware. A finger is mentioned only when a wrong pitch makes the leak obvious. Taste is not scored.

### Wave 2 listening

Still on device, still relative to this whistle’s `break_hz`.

- Expected note may sit above the break. The tracker searches around that note. Open transcription stays out.
- C natural is an interval from the held low D, not a cent mark from A440.
- A short roll is the wave-1 roll with less room. Placement and shortness only.
- A slide is a pitch that moves and arrives. A cran and a triplet are contour checks that may abstain. Vibrato is named and almost never remarked on. Vibrato does not become a grade.
- Breath in song packs is a phrase mark: the line ends, the player breathes, the next line starts. The app listens for a gap. It does not score the poem.

New bench fixtures, still including beginner mess: leaked C natural, second-octave E that cracks, an A-dorian phrase, a breath in the right gap and a breath that chops the line, a short roll that became two notes.

---

## The pack (wave 1)

The first pack is one local folder. The app does not need a content service to make a sound.

```
pack/
  manifest.json          # id, version, track, title, node_ids, content hash
  phrases.json           # chunks, staircase notes, the on-the-breath phrase
  tune.abc               # structural notes and phrase marks only
  fingering-low-d.json   # holes for each note, relative to the local profile
  ornaments.json         # cut / tap / roll, used from node 6 on
  remarks.json           # one sit-in sentence per evidence id
  ref/                   # slow human phrases, ghost and model, ear lane
```

The whistle profile, and the reading and background answers, are not in the pack.

**Tune.** *The May Morning Dew*, slow, in a setting a new player can reach: room to breathe, and no note until the staircase has taught it. If rights or the ABC are messy, swap the file for *Give Me Your Hand* or another public-domain slow air. The last node’s id stays `air_may_morning_dew` until a manifest change. The title on screen comes from the pack.

**Familiar theme.** After the octave, `hedwig` opens letter notes for *Hedwig’s Theme* in D. John Williams, 2001: not a public-domain source, so the pack has no ABC and no audio. The same page’s key of C is for piano, flute, and recorder. This whistle uses D.

**Hedwig’s Theme** is earlier than that air, and it is not a tune file. After the octave, the `hedwig` node opens letter notes in D. The pack stores the page URL only.

**Pictures versus gestures.** ABC holds pitches, durations, and phrases. The fingering file is what a non-reader is actually following. Ornaments live beside both, and the bare air does not use them. Grading raw grace-note spelling would reward the transcription.

**Reference audio.** One player, slow, clear low-D attack, phrase by phrase. That is what a beginner copies. We do not ask them to imitate a synthesized whistle, and we do not ship other people’s commercial recordings. If the book’s files are in `book/` on this machine, Hear plays the matching tracks. On the door that thickens the cut, the tap, the roll, and the octave. Settings can skip ahead to the part that step uses. When those files are absent, a wav in the pack plays, and when that is absent too, Hear plays the pack’s notes as plain tones. The card says when the sound is only the notes. Hedwig’s Theme stays without a melody.

---

## The book on the table

*The Low Whistle Book* (Hannigan and Ledsam, ISBN 9780952530510) is the external syllabus for wave 2. Edition note, to check against the copy on the table before pointer strings freeze: published contents run about 107 pages (a later printing is sold shorter, same ISBN family). The spine below is that contents list.

| Chapter | What it contributes |
| --- | --- |
| Getting Started | Hold, then notes in the book’s order: B, A, G, F♯, E, D, C♯, octave D′, then C natural, G♯, E♭/F natural, B♭, and scales. The book begins on B because three fingers seal before six do. |
| Notes in Practice | Breathing, starting a note, controlling it. |
| First Tones | *Amazing Grace*, *Skye Boat Song*, *Drops of Brandy*, *The Winster Gallop*, breath inside a tune. |
| Rolls | A short guide, standard roll, short roll, then *The Green Gates* and *Melancholy Martin*. |
| More ornament | Slide, cran, double tap, C♯ roll, vibrato, triplet. |
| Common mistakes | A fault list, not a tune. Becomes the remark bank. |
| Terrific Tones | Three shelves: beginner tones, improver tunes, players’ tunes. *Down by the Salley Gardens* is in this tune list (also spelled Sally / Sally Garden). |
| Whistory, transposition, discography | History of the low whistle, changing whistle instead of fingering, and a listening list. |

Chapter labels in the app are pointers. Settings, diagrams, prose, and audio in the packs are ours.

### Remark bank (from Common mistakes)

One sentence, then the loop opens. Still no percentage.

- Leak on the low notes.
- Early break into the octave.
- Tongue on every note.
- Cut that became a note.
- Second harmonic jumping the octave.
- C natural that did not seal.
- Breath that chops the line.

### Whistory, transposition, discography

- **Whistory** is a short page in our words beside the profile: the low D as its own tube. No paste of the chapter.
- **Transposition** stays a refusal for the book’s table. Other keys are other whistles. The product remains low D only. That table is mentioned; it is not rebuilt as a second instrument. A melody someone brings in is different: the file’s pitches are moved onto this tube, and the card says how far.
- **Discography** is a list of names to hear elsewhere. No streamed audio, no book-CD rips.

---

## The two songs

### Down by the Salley Gardens

The book’s song and the second air.

- The air is the traditional *Maids of the Mourne Shore* / *The Mourne Shore* (O’Neill and other early collections; often printed in F and transposed). Yeats’s poem is the words. Screen title uses *Down by the Salley Gardens*; the manifest keeps the other names in `aka`.
- It is a D-major song air: long notes, breath at the end of a line, little ornament. That is the same job as *The May Morning Dew*, with words as a memory of the phrase.
- Our setting comes from a named public-domain source, simplified so the phrase stays in the range the staircase has taught, slow enough to copy. It is not the book’s arrangement and not a commercial recording.
- Words show after the line has been heard, aligned to phrase marks, and can hide. Singing stays offered and ungraded. This is not the reel that shares the title.

### As I Roved Out

Not in the book’s published tune list. Still the second song, because it is the reason the accidental chapter exists.

- The song is the Fermanagh one (Roud 3479), the Brigid Tunney line, in A dorian. It is not a polka and not another “as I roved out one morning.”
- A dorian on a D whistle needs C natural. The sung air often opens high. The pack is a low-D setting a new player can hold: the body of the song in the first octave, and one phrase that crosses the break on purpose. The second octave stops being only the octave-D drill.
- Words are the traditional opening as phrase memory. A full text ships only from a source we can name as public domain. Planxty, Irvine, and later commercial settings are not the model and not the audio.
- If a bonus track on a particular copy of the book is this song, the pack still uses our setting.

---

## Pack chain (wave 2)

Same listen states, same profile, same folder shape as `pack/may-morning-dew/`. One tune, one listen target. The front door is still the first pack. A shelf appears only after *Down by the Salley Gardens* is settled.

```
pack/<id>/
  manifest.json          # plus wave, after, book_ref, shelf, rights, aka
  phrases.json           # chunks the listen loop expects
  tune.abc               # when the pack has a tune
  fingering-low-d.json
  ornaments.json         # when gestures are marked
  remarks.json
  words.json             # song packs: phrase by phrase, hideable
  ref/                   # our slow human phrases only
```

Manifest fields added, none of them a score:

| Field | Meaning |
| --- | --- |
| `wave` | `1` or `2` |
| `after` | The node or pack that must be settled before this pack opens |
| `book_ref` | A chapter name (pointer only) |
| `shelf` | `path`, `beginner`, `improver`, or `players` |
| `rights` | `pd`, `licence`, `brought`, or `page_only`. `brought` is a folder on this machine, not a shipped pack |
| `licence` | The licence, when there is one |
| `author` | The author, when named |
| `site` | A page to look at. The app does not fetch it |
| `placement` | How a brought melody was moved onto this whistle |
| `aka` | Other titles for the same air |

| Order | Pack | Book place | What it asks |
| --- | --- | --- | --- |
| 0 | `may-morning-dew` | Outside the book. Stays the door. | Wave 1, unchanged. |
| 1 | `book-staircase` | Getting Started | B, A, G, F♯, E, D, C♯, octave D′, each heard then found. Low D is already known. No new air. |
| 2 | `salley-gardens` | Terrific Tones, brought forward as the second air | The song, line by line, bare. A cut only where the pack marks it, and only if wave 1’s cut has settled. Pictures can hide. |
| 3 | `c-natural` | Getting Started, the accidental | One fingering that speaks on this whistle, inside a phrase, checked once. Half-hole and a second cross-fingering wait until they disagree on this stick. |
| 4 | `as-i-roved-out` | Not in the book. Follows the accidental. | A dorian song. C natural in the line. One phrase over the break. |
| 5 | `amazing-grace`, `skye-boat-song` | First Tones | Two more slow tunes in D, public domain, our settings. Breath marked in the line. |
| 6 | `drops-of-brandy`, `winster-gallop` | First Tones, after both songs | Slip jig and galop, slowed. Pulse comes from the pack. The metronome stays off the airs. |
| 7 | `green-gates`, `melancholy-martin` | Rolls | Standard roll, then short roll, each on a long note first. Ship only after a public-domain source is named. |
| 8 | Gesture packs | More ornament | Slide, cran, double tap, triplet. Same contract as cut, tap, and roll: demonstrated, then marked, and the detector may stay silent. Vibrato is named and almost never remarked on. |
| 9 | Shelves | Terrific Tones | Beginner, improver, players. An ordered index of packs, not the first screen. |

Chain order:

```
may-morning-dew → book-staircase → salley-gardens → c-natural
  → as-i-roved-out → First Tones → Rolls → More ornament → Shelves
```

ABC or a MIDI brought in from outside is the desk, after *The May Morning Dew* is settled. It is one folder under `teacher/`, the same phrase loop, not a library and not the first screen. The folder names what is known: a public-domain source, a licence and an author, or nothing. When nothing is known, the card says the copyright is not established, and the file stays on this machine. The app does not fetch the file. A piano MIDI already names each pitch. Those pitches are moved into this whistle’s range, and the card says how far. Where two notes sound together, the higher one is kept. A note that is still not on a low D is named, and that folder is skipped. The door still loads. The screen says why, in one sentence. A `page_only` title still has no notes and no audio in the shipped packs. The Session is a way to find a later pack, by deep link, not an API inside the lesson. Someone who cannot yet play *The May Morning Dew* does not need a tune library.

---

## Rights ledger

What the app ships, and what a person brings in, are not the same thing.

A tune in `pack/` names its source when that source is public domain (O’Neill, Petrie, Joyce, or another collection old enough to say so). When a licence or an author is known, the pack says that instead of pretending. The setting is written for this pack. Being in the book is not a licence to ship someone else’s arrangement or recording.

A folder under `teacher/` is not shipped. The desk opens it after *The May Morning Dew*. It says what is known:

- public domain, with the collection or the site named
- a licence, and the author when the author is named
- or nothing established. The card then says the copyright is not established, and the file stays on this machine

The app does not download a file onto the desk. It points at a place to look, including the [Traditional Music Library song MIDIs](https://www.traditionalmusic.co.uk/song-midis/songs-midis.html). The person saves a file and puts the folder here. Words for a song already on the path are a different folder: `songs/library/`, filled only when someone runs `songs/ingest.py`. That copy stays on this machine. See [songs/README.md](songs/README.md). A piano MIDI already names each pitch. The app reads those pitches and moves the melody into this whistle’s range, and says how far it moved. A note the whistle still cannot play is named, and the folder stays shut.

**`page_only`** — the index can say they are in the book; the shipped app has no notes and no audio. *Hedwig’s Theme* (John Williams) is the same rule, earlier: the node opens [letter notes in D](https://www.irish-folk-songs.com/hedwigs-theme-piano-flute-and-tin-whistle-notes.html) and ships neither the melody nor audio.

- *The Lonesome Boatman* (Finbar Furey)
- *Wissahickon Drive* (Liz Carroll)
- *Green Fields of America* (Liam O’Flynn)
- Hannigan’s own tunes: *Northumberland Farewell*, *Spike’s Lane*, *The Blue Remembered Hills*, *The Tabletop Hornpipe*, *Pipe Major Euan Husami*
- *Phryjig* (Mike Cosgrave)
- *The Man of Aran* (Daragh De Brun)
- *12 Long Years* (Mark Bradley)

**Off the shipped shelf until a source is named:** *The Butterfly*, *The Mountain Road*. A person can still bring a file in on the desk.

**On the page only:** *She Moved Through the Fair* — the famous setting is an arrangement.

---

## What stays on the device

Progress:

```
{ node_id, state_reached, as_of }
```

`state_reached` is `started` or `settled`. `started` means an attempt opened. `settled` means the loop heard the lesson’s target, or the player marked the part and moved on. A heard settle stays heard. Each whistle keeps its own progress. No WAV in this record. A review, and a single note opened from a phrase fault, do not write progress. The last take is memory only, and it is dropped with the attempt.

Whistle profile:

```
{ profile_id, label, break_hz, rms_floor, cal_as_of, reads, background, cnat_fingering, warmup_on_launch, lesson_packs }
```

`label` is the nickname. `break_hz` and `rms_floor` are written when `first_sound` settles, not before the player can play. `reads` and `background` choose pictures and wording. `warmup_on_launch` is whether the hands-and-breath pass shows the next time this profile opens the app. It defaults to showing the pass. `cnat_fingering` is written when the C-natural pack settles: which fingering held on this stick. Half-hole and a second cross-fingering wait until they disagree here. `break_hz` and `rms_floor` stay the tuning. `lesson_packs` is songs this whistle has added from packs that are already open. The path order does not change. A page-only title is not on that list.

Several profiles may exist. Practice names the one in hand.

---

## How it is built

Local-first app: a Tauri shell around an offline interface. How the processes, the listen engine, and the pack loader fit together is in [ARCHITECTURE.md](ARCHITECTURE.md). Pitch and breath run on device. The sounding state can follow a note (on the order of 20–40 ms). Phrase remarks are computed when the attempt ends.

The practice screen does not need a server, an account, or a connection. Packs are files. A Python bench may be used to check the on-device tracker against labeled takes from real beginners and from clean players. That bench is not in the app. A neural model is not a runtime dependency.

The beginner corpus has to include the sounds a new player actually makes: all holes leaking, a squeak, no tone at all, a tongue on every note, a cut that lasts half a beat. A corpus of only tidy high-D transfers will tune the remarks to the wrong person.

Build order:

1. Hole picture, `first_sound`, profile written from a settled hold, listen states on that hold and on the octave break.
2. Prove the gate with someone who did not already play whistle: low D, early break, one remark, retry, stable target, first phrase.
3. Staircase and `on_the_breath`, with the reading toggle changing pictures only.
4. Bare air.
5. Cut, tap, and roll as gestures that may abstain, then the air with those marks.
6. Only then another pack — wave 2, in chain order, starting with `book-staircase` and `salley-gardens`.

---

## Left as they were

Wave 1’s listen nodes, plus the early `hedwig` page, the four listen states, calibration only after low D will hold, no accounts, no upload, no score, no camera, no other whistle keys. The Session remains a way to find a later pack, not an API inside the lesson.

Hand size is a physical limit. The first screen can say that some low Ds are too big for some hands, and that a smaller-holed polymer low D is a fine instrument to learn on. The app does not become a shop.

Shorter copy when `background` is `high_d`. Still the beginner path, because the low D is a different tube. Dance-tune pulse taken from the pack, once the tune is a jig or a galop. Airs stay free of a metronome. A folder a teacher wrote is welcome; the app still does not upload a take.

---

## Left out

- A course that assumes they already play whistle, read staff, or know the words cut, tap, and roll.
- Points, streaks, levels, and exam chrome on the sounding screen.
- A cloud tutor, accounts, multiplayer, and audio upload.
- A tune library as the thing they see first.
- Concert pitch baked into content.
- Letter grades.
- An ornament judgment on every attempt, whether or not the contour was heard.
- Camera fingering, a custom wired whistle, and a generated band under a beginner.
- Other whistle keys, and other instruments. Low D, D fingering, from the first note.
- Shipping the book’s arrangements, CD tracks, or download audio as pack content.
- Treating “is in the book” as a license to ship notes or audio.
- Rebuilding the book’s transposition table as a second instrument inside the app.
- Scoring the poem, or requiring the player to sing.

---

## Decisions taken

| Question | Call |
| --- | --- |
| Who is it for? | A complete beginner to the whistle. Reading and other musical experience change the pictures and the wording. They are not prerequisites, and they are not separate tracks. |
| What about someone who already plays high D? | Same path, shorter copy on the notes they can already find. The spine’s ex–high-D track is not the product. |
| What is the first session? | Name the whistle, warm the hands and the breath, learn to hold low D, and only then store a calibration. A listening test before the first lesson is the wrong door. |
| How much curriculum in wave 1? | The listen path, one air bare and then ornamented, and an early page for *Hedwig’s Theme* letter notes in D. The melody is not in the pack. |
| What is wave 2? | Same loop, same profile, packs shaped by *The Low Whistle Book*. Front door stays *The May Morning Dew*. |
| First note: book or app? | Wave 1 opens on low D. Wave 2 walks the book’s B→A→G… staircase after that hold exists. |
| The two song airs? | *Down by the Salley Gardens* (in the book; our PD setting), then *As I Roved Out* (not in the book; A dorian; forces C natural). |
| Are ornaments in the first pack? | Yes, after the player can play the first phrase. Taught as new gestures. The detector may abstain. |
| What is the grade? | After calibration: this whistle’s break and breath, plus gesture timing. Before calibration: “is low D there.” Sit-in language. `started` / `settled` in the store. |
| Where does it run? | On device, offline, local shell. |
| Where do tunes come from? | One pack first. Then a chain of packs. A library is not the front door. A shipped tune cites a public-domain source, or states a licence and an author. A folder on this machine can be less certain, and the card says so. The book is not a licence. |
| Teacher? Book? | Companions. No share action and no upload in P0. Chapter labels are pointers only. |
| Look? | Session table. A low D with the holes visible, warm wood, quiet type. The first card is the instrument, not a form. |

The musical claim: a person who has never played whistle can learn low D as sound, fingers, breath, and one slow air, whether or not they read — then walk the book’s syllabus on this tube, through two song airs and the accidental that makes *As I Roved Out* possible. The engineering claim: the listen loop stays quiet until a note exists, then judges that note against the whistle in the room.
