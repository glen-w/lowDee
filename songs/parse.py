"""Parsers for a personal lyrics and melody library.

The pages and MIDI files stay on this machine. Nothing here decides that a
song is free to perform or to pass on.
"""

from __future__ import annotations

import re
import struct
from html.parser import HTMLParser
from urllib.parse import urlsplit, urlunsplit

ALLOWED_HOSTS = {
    "www.traditionalmusic.co.uk",
    "traditionalmusic.co.uk",
    "mudcat.org",
    "www.mudcat.org",
}

TML_BASE = "https://www.traditionalmusic.co.uk/song-midis/"
MUDCAT_BROWSE = "https://mudcat.org/midi/midibrowseall.cfm"

MATCH_FLOOR = 0.9
LOW_D = 62
HIGH_D = 86
MAX_MELODY = 160

_NAMES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
_STOP = {"the", "a", "an", "of", "and", "by", "in", "on"}
_CHROME = (
    "adsbygoogle",
    "download midi",
    "download mp3",
    "quick links",
    "mudcat cafe",
    "mailing list",
    "printicon",
)
_NOTE_LINE = re.compile(
    r"^(from |note:|notes:|recorded by|child #|@|filename\[|tune file:)",
    re.I,
)


def index_names() -> list[str]:
    names = ["songs-midis-A.html"]
    for letter, count in (("B", 4), ("C", 3), ("D", 2), ("L", 2), ("S", 4), ("W", 2)):
        names.extend(f"songs-midis-{letter}{i}.html" for i in range(1, count + 1))
    names.extend(f"songs-midis-{letter}.html" for letter in "EFGHIJKMNOPQRTUV")
    names.append("songs-midis-XYZ.html")
    return names


def letters_for(queries: list[str]) -> set[str]:
    letters: set[str] = set()
    for query in queries:
        words = re.findall(r"[A-Za-z]+", query)
        if not words:
            continue
        letters.add(words[0][0].upper())
        for word in words:
            if word.lower() in _STOP or len(word) <= 2:
                continue
            letters.add(word[0].upper())
    return letters


def indexes_for_letters(letters: set[str], names: list[str] | None = None) -> list[str]:
    chosen = []
    for name in names or index_names():
        tag = name.removeprefix("songs-midis-").removesuffix(".html")
        if tag == "XYZ":
            if letters & set("XYZ"):
                chosen.append(name)
            continue
        if tag[:1] in letters:
            chosen.append(name)
    return chosen


def prepare_url(url: str) -> str:
    raw = url.strip()
    parts = urlsplit(raw)
    host = (parts.hostname or "").lower()
    scheme = parts.scheme.lower()
    if scheme == "http" and host in ALLOWED_HOSTS:
        scheme = "https"
    if parts.username or parts.password:
        raise ValueError("refused url")
    if parts.port not in (None, 80, 443):
        raise ValueError("refused url")
    path = parts.path or "/"
    if ".." in path.split("/"):
        raise ValueError("refused url")
    cleaned = urlunsplit((scheme, host, path, parts.query, ""))
    if not allowed_url(cleaned):
        raise ValueError("refused url")
    return cleaned


def allowed_url(url: str) -> bool:
    parts = urlsplit(url)
    if parts.scheme != "https":
        return False
    if parts.username or parts.password:
        return False
    host = (parts.hostname or "").lower()
    if host not in ALLOWED_HOSTS:
        return False
    if parts.port not in (None, 443):
        return False
    if ".." in (parts.path or "").split("/"):
        return False
    return True


def decode_html(raw: bytes) -> str:
    if len(raw) > 2_000_000:
        raise ValueError("too large")
    head = raw[:1200].decode("ascii", "ignore").lower()
    if "charset=utf-8" in head or "charset=\"utf-8\"" in head:
        return raw.decode("utf-8", "replace")
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError:
        text = raw.decode("latin-1")
    return text


def norm_title(value: str) -> str:
    text = value.lower().replace("’", "'").replace("`", "'")
    text = text.replace("salley", "sally").replace("gardens", "garden")
    text = re.sub(r"\([^)]*\)", " ", text)
    text = re.sub(r"[^a-z0-9]+", " ", text)
    return " ".join(text.split())


def score_title(query: str, title: str) -> float:
    left = norm_title(query)
    right = norm_title(title)
    if not left or not right:
        return 0.0
    if left == right:
        return 1.0
    query_words = set(left.split())
    title_words = set(right.split())
    shorter, longer = (
        (query_words, title_words)
        if len(query_words) <= len(title_words)
        else (title_words, query_words)
    )
    if shorter and shorter <= longer and len(shorter) >= 2 and any(len(word) >= 4 for word in shorter):
        ratio = len(shorter) / len(longer)
        return 0.92 if ratio >= 0.45 else 0.9
    if not query_words or not title_words:
        return 0.0
    return len(query_words & title_words) / len(query_words | title_words)


