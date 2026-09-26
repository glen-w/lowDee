import struct
import unittest

from songs.parse import (
    best_match,
    build_midi,
    encode_vlq,
    indexes_for_letters,
    letters_for,
    mudcat_browse,
    mudcat_song,
    parse_midi,
    prepare_url,
    score_title,
    tml_index_links,
    tml_song,
)


class Titles(unittest.TestCase):
    def test_spelling_and_near_misses(self):
        self.assertEqual(score_title("Salley Gardens", "Down by the Sally Gardens"), 0.9)
        self.assertEqual(score_title("Amazing Grace", "Amazing Grace (hymn)"), 1.0)
        self.assertLess(score_title("Sally Brown", "Down by the Salley Gardens"), 0.9)
        self.assertLess(score_title("Skye Boat Song", "Skyehigh"), 0.9)
        self.assertGreaterEqual(score_title("The Skye Boat Song", "Skye Boat Song"), 0.9)

    def test_best_match_skips_a_lookalike(self):
        rows = [
            {"title": "Sally Brown"},
            {"title": "Down by the Sally Gardens"},
        ]
        hit = best_match(["Salley Gardens"], rows)
        self.assertIsNotNone(hit)
        self.assertEqual(hit[0]["title"], "Down by the Sally Gardens")

    def test_letters_cover_the_important_words(self):
        letters = letters_for(["Down by the Salley Gardens"])
        self.assertIn("D", letters)
        self.assertIn("S", letters)
        self.assertNotIn("B", letters)
        names = indexes_for_letters(letters, ["songs-midis-D1.html", "songs-midis-S2.html", "songs-midis-B1.html"])
        self.assertEqual(names, ["songs-midis-D1.html", "songs-midis-S2.html"])


class Html(unittest.TestCase):
    def test_index_keeps_song_pages_only(self):
        html = """
        <a href="songs-midis-A.html">A</a>
        <a href="../index.html">Home</a>
        <a href="https://evil.example/x.htm">Away</a>
        <a href="Fish_and_Chips.htm">Fish &amp; Chips</a>
        <a href="Practice_Air.htm">Practice Air</a>
        """
        rows = tml_index_links(html)
        self.assertEqual([row["title"] for row in rows], ["Fish & Chips", "Practice Air"])
        self.assertEqual(rows[0]["href"], "Fish_and_Chips.htm")

    def test_song_page_drops_chrome_title_and_mp3(self):
        html = """
        <h1>Traditional &amp; Folk Songs with lyrics</h1>
        <h2>Practice Air</h2>
        <a href="midi/PRACTICE.mp3">Download MP3</a>
        <a href="midi/PRACTICE.midi">Download Midi</a>
        <a href="https://evil.example/nope.mid">nope</a>
        <pre>adsbygoogle download midi</pre>
        <pre>Practice Air

The practice line is only a fixture
It is not from either site

-----
        </pre>
        """
        song = tml_song(html)
        self.assertEqual(song["title"], "Practice Air")
        self.assertEqual(song["verses"], ["The practice line is only a fixture\nIt is not from either site"])
        self.assertEqual(song["midi_hrefs"], ["midi/PRACTICE.midi", "https://evil.example/nope.mid"])

    def test_mudcat_row_and_lyric_block(self):
        browse = """
        <tr><td><A HREF="midifiles/fixture.mid">Practice Air</A> <font>(a note)</font></td>
        <td><a href="../@displaysong.cfm?SongID=99">DT</a></td></tr>
        """
        rows = mudcat_browse(browse)
        self.assertEqual(rows[0]["title"], "Practice Air")
        self.assertEqual(rows[0]["song_id"], "99")
        self.assertEqual(rows[0]["href"], "midifiles/fixture.mid")
        page = """
        <hr>
        The fixture stays home<br>
        On a made morning<br>
        <br>
        From a made-up booklet<br>
        @madeup<br>
        filename[ FIXTURE
        <a href="HTTP://www.mudcat.org/media/FIXTURE.MID">CLICK TO PLAY</a>
        """
        song = mudcat_song(page)
        self.assertEqual(song["verses"], ["The fixture stays home\nOn a made morning"])
        self.assertEqual(song["midi_hrefs"], ["HTTP://www.mudcat.org/media/FIXTURE.MID"])

    def test_refuses_a_url_off_the_two_sites(self):
        self.assertTrue(prepare_url("http://mudcat.org/midi/midifiles/fixture.mid").startswith("https://mudcat.org/"))
        with self.assertRaises(ValueError):
            prepare_url("https://evil.example/x.mid")
        with self.assertRaises(ValueError):
            prepare_url("https://user:pass@mudcat.org/x")
        with self.assertRaises(ValueError):
            prepare_url("https://mudcat.org/../secret")


class Midi(unittest.TestCase):
    def test_round_trip_and_a_rest(self):
        parsed = parse_midi(build_midi([(62, 1, 0), (64, 1, 1), (66, 1, 0), (67, 1, 0)]))
        self.assertTrue(parsed["playable"])
        self.assertFalse(parsed["folded"])
        self.assertEqual([item["note"] for item in parsed["melody"]], ["D4", "rest", "E4", "F#4", "G4"])

    def test_folds_a_low_note_and_keeps_the_higher_harmony(self):
        low = parse_midi(build_midi([(50, 1, 0), (50, 1, 0), (50, 1, 0), (50, 1, 0)]))
        self.assertTrue(low["playable"])
        self.assertTrue(low["folded"])
        self.assertEqual(low["melody"][0]["note"], "D4")
        body = bytearray()
        body += encode_vlq(0) + bytes([0x90, 60, 64])
        body += encode_vlq(0) + bytes([0x90, 67, 64])
        body += encode_vlq(480) + bytes([0x80, 60, 0])
        body += encode_vlq(0) + bytes([0x80, 67, 0])
        body += bytes([0x00, 0xFF, 0x2F, 0x00])
        track = b"MTrk" + struct.pack(">I", len(body)) + bytes(body)
        header = b"MThd" + struct.pack(">IHHH", 6, 0, 1, 480)
        harmony = parse_midi(header + track)
        self.assertEqual(harmony["melody"][0]["note"], "G4")

    def test_busy_channel_running_status_and_rubbish(self):
        def track(channel: int, pitches: list[int]) -> bytes:
            body = bytearray()
            for pitch in pitches:
                body += encode_vlq(0) + bytes([0x90 | channel, pitch, 64])
                # running status, velocity 0 ends the note after one quarter
                body += encode_vlq(480) + bytes([pitch, 0])
            body += bytes([0x00, 0xFF, 0x2F, 0x00])
            return b"MTrk" + struct.pack(">I", len(body)) + bytes(body)

        quiet = track(0, [60])
        busy = track(1, [62, 64, 66, 67])
        header = b"MThd" + struct.pack(">IHHH", 6, 1, 2, 480)
        parsed = parse_midi(header + quiet + busy)
        self.assertTrue(parsed["playable"])
        self.assertEqual(parsed["melody"][0]["note"], "D4")
        drums = track(9, [36, 38, 42, 46])
        only = b"MThd" + struct.pack(">IHHH", 6, 0, 1, 480) + drums
        self.assertFalse(parse_midi(only)["playable"])
        with self.assertRaises(ValueError):
            parse_midi(b"not midi")
        with self.assertRaises(ValueError):
            parse_midi(build_midi([(62, 1, 0)])[:-8])


if __name__ == "__main__":
    unittest.main()
