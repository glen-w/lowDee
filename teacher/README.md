Type: GUIDE
Authority: Where a teacher folder sits on this machine. What may ship in a pack is decided in [CONCEPT.md](../CONCEPT.md). How the loader treats that folder is in [ARCHITECTURE.md](../ARCHITECTURE.md).

A folder a teacher wrote, or a MIDI the player saved, can live here. One tune per directory. Nothing here is uploaded. The desk opens it after *The May Morning Dew* is settled.

Two shapes:

- The same manifest as `pack/`. `rights` is `pd` with `source` or `site` named, `licence` with `licence` set and `author` when known, or `brought` when the copyright is not established. An empty `rights` is read as `brought`.
- One `.mid` file, and an optional `notice.json` with `title`, `rights`, `source`, `author`, `licence`, and `site`. The pitches in the file are moved onto this low D. The card says how far, and says when the copyright is not established.

A place to look is the [Traditional Music Library](https://www.traditionalmusic.co.uk/song-midis/songs-midis.html). Download the file yourself. What may ship in `pack/` is decided in [CONCEPT.md](../CONCEPT.md). How the loader treats this folder is in [ARCHITECTURE.md](../ARCHITECTURE.md). A folder the loader skips is named on the desk, in one sentence.
