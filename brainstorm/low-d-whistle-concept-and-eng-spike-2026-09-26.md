Type: PRODUCT
Authority: Notes from the 2026-09-26 room. Not the current product. [CONCEPT.md](../CONCEPT.md) is the contract.

# Low-D whistle teaching software — concept + eng spike

**Room:** Glen, Intern 1, UX, Intern 2, New Bot, life coach  
**Date:** 2026-09-26  
**Ask:** concept brainstorming and eng pathways for software that teaches people (varying musical backgrounds) to play the Irish low-D whistle.

---

## Product shape

- **One engine, two entry tracks** — “never held a whistle” vs “already plays flute/pipes” share the same lesson loop; different first hours, not different apps.
- **Companion to a teacher**, not a lesson-replacement product.
- **Offline-first** — local pitch/breath; no cloud STT required.
- **Repertoire** from ABC packs / thesession.org, not a closed songbook.
- **v0.1 stranger job:** play one air in tune, with a clean octave break. Ornament grammar (cut → tap → roll) stays off the door.

---

## Why Low-D is not a tin-whistle reskin

Bigger holes, longer tube, harder octave break. The thin mic loop must treat **breath + break** as first-class (quiet/loud → low/high octave), not only cents-off pitch. Pass/fail that ignores “you broke early” teaches the wrong habit.

---

## Eng spike (greenlight)

1. Offline pitch tracker (aubio/CREPE) + RMS breath proxy + target note from ABC.
2. **10s “hold this Low D” cal on the first practice screen** (not Settings) — makers vary; fixed concert D gaslights honest players.
3. Store cal as a **named local whistle profile** (cents offset + break RMS) — adults own more than one whistle.
4. Sequence: hold low D → clean break to high D → first phrase of one ABC air.
5. Feedback = sit-in coaching only (`late` / `early` / `octave broke` / “softer into the break”) — not a score. Short sessions.
6. Cut/tap later as **timed gestures**, not “AI-scored rolls” (brittle).

### Spike exit (honest)

Same player can **soft-fail** early break, then **pass on retry**, with the **target not drifting mid-phrase**. If that fails on a real Low D, ABC packs and ornaments don’t save it. Anything past that is v0.2.

---

## Hard offs for v0.1

- Ornament grammar / AI roll scoring  
- Shamey pass/fail / long graded sessions  
- Global (unnamed) cal  
- Fixed concert-D with no maker offset  
- Cloud STT dependency  
- Closed proprietary songbook as the only repertoire  

---

## Participants (this thread)

Intern 1, UX, Intern 2, New Bot, life coach — consensus as above.
