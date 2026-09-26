<p align="center">
  <img src="icons/logo.png" alt="lowDee" width="280" />
</p>

Local practice companion for the Irish low-D whistle. One path from the first sound through *The May Morning Dew*, bare and then with cut, tap, and roll. Offline, on device, no account.

## Docs

| Doc | Role |
| --- | --- |
| [CONCEPT.md](CONCEPT.md) | Product decisions: who it is for, the path, feedback, rights, what stays out |
| [ARCHITECTURE.md](ARCHITECTURE.md) | How the app is put together: processes, listen loop, pack, store |
| [FEATURES.md](FEATURES.md) | What the build does today, and what is still ahead |
| [bench/HUMAN_GATE.md](bench/HUMAN_GATE.md) | Human gate on a real low D, and the synthetic stand-in |

`brainstorm/` holds the notes those decisions came from. It is not the current product. `pack/` is the chain the app loads. *The May Morning Dew* is the door. A folder under `teacher/` can be opened from the desk after that door is settled; see [teacher/README.md](teacher/README.md).

## Develop

App window and bundle icons are generated from `icons/logo.png`. Regenerate with:

```bash
npx @tauri-apps/cli icon icons/logo.png -o src-tauri/icons
```

```bash
source "$HOME/.cargo/env"
npm install
npm run tauri dev
```

Vite serves the webview at `http://localhost:1420`. The window title is Low D. The packs bundled beside the app live in `pack/`. *The May Morning Dew* is the door. The rest of the chain opens only after the pack before it is settled.

```bash
npm run check
```

That runs the picture, path, glossary, and ghost tests, `cargo test`, the synthetic fixtures, the trace scorer, and `listen gate`.

## Listen bench (not in the app)

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin listen -- gate
cargo run --manifest-path src-tauri/Cargo.toml --bin listen -- fixtures
python3 bench/score_traces.py
```

`listen grade <name>` prints one fixture as JSON (`steady_low_d`, `early_break`, `noise`). `listen grade-take <sidecar.json>` scores a wav named by that file. Human protocol: [bench/HUMAN_GATE.md](bench/HUMAN_GATE.md).
