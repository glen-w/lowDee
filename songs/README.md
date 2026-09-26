Type: GUIDE
Authority: Where a personal copy of lyrics and a melody MIDI can sit, and what that copy is allowed to be. The practice screen reads this folder. It does not fetch it.

# Lyrics and melody, on this machine

The song packs already carry a short line of words. A longer set, and a melody to hear beside it, can be fetched into `songs/library/`. Git ignores that folder. A build does not bundle it. The sitting does not call the network. Nothing in the app uploads the files.

```bash
python3 songs/ingest.py
python3 songs/ingest.py --pack salley-gardens
python3 songs/ingest.py --title "The Parting Glass"
python3 songs/ingest.py --dry-run
python3 songs/ingest.py --refresh
```

`--pack` limits the list in `wanted.json` (Salley Gardens, As I Roved Out, Amazing Grace, Skye Boat Song). `--title` fetches one more name. A second run leaves a saved sheet where it is, unless `--refresh` is set. `--dry-run` still downloads into the cache and does not write the sheet.

The panel is Lyrics, on a phrase, after the line has been heard. Hear this melody plays plain tones of that fetched line. It is not this pack’s setting, and it is not graded. Hide lyrics puts it away. If the folder is empty, the button is not there.

## Where it comes from

Two sites, and only these:

- [Traditional Music Library, song MIDIs](https://www.traditionalmusic.co.uk/song-midis/songs-midis.html). Richard Robinson’s collection of traditional and folk songs with lyrics and melody MIDI. The arrangements and the MIDI files are his copyright. He offers them for personal, non-commercial music training, including passing a file to a musician friend for that same personal training. The lyrics are for study and scholarship. He says many of the songs are traditional or public domain, and that this cannot be guaranteed. Before a public performance or any commercial use, he says to check the copyright. His own note is on the [about page](https://www.traditionalmusic.co.uk/bgabout.htm). A website that republishes a MIDI also needs an acknowledgement and a link to his home page. This app does not republish the files.
- [Mudcat Café](https://mudcat.org/), the Digital Tradition. Dick Greenhaus and friends collected the words and tunes and described the database as free to use, with a credit appreciated, because they do not claim to own the songs. Copyrighted material in the Digital Tradition stays copyrighted, whether or not a notice is printed, and is not cleared for commercial use. The site’s own line is that it is not a copyright resource. Original material on the site is copyright the Mudcat Café Music Foundation. The rules are on [Digital Tradition Rules](https://mudcat.org/DigiTrad-rules.cfm) and [About the Digital Tradition](https://mudcat.org/aboutdigitrad.cfm). A full copy of the old database is on [the download page](https://mudcat.org/download.cfm). This ingest does not take the forum, and it does not take that whole archive. It reads the MIDI list and, when a title matches, one song page.

The script waits 2 seconds between Traditional Music Library requests and 10 seconds between Mudcat requests. Mudcat’s `robots.txt` asks for that gap. A failed request is tried again. A 401, 403, or 429 is tried once more after a longer pause, and then the Traditional Music Library walk stops. A title whose index never arrived is `unreadable`, not a miss, and the other letters are not requested. A page that is not one of those two hosts is refused. A file that is not a MIDI is not kept as a melody. A weak title match is not saved. The sheet records the page it came from, the date, and `licence: personal`. The app ignores any other licence.

## What this is not

Personal practice on this machine. Not a performance licence, not a recording licence, and not permission to pass the files on. Not this pack’s setting: the words and the melody may be a different version from the one in the lesson, and when they came from two different pages the sheet says so. Not a promise that a song is in the public domain because it is old, or because a forum posted it. If a use needs a clearance, check the song with the people who hold it.
