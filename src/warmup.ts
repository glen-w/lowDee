// One guided pass before each sitting. The whistle stays down.
// Stretches and wrist motions are the ordinary musician set (finger pulls,
// wrist up and down, palms together, woven fingers, thumb, table rises,
// flips, side bends, a gentle press, one finger at a time). The breath is a
// counted hiss at 60 beats a minute: in for four, out for four, then eight,
// then twelve. One pass, not a rehab dose. Ease off if it hurts.

export interface WarmBeat {
  exercise: string;
  group: "hands" | "breath";
  title: string;
  line: string;
  mark: string;
  seconds: number;
}

type Hand = "Right" | "Left";

function beat(
  exercise: string,
  group: WarmBeat["group"],
  title: string,
  line: string,
  mark: string,
  seconds: number,
): WarmBeat {
  return { exercise, group, title, line, mark, seconds };
}

function repeat(n: number, marks: Array<{ mark: string; seconds: number }>) {
  const out: Array<{ mark: string; seconds: number }> = [];
  for (let i = 0; i < n; i++) out.push(...marks);
  return out;
}

function handed(
  exercise: string,
  title: string,
  lineFor: (hand: Hand) => string,
  marks: Array<{ mark: string; seconds: number }>,
): WarmBeat[] {
  const out: WarmBeat[] = [];
  for (const hand of ["Right", "Left"] as const) {
    const line = lineFor(hand);
    for (const m of marks) {
      out.push(beat(exercise, "hands", title, line, m.mark, m.seconds));
    }
  }
  return out;
}

function breathCycle(outBeats: number, outWord: string): WarmBeat[] {
  const beats: WarmBeat[] = [];
  for (let i = 1; i <= 4; i++) {
    beats.push(
      beat(
        "breath",
        "breath",
        "Breath",
        "In for four, through the mouth. Belly moves out. Shoulders stay down.",
        `In ${i}`,
        1,
      ),
    );
  }
  for (let i = 1; i <= outBeats; i++) {
    beats.push(
      beat(
        "breath",
        "breath",
        "Breath",
        `Out for ${outWord}, on a hiss — sss. Even. Don't pulse it.`,
        `Out ${i}`,
        1,
      ),
    );
  }
  return beats;
}

function buildWarmup(): WarmBeat[] {
  const fingers = ["Index", "Middle", "Ring", "Little"];
  const lifts = ["Thumb", "Index", "Middle", "Ring", "Little"];

  return [
    beat(
      "shake",
      "hands",
      "Shake out",
      "Whistle down. Shake the hands, then the elbows. Loose, not fast.",
      "Loose",
      12,
    ),
    ...handed(
      "rise",
      "Rise and drop",
      (hand) =>
        `${hand} forearm on a table. Palm down, hand past the edge. Up into a loose fist, then drop the hand and let the fingers open.`,
      repeat(4, [
        { mark: "Up", seconds: 4 },
        { mark: "Down", seconds: 4 },
      ]),
    ),
    ...handed(
      "flip",
      "Flip",
      (hand) =>
        `${hand} forearm on the thigh. Turn the hand over. The forearm stays put.`,
      repeat(4, [
        { mark: "Palm up", seconds: 2 },
        { mark: "Palm down", seconds: 2 },
      ]),
    ),
    ...handed(
      "sides",
      "Side to side",
      (hand) =>
        `${hand} forearm on the table, palm down. Bend the wrist one way, then the other. Slow.`,
      repeat(4, [
        { mark: "Thumb side", seconds: 4 },
        { mark: "Little finger", seconds: 4 },
      ]),
    ),
    ...handed(
      "fingers",
      "One finger back",
      (hand) =>
        `${hand} arm forward, palm toward you, fingers up. The other hand draws this finger back a little. A mild pull.`,
      fingers.map((mark) => ({ mark, seconds: 6 })),
    ),
    ...handed(
      "wrist-up",
      "Wrist back",
      (hand) =>
        `${hand} arm forward, palm out, fingers up. The other hand draws the fingers back. Elbow soft. Straighten it only if that stays easy.`,
      [{ mark: "Fingers up", seconds: 20 }],
    ),
    ...handed(
      "wrist-down",
      "Wrist down",
      (hand) =>
        `${hand} arm forward, fingers toward the floor. The other hand eases the wrist a little further, toward the little finger.`,
      [{ mark: "Fingers down", seconds: 20 }],
    ),
    beat(
      "prayer",
      "hands",
      "Palms together",
      "Palms together in front of the chest, fingers up. Lower the hands toward the waist. Elbows wide. Keep breathing.",
      "Ease down",
      20,
    ),
    beat(
      "weave-out",
      "hands",
      "Fingers woven",
      "Weave the fingers and turn the palms out. Reach forward. Soft.",
      "Forward",
      15,
    ),
    beat(
      "weave-up",
      "hands",
      "Reach up",
      "Same weave. Reach up, arms by the ears. Stop if the shoulders complain.",
      "Up",
      15,
    ),
    ...handed(
      "thumb",
      "Thumb",
      (hand) =>
        `${hand} arm forward. The other hand draws the thumb back toward the wrist. Don't pinch.`,
      [{ mark: "Thumb", seconds: 15 }],
    ),
    ...handed(
      "lifts",
      "One finger up",
      (hand) =>
        `${hand} hand flat on the table, palm down, as if the fingers were over holes. Lift only this one. The others stay down.`,
      lifts.map((mark) => ({ mark, seconds: 3 })),
    ),
    ...["Right", "Left"].flatMap((hand) => [
      beat(
        "press",
        "hands",
        "A gentle press",
        `${hand} hand, palm down. The other hand on top. Try to lift. The hand stays put. Keep breathing.`,
        "Palm down",
        10,
      ),
      beat(
        "press",
        "hands",
        "A gentle press",
        `${hand} hand, palm up. The other hand stops the lift. Easy pressure. Keep breathing.`,
        "Palm up",
        10,
      ),
    ]),
    beat(
      "breath",
      "breath",
      "Breath",
      "Sit tall. In and out through the mouth. Belly out, shoulders down. Stop if you feel dizzy.",
      "Ready",
      5,
    ),
    ...breathCycle(4, "four"),
    ...breathCycle(8, "eight"),
    ...breathCycle(12, "twelve"),
  ];
}

