"""Fetch a few songs into songs/library for personal practice.

Hits the Traditional Music Library and Mudcat, slowly, and only the pages a
title needs. The files stay in songs/library, which git ignores.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path
from urllib.parse import urljoin, urlsplit

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from songs.parse import (
    MUDCAT_BROWSE,
    TML_BASE,
    best_match,
    decode_html,
    index_names,
    indexes_for_letters,
    letters_for,
    mudcat_browse,
    mudcat_song,
    norm_title,
    parse_midi,
    prepare_url,
    score_title,
    tml_index_links,
    tml_song,
)

UA = "low-d-personal-ingest/0.1 (local practice; personal use)"


class SiteRefused(RuntimeError):
    """The site answered 401, 403, or 429. Stop asking it for more pages."""
ROOT = Path(__file__).resolve().parent
LIBRARY = ROOT / "library"
WANTED = ROOT / "wanted.json"


def delay_for(url: str) -> float:
    host = (urlsplit(url).hostname or "").lower()
    if host.endswith("mudcat.org"):
        return 10.0
    if host.endswith("traditionalmusic.co.uk"):
        return 2.0
    raise ValueError("refused url")


def site_of(url: str) -> str:
    host = (urlsplit(url).hostname or "").lower()
    if host.endswith("mudcat.org"):
        return "mudcat.org"
    if host.endswith("traditionalmusic.co.uk"):
        return "traditionalmusic.co.uk"
    return host


class HostGate:
    def __init__(self) -> None:
        self.seen: set[str] = set()

    def wait(self, url: str, sleep) -> None:
        site = site_of(url)
        if site in self.seen:
            sleep(delay_for(url))
        self.seen.add(site)


def polite_get(url: str, gate: HostGate, get, sleep, attempts: int = 3) -> bytes:
    url = prepare_url(url)
    gate.wait(url, sleep)
    last: Exception | None = None
    for attempt in range(attempts):
        try:
            data = get(url)
        except SiteRefused as exc:
            last = exc
            if attempt == 0:
                sleep(20)
                continue
            raise
        except Exception as exc:
            last = exc
            if attempt + 1 < attempts:
                sleep(min(8.0, 1.5 * (attempt + 1)))
            continue
        if not isinstance(data, (bytes, bytearray)):
            raise TypeError("fetch returned text")
        if len(data) == 0:
            last = ValueError("empty")
            if attempt + 1 < attempts:
                sleep(min(8.0, 1.5 * (attempt + 1)))
            continue
        if len(data) > 2_000_000:
            raise ValueError("too large")
        return bytes(data)
    assert last is not None
    raise last


def cached_get(url: str, dest: Path, refresh: bool, gate: HostGate, get, sleep) -> bytes:
    url = prepare_url(url)
    path = dest / "cache" / hashlib.sha256(url.encode()).hexdigest()
    if path.is_file() and not refresh:
        data = path.read_bytes()
        if data:
            return data
    data = polite_get(url, gate, get, sleep)
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(".tmp")
    tmp.write_bytes(data)
    tmp.replace(path)
    return data


def _slug(title: str) -> str:
    text = norm_title(title).replace(" ", "-")
    text = "-".join(part for part in text.split("-") if part)
    return text[:64] or "song"


def _safe_id(pack_id: str) -> bool:
    if not pack_id or len(pack_id) > 64:
        return False
    prev = True
    for char in pack_id:
        if char.islower() or char.isdigit():
            prev = False
            continue
        if char == "-" and not prev:
            prev = True
            continue
        return False
    return not prev


def _plain_title(title: str) -> str:
    cleaned = "".join(ch for ch in title if ch not in "<>").strip()
    return cleaned[:160]


def _safe_quote(value: str) -> str:
    return "".join(ch for ch in value if ch not in "<>").strip()


def credit_for(lyrics: dict | None, melody: dict | None, differ: bool) -> tuple[str, str, str, str]:
    """Return credit, source name, lyrics url, and the other url."""
    parts = []
    name_bits = []
    lyrics_url = ""
    also_url = ""
    if lyrics:
        lyrics_url = lyrics["url"]
        title = _safe_quote(lyrics["title"])
        if lyrics["origin"] == "tml":
            name_bits.append("Traditional Music Library")
            parts.append(
                f"Words: Traditional Music Library (traditionalmusic.co.uk), “{title}”. {lyrics_url} "
                "Arrangements and MIDI files on that site are the editor’s copyright, offered for personal, "
                "non-commercial music training. The lyrics are for study. Many are traditional or public domain, "
                "which the site does not guarantee."
            )
        else:
            name_bits.append("Mudcat Digital Tradition")
            parts.append(
                f"Words: Digital Tradition, Mudcat Café (mudcat.org), “{title}”. {lyrics_url} "
                "Mudcat does not claim to own the songs and asks for a credit. Copyrighted songs in that "
                "collection stay copyrighted, whether or not a notice is printed, and are not cleared for "
                "commercial use."
            )
    if melody and melody.get("melody", {}).get("playable"):
        title = _safe_quote(melody["title"])
        same = bool(lyrics and melody["origin"] == lyrics.get("origin") and melody["url"] == lyrics.get("url"))
        if not same:
            also_url = melody["url"]
        if melody["origin"] == "tml":
            if "Traditional Music Library" not in name_bits:
                name_bits.append("Traditional Music Library")
            parts.append(
                f"Melody: Traditional Music Library, “{title}”. {melody['url']} "
                "That MIDI is the site’s own arrangement, for personal music training."
            )
        else:
            if "Mudcat Digital Tradition" not in name_bits:
                name_bits.append("Mudcat Digital Tradition")
            parts.append(
                f"Melody: the Mudcat MIDI linked under “{title}”. {melody['url']} "
                "Copyrighted songs in that collection stay copyrighted, whether or not a notice is printed. "
                "Personal practice on this machine."
            )
    if differ:
        parts.append("The words and the melody came from different pages and may not be the same version.")
    parts.append("Not for performance or for passing the files on.")
    credit = " ".join(parts)
    if len(credit) > 1200:
        credit = credit[:1197].rstrip() + "..."
    return credit, " and ".join(name_bits), lyrics_url, also_url


def _prefer_lyrics(tml: dict | None, mudcat: dict | None) -> dict | None:
    if tml and mudcat:
        if tml["score"] > mudcat["score"] + 0.02:
            return tml if len(tml["verses"]) >= 2 or len(mudcat["verses"]) < 2 else mudcat
        if mudcat["score"] > tml["score"] + 0.02:
            return mudcat if len(mudcat["verses"]) >= 2 or len(tml["verses"]) < 2 else tml
        if len(tml["verses"]) >= len(mudcat["verses"]) and len(tml["verses"]) >= 2:
            return tml
        if len(mudcat["verses"]) >= 2:
            return mudcat
        return tml if tml["verses"] else mudcat
    return tml or mudcat


def _prefer_melody(tml: dict | None, mudcat: dict | None, lyrics: dict | None) -> dict | None:
    options = [item for item in (tml, mudcat) if item and item.get("melody", {}).get("playable")]
    if not options:
        return None
    if lyrics:
        same = [item for item in options if item["origin"] == lyrics["origin"]]
        if same:
            return same[0]
    tml_ok = [item for item in options if item["origin"] == "tml"]
    return tml_ok[0] if tml_ok else options[0]


def assemble(pack_id: str, queries: list[str], tml: dict | None, mudcat: dict | None, today: str) -> dict | None:
    lyrics = _prefer_lyrics(
        tml if tml and tml.get("verses") else None,
        mudcat if mudcat and mudcat.get("verses") else None,
    )
    melody = _prefer_melody(tml, mudcat, lyrics)
    if lyrics is None and melody is None:
        return None
    differ = bool(lyrics and melody and lyrics["origin"] != melody["origin"])
    credit, source_name, lyrics_url, also_url = credit_for(lyrics, melody, differ)
    source_url = lyrics_url or (melody["url"] if melody else "")
    try:
        source_url = prepare_url(source_url)
    except ValueError:
        return None
    if also_url:
        try:
            also_url = prepare_url(also_url)
        except ValueError:
            also_url = ""
    if also_url == source_url:
        also_url = ""
    melody_body = melody["melody"] if melody else {"melody": [], "playable": False, "folded": False, "trimmed": False}
    midi_url = ""
    if melody and melody.get("midi_url"):
        midi_url = melody["midi_url"]
    elif lyrics and lyrics.get("midi_url"):
        midi_url = lyrics["midi_url"]
    also_name = ""
    if also_url:
        also_name = "Mudcat" if "mudcat.org" in also_url else "Traditional Music Library"
    sheet = {
        "pack_id": pack_id,
        "title": _plain_title((lyrics or melody)["title"]),
        "queries": queries,
        "verses": lyrics["verses"] if lyrics else [],
        "credit": credit,
        "licence": "personal",
        "source_name": source_name,
        "source_url": source_url,
        "also_name": also_name,
        "also_url": also_url if also_url != source_url else "",
        "retrieved": today,
        "melody": melody_body.get("melody") or [],
        "playable": bool(melody_body.get("playable")),
        "folded": bool(melody_body.get("folded")),
        "trimmed": bool(melody_body.get("trimmed")),
        "midi_url": midi_url,
    }
    if not sheet["verses"] and not sheet["playable"]:
        return None
    return sheet


def _write_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    tmp.replace(path)


def _keep(path: Path, pack_id: str) -> bool:
    if not path.is_file():
        return False
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return False
    return data.get("licence") == "personal" and data.get("pack_id") == pack_id


def _pack_for_title(title: str, wanted: list[dict]) -> str:
    slug = _slug(title)
    for song in wanted:
        if max((score_title(title, query) for query in song["queries"]), default=0) >= 0.99:
            return song["pack_id"]
    if _safe_id(slug):
        return slug
    return "song"


def _fetch_indexes(
    names: list[str], dest: Path, refresh: bool, gate: HostGate, get, sleep
) -> tuple[list[dict], list[str], set[str], bool]:
    rows: list[dict] = []
    errors: list[str] = []
    succeeded: set[str] = set()
    blocked = False
    for name in names:
        if blocked:
            errors.append(f"{name}: skipped after the site refused")
            continue
        url = TML_BASE + name
        try:
            raw = cached_get(url, dest, refresh, gate, get, sleep)
            html = decode_html(raw)
        except SiteRefused as exc:
            blocked = True
            errors.append(f"{name}: {exc}")
            continue
        except Exception as exc:
            errors.append(f"{name}: {exc}")
            continue
        succeeded.add(name)
        page = url
        for link in tml_index_links(html):
            try:
                absolute = prepare_url(urljoin(page, link["href"]))
            except ValueError:
                continue
            rows.append({"title": link["title"], "url": absolute})
    return rows, errors, succeeded, blocked


def _detail_tml(row: dict, score: float, dest: Path, refresh: bool, gate, get, sleep) -> dict:
    raw = cached_get(row["url"], dest, refresh, gate, get, sleep)
    parsed = tml_song(decode_html(raw))
    title = parsed["title"] or row["title"]
    melody = {"melody": [], "playable": False, "folded": False, "trimmed": False}
    midi_url = ""
    for href in parsed["midi_hrefs"]:
        try:
            midi_url = prepare_url(urljoin(row["url"], href))
        except ValueError:
            continue
        break
    if midi_url:
        try:
            melody = parse_midi(cached_get(midi_url, dest, refresh, gate, get, sleep))
        except Exception:
            melody = {"melody": [], "playable": False, "folded": False, "trimmed": False}
    return {
        "origin": "tml",
        "title": title,
        "url": row["url"],
        "score": score,
        "verses": parsed["verses"],
        "melody": melody,
        "midi_url": midi_url,
        "midi_bytes_key": midi_url,
    }


def _detail_mudcat(row: dict, score: float, dest: Path, refresh: bool, gate, get, sleep) -> dict:
    lyric_url = ""
    verses: list[str] = []
    if row.get("song_id"):
        lyric_url = prepare_url(f"https://mudcat.org/@displaysong.cfm?SongID={row['song_id']}")
        try:
            parsed = mudcat_song(decode_html(cached_get(lyric_url, dest, refresh, gate, get, sleep)))
            verses = parsed["verses"]
        except Exception:
            verses = []
    midi_url = ""
    try:
        midi_url = prepare_url(urljoin(MUDCAT_BROWSE, row["href"]))
    except ValueError:
        midi_url = ""
    melody = {"melody": [], "playable": False, "folded": False, "trimmed": False}
    if midi_url:
        try:
            melody = parse_midi(cached_get(midi_url, dest, refresh, gate, get, sleep))
        except Exception:
            melody = {"melody": [], "playable": False, "folded": False, "trimmed": False}
    return {
        "origin": "mudcat",
        "title": row["title"],
        "url": lyric_url or midi_url,
        "score": score,
        "verses": verses,
        "melody": melody,
        "midi_url": midi_url,
    }


def ingest(
    songs: list[dict],
    dest: Path,
    get,
    sleep,
    today: str,
    refresh: bool = False,
    dry_run: bool = False,
    names: list[str] | None = None,
) -> list[dict]:
    gate = HostGate()
    catalog = list(names or index_names())
    report: list[dict] = []
    pending = []
    for song in songs:
        pack_id = song["pack_id"]
        if not _safe_id(pack_id):
            report.append({"pack_id": pack_id, "status": "refused"})
            continue
        path = dest / f"{pack_id}.json"
        if not refresh and not dry_run and _keep(path, pack_id):
            report.append({"pack_id": pack_id, "status": "kept"})
            continue
        pending.append(song)
    if not pending:
        if not dry_run:
            _write_json(dest / "report.json", {"songs": report})
        return report

    letters: set[str] = set()
    for song in pending:
        letters |= letters_for(song["queries"])
    targeted = indexes_for_letters(letters, catalog)
    links, errors, succeeded, blocked = _fetch_indexes(targeted, dest, refresh, gate, get, sleep)
    browse_rows: list[dict] = []
    try:
        browse_html = decode_html(cached_get(MUDCAT_BROWSE, dest, refresh, gate, get, sleep))
        browse_rows = mudcat_browse(browse_html)
    except Exception as exc:
        errors.append(f"mudcat index: {exc}")

    def consider(song: dict) -> tuple[dict | None, dict | None]:
        tml_hit = best_match(song["queries"], links)
        mud_hit = best_match(song["queries"], browse_rows)
        return (
            {"row": tml_hit[0], "score": tml_hit[1]} if tml_hit else None,
            {"row": mud_hit[0], "score": mud_hit[1]} if mud_hit else None,
        )

    missed = []
    chosen: dict[str, tuple] = {}
    for song in pending:
        tml_hit, mud_hit = consider(song)
        if tml_hit or mud_hit:
            chosen[song["pack_id"]] = (tml_hit, mud_hit)
        else:
            missed.append(song)
    if missed and not blocked:
        rest = [name for name in catalog if name not in targeted]
        more, more_errors, more_ok, blocked = _fetch_indexes(rest, dest, refresh, gate, get, sleep)
        errors.extend(more_errors)
        succeeded |= more_ok
        links.extend(more)
        for song in missed:
            chosen[song["pack_id"]] = consider(song)

    for song in pending:
        pack_id = song["pack_id"]
        tml_hit, mud_hit = chosen.get(pack_id, (None, None))
        if not tml_hit and not mud_hit:
            needed = set(indexes_for_letters(letters_for(song["queries"]), catalog))
            status = "no match" if needed and needed <= succeeded else "unreadable"
            report.append({"pack_id": pack_id, "status": status})
            continue
        tml = None
        mudcat = None
        if tml_hit:
            try:
                tml = _detail_tml(tml_hit["row"], tml_hit["score"], dest, refresh, gate, get, sleep)
            except Exception as exc:
                errors.append(f"{pack_id} traditional music library: {exc}")
        if mud_hit:
            try:
                mudcat = _detail_mudcat(mud_hit["row"], mud_hit["score"], dest, refresh, gate, get, sleep)
            except Exception as exc:
                errors.append(f"{pack_id} mudcat: {exc}")
        if tml is None and mudcat is None:
            report.append({"pack_id": pack_id, "status": "failed"})
            continue
        sheet = assemble(pack_id, song["queries"], tml, mudcat, today)
        if sheet is None:
            report.append({"pack_id": pack_id, "status": "empty"})
            continue
        melody_url = sheet.get("midi_url") or ""
        line = {
            "pack_id": pack_id,
            "status": "dry-run" if dry_run else "saved",
            "source": sheet["source_name"],
            "lyrics": len(sheet["verses"]),
            "melody": sum(1 for note in sheet["melody"] if note["note"] != "rest"),
            "url": sheet["source_url"],
        }
        if dry_run:
            report.append(line)
            continue
        _write_json(dest / f"{pack_id}.json", sheet)
        if melody_url:
            try:
                midi = cached_get(melody_url, dest, refresh, gate, get, sleep)
                if midi[:4] == b"MThd":
                    midi_path = dest / "midi" / f"{pack_id}.mid"
                    midi_path.parent.mkdir(parents=True, exist_ok=True)
                    tmp = midi_path.with_suffix(".mid.tmp")
                    tmp.write_bytes(midi)
                    tmp.replace(midi_path)
            except Exception as exc:
                line["midi_error"] = str(exc)
        report.append(line)
    if not dry_run:
        _write_json(dest / "report.json", {"songs": report, "errors": errors})
    return report


def live_get(url: str) -> bytes:
    url = prepare_url(url)
    request = urllib.request.Request(
        url,
        headers={
            "User-Agent": UA,
            "Accept": "text/html,application/octet-stream,*/*;q=0.8",
            "Referer": TML_BASE,
        },
    )

    class _Guard(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            try:
                target = prepare_url(newurl)
            except ValueError as exc:
                raise urllib.error.HTTPError(req.full_url, code, "redirect refused", headers, fp) from exc
            return urllib.request.Request(target, headers=req.headers)

    opener = urllib.request.build_opener(_Guard)
    try:
        with opener.open(request, timeout=25) as response:
            prepare_url(response.geturl())
            data = response.read(2_000_001)
    except urllib.error.HTTPError as exc:
        if exc.code in (401, 403, 429):
            raise SiteRefused(f"http {exc.code} for {url}") from exc
        raise RuntimeError(f"http {exc.code} for {url}") from exc
    if len(data) > 2_000_000:
        raise ValueError("too large")
    return data


def _load_wanted(path: Path) -> list[dict]:
    data = json.loads(path.read_text(encoding="utf-8"))
    songs = []
    for song in data["songs"]:
        pack_id = song["pack_id"]
        queries = [str(item) for item in song["queries"] if str(item).strip()]
        if queries:
            songs.append({"pack_id": pack_id, "queries": queries})
    return songs


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Fetch personal-practice lyrics and melody MIDI.")
    parser.add_argument("--pack", action="append", default=[], help="Only this pack id. Repeatable.")
    parser.add_argument("--title", action="append", default=[], help="Also fetch this title.")
    parser.add_argument("--refresh", action="store_true", help="Fetch again even if a sheet is saved.")
    parser.add_argument("--dry-run", action="store_true", help="Resolve and download, but do not write a sheet.")
    parser.add_argument("--wanted", type=Path, default=WANTED)
    parser.add_argument("--dest", type=Path, default=LIBRARY)
    args = parser.parse_args(argv)
    wanted = _load_wanted(args.wanted)
    if args.pack:
        wanted = [song for song in wanted if song["pack_id"] in args.pack]
    for title in args.title:
        wanted.append({"pack_id": _pack_for_title(title, _load_wanted(args.wanted)), "queries": [title]})
    if not wanted:
        print("No titles to fetch.")
        return 1
    today = time.strftime("%Y-%m-%d", time.gmtime())
    report = ingest(
        wanted,
        args.dest,
        live_get,
        time.sleep,
        today,
        refresh=args.refresh,
        dry_run=args.dry_run,
    )
    for line in report:
        bits = [str(line.get("pack_id")), str(line.get("status"))]
        if line.get("source"):
            bits.append(str(line["source"]))
        if "lyrics" in line:
            bits.append(f"lyrics={line['lyrics']}")
        if "melody" in line:
            bits.append(f"melody={line['melody']}")
        if line.get("error"):
            bits.append(str(line["error"]))
        print("  ".join(bits))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
