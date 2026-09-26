import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory

from songs.ingest import HostGate, assemble, delay_for, ingest, polite_get
from songs.parse import build_midi


PAGE = """
<h2>Practice Air</h2>
<a href="midi/PRACTICE.mp3">Download MP3</a>
<a href="https://evil.example/nope.mid">nope</a>
<a href="midi/PRACTICE.midi">Download Midi</a>
<pre>Practice Air

The practice line is only a fixture
It is not from either site

Second verse of the fixture
Still invented
</pre>
"""

INDEX_B = """
<a href="songs-midis-A.html">A</a>
<a href="Practice_Air.htm">Practice Air</a>
<a href="https://evil.example/x.htm">Away</a>
"""


class Fake:
    def __init__(self, pages: dict):
        self.pages = pages
        self.calls: list[str] = []
        self.sleeps: list[float] = []

    def get(self, url: str) -> bytes:
        self.calls.append(url)
        page = self.pages.get(url)
        if isinstance(page, list):
            item = page.pop(0)
            if isinstance(item, Exception):
                raise item
            return item
        if page is None:
            raise LookupError(url)
        return page

    def sleep(self, seconds: float) -> None:
        self.sleeps.append(seconds)


def _library_pages() -> dict:
    base = "https://www.traditionalmusic.co.uk/song-midis/"
    return {
        base + "songs-midis-A.html": b"<a href='Other.htm'>Other Song</a>",
        base + "songs-midis-P.html": b"<a href='Nothing.htm'>Nothing Here</a>",
        base + "songs-midis-B.html": INDEX_B.encode(),
        base + "Practice_Air.htm": PAGE.encode(),
        base + "midi/PRACTICE.midi": build_midi([(62, 1, 0), (64, 1, 0), (66, 1, 0), (67, 1, 0)]),
    }


class Ingest(unittest.TestCase):
    def test_fetches_once_keeps_the_sheet_and_ignores_other_hosts(self):
        fake = Fake(_library_pages())
        songs = [{"pack_id": "practice-air", "queries": ["Practice Air"]}]
        names = ["songs-midis-A.html", "songs-midis-P.html", "songs-midis-B.html"]
        with TemporaryDirectory() as tmp:
            dest = Path(tmp)
            report = ingest(songs, dest, fake.get, fake.sleep, "2026-09-26", names=names)
            self.assertEqual(report[0]["status"], "saved")
            self.assertGreaterEqual(report[0]["lyrics"], 2)
            self.assertGreaterEqual(report[0]["melody"], 4)
            sheet = json.loads((dest / "practice-air.json").read_text())
            self.assertEqual(sheet["licence"], "personal")
            self.assertNotIn("<", sheet["credit"])
            self.assertIn("Not for performance", sheet["credit"])
            self.assertIn("does not guarantee", sheet["credit"])
            self.assertIn("The practice line is only a fixture", "\n".join(sheet["verses"]))
            self.assertTrue((dest / "midi" / "practice-air.mid").read_bytes().startswith(b"MThd"))
            self.assertFalse(any("evil.example" in call for call in fake.calls))
            self.assertFalse(any(call.endswith(".mp3") for call in fake.calls))
            calls = len(fake.calls)
            again = ingest(songs, dest, fake.get, fake.sleep, "2026-09-26", names=names)
            self.assertEqual(again[0]["status"], "kept")
            self.assertEqual(len(fake.calls), calls)

    def test_different_pages_are_named_as_different(self):
        tml = {
            "origin": "tml",
            "title": "Practice Air",
            "url": "https://www.traditionalmusic.co.uk/song-midis/Practice_Air.htm",
            "score": 1,
            "verses": ["One line of a fixture\nAnd the next"],
            "melody": {"melody": [], "playable": False, "folded": False, "trimmed": False},
            "midi_url": "",
        }
        mudcat = {
            "origin": "mudcat",
            "title": "Practice Air",
            "url": "https://mudcat.org/@displaysong.cfm?SongID=99",
            "score": 1,
            "verses": [],
            "melody": {
                "melody": [
                    {"note": "D4", "beats": 1},
                    {"note": "E4", "beats": 1},
                    {"note": "F#4", "beats": 1},
                    {"note": "G4", "beats": 1},
                ],
                "playable": True,
                "folded": False,
                "trimmed": False,
            },
            "midi_url": "https://mudcat.org/midi/midifiles/fixture.mid",
        }
        sheet = assemble("practice-air", ["Practice Air"], tml, mudcat, "2026-09-26")
        self.assertIsNotNone(sheet)
        self.assertIn("may not be the same version", sheet["credit"])
        self.assertIn("stay copyrighted", sheet["credit"])
        self.assertLessEqual(len(sheet["credit"]), 1200)
        self.assertEqual(sheet["licence"], "personal")
        self.assertTrue(sheet["playable"])
        self.assertIn("mudcat.org", sheet["also_url"])

    def test_polite_retry_and_the_mudcat_gap(self):
        self.assertEqual(delay_for("https://mudcat.org/midi/midibrowseall.cfm"), 10)
        self.assertEqual(delay_for("https://www.traditionalmusic.co.uk/song-midis/songs-midis.html"), 2)
        fake = Fake({"https://mudcat.org/midi/x": [TimeoutError("one"), TimeoutError("two"), b"<html></html>"]})
        data = polite_get("https://mudcat.org/midi/x", HostGate(), fake.get, fake.sleep)
        self.assertEqual(data, b"<html></html>")
        self.assertEqual(len(fake.calls), 3)
        self.assertIn(1.5, fake.sleeps)
        gate = HostGate()
        fake.pages["https://mudcat.org/midi/y"] = b"ok"
        fake.pages["https://www.mudcat.org/midi/z"] = b"ok"
        polite_get("https://mudcat.org/midi/y", gate, fake.get, fake.sleep)
        polite_get("http://www.mudcat.org/midi/z", gate, fake.get, fake.sleep)
        self.assertIn(10.0, fake.sleeps)
        self.assertEqual(fake.calls[-1], "https://www.mudcat.org/midi/z")
        before = len(fake.calls)
        with self.assertRaises(ValueError):
            polite_get("https://evil.example/x", HostGate(), fake.get, fake.sleep)
        self.assertEqual(len(fake.calls), before)


if __name__ == "__main__":
    unittest.main()