export const WARMUP: WarmBeat[] = buildWarmup();

export function exerciseIds(beats: WarmBeat[] = WARMUP): string[] {
  const ids: string[] = [];
  for (const b of beats) {
    if (ids[ids.length - 1] !== b.exercise) ids.push(b.exercise);
  }
  return ids;
}

export function warmupStats(beats: WarmBeat[] = WARMUP): { steps: number; seconds: number } {
  return {
    steps: exerciseIds(beats).length,
    seconds: beats.reduce((n, b) => n + b.seconds, 0),
  };
}

export function stepNumber(beats: WarmBeat[], at: number): number {
  const id = beats[at]?.exercise;
  const ids = exerciseIds(beats);
  const i = ids.indexOf(id ?? "");
  return i < 0 ? ids.length : i + 1;
}

export function groupLabel(group: WarmBeat["group"]): string {
  return group === "breath" ? "Breath" : "Hands";
}

export interface WarmPaint {
  beat: WarmBeat;
  elapsedMs: number;
  paused: boolean;
  step: number;
  steps: number;
  overallElapsedMs: number;
  overallMs: number;
}

export function warmIntroHtml(steps: number, seconds: number): string {
  const minutes = Math.max(1, Math.round(seconds / 60));
  return `
    <div class="app-shell">
      <div class="topbar">
        <p class="eyebrow">Before you play</p>
        <button type="button" class="ghost" id="glossary-open">Glossary</button>
      </div>
      <h1>Hands, then breath</h1>
      <p class="lede">
        Every sitting starts here. One pass for the hands and wrists, then a counted breath.
        The whistle stays down. About ${minutes} minutes, ${steps} steps.
      </p>
      <div class="card">
        <p class="remark">Go gently. Ease off if it hurts. Stop if you feel dizzy, numb, or a sharp pull. This warms you up. It is not a treatment.</p>
        <div class="row" style="margin-top:0.5rem">
          <button class="primary" id="warm-begin">Begin</button>
          <button class="ghost" id="warm-skip">Ease off — skip</button>
        </div>
      </div>
    </div>`;
}

export function warmRunHtml(): string {
  return `
    <div class="app-shell">
      <div class="topbar">
        <p class="eyebrow" id="warm-kicker"></p>
        <button type="button" class="ghost" id="glossary-open">Glossary</button>
      </div>
      <h1 id="warm-title"></h1>
      <p class="lede" id="warm-line"></p>
      <div class="card">
        <p class="warm-mark" id="warm-mark"></p>
        <p class="meta" id="warm-left"></p>
        <div class="hold-meter warm-meter"><i id="warm-bar"></i></div>
        <div class="hold-meter warm-meter warm-all"><i id="warm-all"></i></div>
        <div class="row" style="margin-top:1.25rem">
          <button class="primary" id="warm-next">Next</button>
          <button id="warm-pause">Pause</button>
          <button class="ghost" id="warm-skip">Ease off — skip</button>
        </div>
      </div>
    </div>`;
}

export function warmDoneHtml(): string {
  return `
    <div class="app-shell">
      <div class="topbar">
        <p class="eyebrow">Before you play</p>
        <button type="button" class="ghost" id="glossary-open">Glossary</button>
      </div>
      <h1>Pick up the whistle</h1>
      <p class="lede">Hands and breath are done for this sitting.</p>
      <div class="card">
        <div class="row">
          <button class="primary" id="warm-done">To the whistle</button>
        </div>
      </div>
    </div>`;
}

export function paintWarm(root: ParentNode, state: WarmPaint): void {
  const { beat, elapsedMs, paused } = state;
  const dur = beat.seconds * 1000;
  const left = Math.max(0, Math.ceil((dur - elapsedMs) / 1000));
  const kicker = root.querySelector("#warm-kicker");
  const title = root.querySelector("#warm-title");
  const line = root.querySelector("#warm-line");
  const mark = root.querySelector("#warm-mark");
  const meta = root.querySelector("#warm-left");
  const bar = root.querySelector("#warm-bar") as HTMLElement | null;
  const all = root.querySelector("#warm-all") as HTMLElement | null;
  const pause = root.querySelector("#warm-pause");
  if (kicker) kicker.textContent = `${groupLabel(beat.group)} · ${state.step} of ${state.steps}`;
  if (title) title.textContent = beat.title;
  if (line) line.textContent = beat.line;
  if (mark) mark.textContent = beat.mark;
  if (meta) meta.textContent = paused ? "Paused" : beat.seconds > 2 ? `${left} s` : "";
  if (bar) bar.style.width = `${Math.min(100, (elapsedMs / dur) * 100)}%`;
  if (all) {
    all.style.width = `${Math.min(100, (state.overallElapsedMs / state.overallMs) * 100)}%`;
  }
  if (pause) pause.textContent = paused ? "Continue" : "Pause";
}
