Type: GUIDE
Authority: Where labeled takes from a real player are kept. The human gate is [HUMAN_GATE.md](../bench/HUMAN_GATE.md). The scorer does not read this folder.

# Local beginner corpus

Put labeled takes here (gitignored). Synthetic fixtures stay under `bench/fixtures/`.

Suggested labels: leak, squeak, no_tone, tongued, cut_long, clean_hold,
early_break, settled_retry.

Each take is a wav plus a sidecar of the same shape as `bench/takes/`.
`listen grade-take path/to/take.json` runs that wav through the attempt engine.
`bench/score_traces.py` does not read this folder.
