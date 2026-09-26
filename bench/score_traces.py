#!/usr/bin/env python3
"""Score listen CLI traces against expected evidence. Not shipped in the app."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "bench" / "out"

EXPECTED = {
    "steady_low_d": {"evidence": "low_d_held", "settled": True},
    "early_break": {"evidence": "early_break", "settled": False},
    "retry_settle": {"evidence": "low_d_held", "settled": True},
    "noise": {"evidence": "couldnt_hear", "settled": False},
    "octave": {"evidence": "octave_there", "settled": True},
    "tongued": {"evidence": "restarted_notes", "settled": False},
    "joined": {"evidence": "phrase_ok", "settled": True},
    "cut_too_long": {"evidence": "cut_too_long", "settled": False},
    "cut_ok": {"evidence": "abstain", "settled": True},
    "leaked_c": {"evidence": "sealed", "settled": False},
    "octave_e_crack": {"evidence": "cracked", "settled": False},
    "a_dorian": {"evidence": "phrase_ok", "settled": True},
    "breath_gap": {"evidence": "phrase_ok", "settled": True},
    "breath_chop": {"evidence": "breath_chops", "settled": False},
    "short_roll_split": {"evidence": "became_notes", "settled": False},
}


def load_summary() -> list[dict]:
    path = OUT / "summary.json"
    if not path.exists():
        print(f"missing {path}; run: cargo run --bin listen -- fixtures", file=sys.stderr)
        sys.exit(2)
    return json.loads(path.read_text())


def main() -> int:
    rows = load_summary()
    failed = 0
    for row in rows:
        name = row["name"]
        exp = EXPECTED.get(name)
        if not exp:
            print(f"SKIP {name}")
            continue
        ok = True
        if "evidence" in exp and row["evidence"] != exp["evidence"]:
            ok = False
        if "settled" in exp and row["settled"] != exp["settled"]:
            ok = False
        status = "PASS" if ok else "FAIL"
        if not ok:
            failed += 1
        print(f"{status} {name}: got evidence={row['evidence']} settled={row['settled']} expected={exp}")
    # Target freeze check on early_break trace
    early = OUT / "early_break.trace.json"
    if early.exists():
        t = json.loads(early.read_text())
        target = t.get("target_hz")
        print(f"INFO early_break target_hz={target}")
    print("OK" if failed == 0 else f"{failed} failed")
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
