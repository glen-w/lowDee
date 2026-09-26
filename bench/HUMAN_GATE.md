Type: CONTRACT
Authority: The human gate — the five steps a new player on a real low D has to clear, and the synthetic stand-in that stands in for that session. The listen algorithm is described in [ARCHITECTURE.md](../ARCHITECTURE.md). Product rules for the gate live in [CONCEPT.md](../CONCEPT.md).

# Human gate protocol

Someone who did not already play whistle, on a real low D.

1. Name the whistle on the object card.
2. Cover all holes. Hold low D until the app settles (~10s). Profile writes `break_hz`.
3. On breath and octave, with Low D selected: blow into the second octave on purpose. Expect one remark: “Softer — stay on the low one.” Target Hz must not change.
4. Retry soft. Settle on the low note against the same target.
5. Play the first phrase of the air (D–E–F♯–G–A) against that same `break_hz`.

If the target drifts mid-phrase or the early-break remark is wrong, stop ornament work and fix the tracker.

Synthetic stand-in (CI / no whistle in the room):

```bash
source "$HOME/.cargo/env"
cargo run --manifest-path src-tauri/Cargo.toml --bin listen -- gate
```
