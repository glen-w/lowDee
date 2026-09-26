Type: CONTRACT
Authority: The human gate on a real low D — the five steps that open the door, and four later takes once that hold exists — and the synthetic stand-in. The listen algorithm is described in [ARCHITECTURE.md](../ARCHITECTURE.md). Product rules for the gate live in [CONCEPT.md](../CONCEPT.md).

# Human gate protocol

Someone who did not already play whistle, on a real low D.

1. Name the whistle on the object card.
2. Cover all holes. Hold low D until the app settles (~10s). Profile writes `break_hz`.
3. On breath and octave, with Low D selected: blow into the second octave on purpose. Expect one remark: “Softer — stay on the low one.” Target Hz must not change.
4. Retry soft. Settle on the low note against the same target.
5. Play the first phrase of the air (D–E–F♯–G–A) against that same `break_hz`.

If the target drifts mid-phrase or the early-break remark is wrong, stop ornament work and fix the tracker.

## After the door will hold

Same whistle, same profile. One take each, labeled with a sidecar beside the wav. The engine already has these evidence ids. A take that disagrees is a reason to change the tracker. Another sine is not.

1. Ask for C natural. Leave a hole open so G speaks. Expect `sealed`. The node does not settle.
2. A line with a breath mark. Breathe in the middle of the line, not on the mark. Expect `breath_chops`.
3. Ask for second-octave E. Let it fall to the low E. Expect `cracked`.
4. A short roll that breaks into two notes. Expect `became_notes`.

Sidecar shape (`listen grade-take <file.json>`, wav path relative to the json):

```json
{
  "wav": "leaked_c.wav",
  "mode": "single_note",
  "note": "C5",
  "break_hz": 293.66,
  "rms_floor": 0.05,
  "expect_evidence": "sealed",
  "expect_settled": false
}
```

`mode` is `first_sound`, `breath_octave`, `single_note`, `phrase`, `on_the_breath`, or `ornament`. Put real takes under `corpus/` (gitignored). Two synthetic sidecars live in `bench/takes/` and are graded in `cargo test` without a whistle.

Synthetic stand-in (CI / no whistle in the room):

```bash
source "$HOME/.cargo/env"
cargo run --manifest-path src-tauri/Cargo.toml --bin listen -- gate
```
