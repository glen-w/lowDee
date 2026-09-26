Type: PRODUCT
Authority: Notes from the 2026-09-26 room. Not the current product. [CONCEPT.md](../CONCEPT.md) replaced this spine, including the ex–high-D track.

# Low-D whistle teacher — P0 spine (room lock)

**Room:** Glen, Researcher, Design eye, Infra, engineer, UX  
**Date:** 2026-09-26 (rev: whistle profile + cal merge)  
**Status:** Superseded. The contract is [CONCEPT.md](../CONCEPT.md). This file is the discussion that led there.

---

## Product shape

- **Not** Duolingo gamification or Celtic tourist chrome — session-table feel.
- **P0 track:** ex–high-D player only.
- **P0 content:** one slow air + cut / tap / roll drills + slow-down loop.
- **Ear lane default;** notation toggle secondary.
- **Offline-first;** on-device pitch/timing; no audio upload; no accounts/CDN/multiplayer/AI tutor in P0.

---

## First practice screen (pre-curriculum)

**Named local whistle profile + ~10s Low-D calibration** before `grip_low_d`.

- Design: quiet object card (make/model or nickname + “this horn”) — not a settings form; no tuner-needle anxiety.
- Single “settled” cue when break + RMS lock → then into `grip_low_d`.
- **Not** a ninth curriculum node — device step only.
- Cal audio on-device only; **no upload**.

---

## UX listen states (`attempt.v1`)

| State | Behaviour |
| --- | --- |
| `idle` | Ready; mic off or armed quiet; whistle + air title OK |
| `wait` | App silent; learner plays; **Wait-Mode** — do not grade ourselves |
| `sounding` | On-device pitch/timing live; **chrome-free** (no XP, red X, live score) |
| `feedback` | Sit-in phrase note; optional ghost vs slow air; **never letter grade** |

Bad model → stay in `wait` unmuted; never brick the loop.

**Grading target:** profile-relative **break + RMS**, not fixed concert D / cents-only (maker variance).

---

## Design contract

- Ornament glyphs (cut / tap / roll / cran / slide) as first-class marks; only when the node asks.
- Sounding frame: no exam chrome.
- Feedback: sit-in prose + optional phrase ghost.
- Hero: brass/copper Low D (big holes visible), warm wood, quiet type.
- Cal card: same session-table chrome as sounding.

---

## Curriculum nodes (`curriculum.v1`) — Researcher freeze

**Track:** `ex_high_d`  
**Strict prereq chain (eng: no renames):**

1. `grip_low_d`
2. `breath_octave`
3. `cnat_vs_csharp`
4. `tongue_vs_pulse`
5. `orn_cut`
6. `orn_tap`
7. `orn_roll`
8. `air_may_morning_dew` (vehicle tying the three ornaments)

**Deferred (not P0 graph):** cran, slide, vibrato, jig/reel speed, long second-octave nodes.

**Placement → start:** high-D / other wind / none → `grip_low_d` for P0 (`track=zero_music` later).

**Tune shortlist:** *The May Morning Dew* (primary). Alternates if ABC/rights messy: *Give Me Your Hand* or another public-domain slow air — keep node id until manifest swap.

---

## Pack format (`pack.v1`) — content only

```
pack/
  manifest.json      # id, version, track, node_ids, content hash
  tune.abc | .musicxml
  fingering-low-d.json   # relative to local profile, not baked concert D
  ornaments.json         # cut/tap/roll layers as data
  ref/                   # optional slow-air audio for ghost only
```

**Whistle profile does NOT live in the pack** — per-device only.

---

## Local progress store

```
{ node_id, state_reached, as_of }
```

plus local **whistle profile**:

```
{ profile_id, label, break_hz, rms_floor, cal_as_of }
```

No WAV in progress store.

---

## Eng non-goals (P0)

- Multiplayer, generative AI tutor, cloud accounts
- Folk Directory live API (deep-links later as pack source only)
- Tune scrape / audio sync upload
- React SPA that needs a backend to play a note
- Baking concert D into packs

**Stack bias:** low-latency on-device pitch; UI offline-first (e.g. Tauri / local webview).

---

## Handoff status

| Lane | Lock |
| --- | --- |
| UX | Four states + sit-in feedback + cal on first screen; break/RMS grading |
| Design eye | Sounding chrome-free; cal as object card |
| Infra | `pack.v1` = content; profile in progress store |
| engineer | Eight node ids + profile fields on schema one-pager |
| Researcher | Nodes unchanged; cal is pre-`grip_low_d` device step |