def tie_break(title: str) -> float:
    penalty = 2.0 if "(" in title else 0.0
    penalty += len(norm_title(title).split()) * 0.01
    return penalty


def best_match(queries: list[str], rows: list[dict]) -> tuple[dict, float] | None:
    chosen: tuple[dict, float] | None = None
    chosen_key: tuple[float, float] | None = None
    for row in rows:
        title = str(row.get("title") or "")
        score = max((score_title(query, title) for query in queries), default=0.0)
        key = (-score, tie_break(title))
        if chosen is None or chosen_key is None or key < chosen_key:
            chosen = (row, score)
            chosen_key = key
    if chosen and chosen[1] >= MATCH_FLOOR:
        return chosen
    return None


class _Links(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.links: list[tuple[str, str]] = []
        self._href: str | None = None
        self._buf: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag.lower() != "a":
            return
        href = dict(attrs).get("href")
        self._href = href
        self._buf = []

    def handle_endtag(self, tag: str) -> None:
        if tag.lower() != "a" or self._href is None:
            return
        text = re.sub(r"\s+", " ", "".join(self._buf)).strip()
        self.links.append((self._href, text))
        self._href = None
        self._buf = []

    def handle_data(self, data: str) -> None:
        if self._href is not None:
            self._buf.append(data)


_SONG_HREF = re.compile(r"^[A-Za-z0-9][A-Za-z0-9_().' -]*\.html?$")


def tml_index_links(html: str) -> list[dict]:
    parser = _Links()
    try:
        parser.feed(html)
        parser.close()
    except Exception:
        return []
    rows = []
    for href, title in parser.links:
        href = href.strip()
        if not _SONG_HREF.match(href):
            continue
        if href.lower().startswith("songs-midis"):
            continue
        if not re.search(r"[A-Za-z]", title):
            continue
        if len(title) > 160:
            continue
        rows.append({"title": title, "href": href})
    return rows


def _plain(fragment: str) -> str:
    text = re.sub(r"(?is)<script.*?</script>", " ", fragment)
    text = re.sub(r"(?is)<style.*?</style>", " ", text)
    text = re.sub(r"(?is)<[^>]+>", " ", text)
    text = re.sub(r"\s+", " ", text).strip()
    return text.replace("\x00", "")


def _heading(html: str) -> str:
    for tag in ("h2", "h1"):
        match = re.search(rf"(?is)<{tag}[^>]*>(.*?)</{tag}>", html)
        if not match:
            continue
        text = _plain(match.group(1))
        if text and "traditional" not in text.lower() and len(text) < 160:
            return text
    return ""


def _chrome(text: str) -> bool:
    low = text.lower()
    return any(piece in low for piece in _CHROME)


def verses_from_lines(lines: list[str]) -> list[str]:
    groups: list[list[str]] = []
    current: list[str] = []
    for line in lines:
        if not line.strip():
            if current:
                groups.append(current)
                current = []
            continue
        current.append(line.strip())
    if current:
        groups.append(current)
    verses: list[str] = []
    for group in groups:
        if len(group) > 12:
            for start in range(0, len(group), 8):
                verses.append("\n".join(group[start : start + 8]))
        else:
            verses.append("\n".join(group))
    clean = []
    for verse in verses:
        if _chrome(verse) or "<" in verse or ">" in verse:
            continue
        if len(verse) > 1800:
            continue
        clean.append(verse)
        if len(clean) >= 40:
            break
    return clean


def _midi_hrefs(html: str) -> list[str]:
    found = re.findall(r"""href\s*=\s*["']([^"']+\.(?:midi|mid))["']""", html, re.I)
    kept = []
    for href in found:
        low = href.lower()
        if low.endswith(".mp3"):
            continue
        if "8notes.com" in low:
            continue
        kept.append(href.strip())
    return kept


def tml_song(html: str) -> dict:
    title = _heading(html)
    blocks = re.findall(r"(?is)<pre[^>]*>(.*?)</pre>", html)
    best_lines: list[str] = []
    for block in blocks:
        lines = []
        notes = []
        for raw in block.replace("\r\n", "\n").replace("\r", "\n").split("\n"):
            line = _plain(raw)
            if not line or set(line) <= {"-", " "}:
                lines.append("")
                continue
            if title and norm_title(line) == norm_title(title):
                continue
            if _NOTE_LINE.match(line):
                notes.append(line)
                continue
            lines.append(line)
        text = "\n".join(lines)
        if _chrome(text):
            continue
        if sum(ch.isalpha() for ch in text) > sum(ch.isalpha() for ch in "\n".join(best_lines)):
            best_lines = lines
    verses = verses_from_lines(best_lines)
    return {
        "title": title,
        "verses": verses,
        "midi_hrefs": _midi_hrefs(html),
    }


def mudcat_browse(html: str) -> list[dict]:
    pattern = re.compile(
        r"""<a\s+href="(midifiles/[^"]+\.mid)"[^>]*>(.*?)</a>(.*?)</td>\s*<td>(.*?)</td>""",
        re.I | re.S,
    )
    rows = []
    for midi_href, title_html, comment_html, cell in pattern.findall(html):
        title = _plain(title_html)
        if not title or len(title) > 160:
            continue
        song_id = ""
        match = re.search(r"SongID=(\d+)", cell, re.I)
        if match:
            song_id = match.group(1)
        comment = _plain(comment_html)
        if len(comment) > 240:
            comment = comment[:240]
        rows.append(
            {
                "title": title,
                "href": midi_href,
                "song_id": song_id,
                "comment": comment,
            }
        )
    return rows


def mudcat_song(html: str) -> dict:
    marker = html.lower().find("filename[")
    head = html if marker < 0 else html[:marker]
    hr = head.lower().rfind("<hr")
    chunk = head[hr:] if hr >= 0 else head
    parts = re.split(r"(?i)<br\s*/?>", chunk)
    lyrics: list[str] = []
    notes: list[str] = []
    for part in parts:
        line = _plain(part)
        if not line or _chrome(line):
            continue
        if _NOTE_LINE.match(line) or line.lower().startswith("child #"):
            if lyrics:
                notes.append(line)
            continue
        lyrics.append(line)
    hrefs = _midi_hrefs(html[marker:] if marker >= 0 else html)
    return {
        "verses": verses_from_lines(lyrics),
        "notes": notes[:4],
        "midi_hrefs": hrefs,
    }


def _vlq(data: bytes, index: int) -> tuple[int, int]:
    value = 0
    start = index
    while True:
        if index >= len(data) or index - start > 4:
            raise ValueError("truncated midi")
        byte = data[index]
        index += 1
        value = (value << 7) | (byte & 0x7F)
        if not byte & 0x80:
            return value, index


def _note_name(pitch: int) -> str:
    return f"{_NAMES[pitch % 12]}{pitch // 12 - 1}"


def _fold(pitch: int) -> tuple[str | None, int]:
    shifted = 0
    while pitch < LOW_D and shifted < 5:
        pitch += 12
        shifted += 1
    while pitch > HIGH_D and shifted < 5:
        pitch -= 12
        shifted += 1
    if pitch < LOW_D or pitch > HIGH_D:
        return None, shifted
    return _note_name(pitch), shifted


def _quantize(quarters: float) -> float:
    steps = round(quarters * 4) / 4
    if steps < 0.25:
        steps = 0.25
    if steps > 4:
        steps = 4
    return steps


def _read_track(chunk: bytes, division: int) -> dict[int, list[tuple[float, float, int]]]:
    index = 0
    tick = 0
    running: int | None = None
    sounding: dict[tuple[int, int], int] = {}
    found: dict[int, list[tuple[float, float, int]]] = {}
    while index < len(chunk):
        delta, index = _vlq(chunk, index)
        tick += delta
        if index >= len(chunk):
            break
        status = chunk[index]
        if status & 0x80:
            index += 1
            running = status
        elif running is None:
            raise ValueError("bad midi")
        else:
            status = running
        if status == 0xFF:
            if index >= len(chunk):
                break
            kind = chunk[index]
            index += 1
            length, index = _vlq(chunk, index)
            index += length
            if kind == 0x2F:
                break
            continue
        if status in (0xF0, 0xF7):
            length, index = _vlq(chunk, index)
            index += length
            running = None
            continue
        kind = status & 0xF0
        channel = status & 0x0F
        if kind in (0xC0, 0xD0):
            index += 1
            continue
        if kind not in (0x80, 0x90, 0xA0, 0xB0, 0xE0):
            raise ValueError("bad midi")
        if index + 1 >= len(chunk):
            break
        data1 = chunk[index]
        data2 = chunk[index + 1]
        index += 2
        if channel == 9:
            continue
        if kind == 0x90 and data2 > 0:
            sounding[(channel, data1)] = tick
            continue
        if kind == 0x80 or (kind == 0x90 and data2 == 0):
            start = sounding.pop((channel, data1), None)
            if start is None or tick <= start:
                continue
            found.setdefault(channel, []).append(
                (start / division, tick / division, data1)
            )
    for (channel, pitch), start in sounding.items():
        if tick <= start:
            continue
        found.setdefault(channel, []).append((start / division, tick / division, pitch))
    return found


def _monophonic(notes: list[tuple[float, float, int]]) -> list[tuple[float, float, int]]:
    ordered = sorted(notes, key=lambda item: (item[0], -item[2]))
    kept: list[tuple[float, float, int]] = []
    for start, end, pitch in ordered:
        if end - start < 0.05:
            continue
        if kept and start < kept[-1][1] - 0.02:
            previous = kept[-1]
            if pitch <= previous[2]:
                continue
            if start - previous[0] >= 0.2:
                kept[-1] = (previous[0], start, previous[2])
                kept.append((start, end, pitch))
            else:
                kept[-1] = (start, end, pitch)
            continue
        kept.append((start, end, pitch))
    return kept


def parse_midi(data: bytes) -> dict:
    if len(data) < 14 or data[:4] != b"MThd":
        raise ValueError("not a MIDI file")
    if len(data) > 1_500_000:
        raise ValueError("too large")
    header_len = struct.unpack(">I", data[4:8])[0]
    if header_len < 6 or 8 + header_len > len(data):
        raise ValueError("bad midi")
    fmt, tracks, division = struct.unpack(">HHH", data[8:14])
    if fmt not in (0, 1) or division == 0 or division & 0x8000 or tracks == 0 or tracks > 64:
        raise ValueError("bad midi")
    pos = 8 + header_len
    by_channel: dict[int, list[tuple[float, float, int]]] = {}
    for _ in range(tracks):
        if pos + 8 > len(data) or data[pos : pos + 4] != b"MTrk":
            raise ValueError("bad midi")
        length = struct.unpack(">I", data[pos + 4 : pos + 8])[0]
        pos += 8
        end = pos + length
        if end > len(data):
            raise ValueError("truncated midi")
        for channel, notes in _read_track(data[pos:end], division).items():
            by_channel.setdefault(channel, []).extend(notes)
        pos = end
    if not by_channel:
        return {"melody": [], "playable": False, "folded": False, "trimmed": False}
    channel = max(by_channel, key=lambda key: len(by_channel[key]))
    line = _monophonic(by_channel[channel])
    melody: list[dict] = []
    folded = False
    dropped = 0
    cursor = line[0][0] if line else 0.0
    trimmed = False
    for start, end, pitch in line:
        if len(melody) >= MAX_MELODY:
            trimmed = True
            break
        if melody and start - cursor >= 0.35:
            if len(melody) >= MAX_MELODY:
                trimmed = True
                break
            melody.append({"note": "rest", "beats": _quantize(start - cursor)})
        if len(melody) >= MAX_MELODY:
            trimmed = True
            break
        name, shifts = _fold(pitch)
        if name is None:
            dropped += 1
            cursor = end
            continue
        folded = folded or shifts > 0
        melody.append({"note": name, "beats": _quantize(end - start)})
        cursor = end
        if len(melody) >= MAX_MELODY:
            trimmed = True
            break
    sounded = [item for item in melody if item["note"] != "rest"]
    playable = len(sounded) >= 4 and (dropped / max(1, len(line))) <= 0.15
    return {
        "melody": melody if playable or sounded else [],
        "playable": playable,
        "folded": folded and playable,
        "trimmed": trimmed,
    }


def encode_vlq(value: int) -> bytes:
    if value < 0:
        raise ValueError("delta")
    buf = [value & 0x7F]
    value >>= 7
    while value:
        buf.append((value & 0x7F) | 0x80)
        value >>= 7
    return bytes(reversed(buf))


def build_midi(notes: list[tuple[int, float, float]], division: int = 480) -> bytes:
    """One melody. Each item is pitch, length in quarters, gap in quarters before it."""
    body = bytearray()
    for pitch, quarters, gap in notes:
        body += encode_vlq(int(round(gap * division)))
        body += bytes([0x90, pitch & 0x7F, 64])
        body += encode_vlq(max(1, int(round(quarters * division))))
        body += bytes([0x80, pitch & 0x7F, 0])
    body += bytes([0x00, 0xFF, 0x2F, 0x00])
    track = b"MTrk" + struct.pack(">I", len(body)) + bytes(body)
    header = b"MThd" + struct.pack(">IHHH", 6, 0, 1, division)
    return header + track
