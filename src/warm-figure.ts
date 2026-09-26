// Diagrams for the hands-and-breath pass. Cream, ink, and brass, same as the whistle.
// A left-hand step is the right-hand drawing mirrored. The picture follows the pose
// (up and down, which finger, in and out), not the count.

const SKIN = "#f3ead8";
const HELP = "#e4c99a";
const INK = "#1a140f";
const BRASS = "#c4a574";
const CREASE = "#a89070";
const WOOD = "#5c4632";
const WOOD_EDGE = "#c4a574";
const WOOD_DEEP = "#24180f";

const VB_W = 480;
const VB_H = 228;

export interface FigureBeat {
  exercise: string;
  mark: string;
  line: string;
}

type Digit = "thumb" | "index" | "middle" | "ring" | "little";
type HandName = "Right" | "Left";

interface DigitSpec {
  id: Exclude<Digit, "thumb">;
  x: number;
  w: number;
  h: number;
}

const FINGER_BASE = -52;
const FINGERS: DigitSpec[] = [
  { id: "little", x: -27, w: 13, h: 50 },
  { id: "ring", x: -11, w: 14, h: 62 },
  { id: "middle", x: 5, w: 15, h: 70 },
  { id: "index", x: 22, w: 14, h: 60 },
];

interface HandOpts {
  x: number;
  y: number;
  rotate?: number;
  scale?: number;
  /** Back of the hand toward the viewer: knuckle dots, thumb on the other side. */
  back?: boolean;
  /** Mirror the thumb side without turning the hand over. */
  flip?: boolean;
  fill?: string;
  arm?: boolean;
  focus?: { digit: Digit; pose: "back" | "lift" } | null;
}

function n(v: number): string {
  const r = Math.round(v * 10) / 10;
  return Object.is(r, -0) ? "0" : String(r);
}

function esc(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;");
}

function slug(s: string): string {
  return s.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "x";
}

function workingHand(line: string): HandName | null {
  if (line.startsWith("Right")) return "Right";
  if (line.startsWith("Left")) return "Left";
  return null;
}

function bilateral(exercise: string): boolean {
  return (
    exercise === "intro" ||
    exercise === "shake" ||
    exercise === "prayer" ||
    exercise === "weave-out" ||
    exercise === "weave-up" ||
    exercise === "breath"
  );
}

function poseOf(beat: FigureBeat): string {
  switch (beat.exercise) {
    case "breath":
      if (beat.mark.startsWith("In")) return "in";
      if (beat.mark.startsWith("Out")) return "out";
      return "ready";
    case "shake":
    case "wrist-up":
    case "wrist-down":
    case "prayer":
    case "weave-out":
    case "weave-up":
    case "thumb":
    case "intro":
      return "";
    default:
      return beat.mark;
  }
}

export function figureId(beat: FigureBeat): string {
  const hand = bilateral(beat.exercise) ? "" : (workingHand(beat.line) ?? "");
  return `${beat.exercise}|${poseOf(beat)}|${hand}`;
}

function limb(
  x1: number,
  y1: number,
  x2: number,
  y2: number,
  width: number,
  fill: string,
): string {
  const ink = n(width + 3.4);
  return `<line x1="${n(x1)}" y1="${n(y1)}" x2="${n(x2)}" y2="${n(y2)}" stroke="${INK}" stroke-width="${ink}" stroke-linecap="round"/><line x1="${n(x1)}" y1="${n(y1)}" x2="${n(x2)}" y2="${n(y2)}" stroke="${fill}" stroke-width="${n(width)}" stroke-linecap="round"/>`;
}

function arrow(x1: number, y1: number, x2: number, y2: number): string {
  const dx = x2 - x1;
  const dy = y2 - y1;
  const len = Math.hypot(dx, dy) || 1;
  const ux = dx / len;
  const uy = dy / len;
  const ah = 9;
  const aw = 5;
  const bx = x2 - ux * ah;
  const by = y2 - uy * ah;
  const px = -uy * aw;
  const py = ux * aw;
  return `<line x1="${n(x1)}" y1="${n(y1)}" x2="${n(bx)}" y2="${n(by)}" stroke="${BRASS}" stroke-width="1.8" stroke-linecap="round"/><polygon points="${n(x2)},${n(y2)} ${n(bx + px)},${n(by + py)} ${n(bx - px)},${n(by - py)}" fill="${BRASS}"/>`;
}

function swoosh(cx: number, cy: number, r: number, a0: number, a1: number): string {
  const rad = (a: number) => (a * Math.PI) / 180;
  const x0 = cx + r * Math.cos(rad(a0));
  const y0 = cy + r * Math.sin(rad(a0));
  const x1 = cx + r * Math.cos(rad(a1));
  const y1 = cy + r * Math.sin(rad(a1));
  return `<path d="M ${n(x0)} ${n(y0)} A ${n(r)} ${n(r)} 0 0 1 ${n(x1)} ${n(y1)}" fill="none" stroke="${BRASS}" stroke-width="1.7" stroke-linecap="round" stroke-dasharray="1 7"/>`;
}

function ledge(x1: number, x2: number, top: number): string {
  const w = x2 - x1;
  return `<rect x="${n(x1)}" y="${n(top)}" width="${n(w)}" height="12" rx="2" fill="${WOOD}" stroke="${WOOD_EDGE}" stroke-width="1.3"/><rect x="${n(x1)}" y="${n(top + 12)}" width="${n(w)}" height="34" fill="${WOOD_DEEP}"/>`;
}

function plane(x: number, y: number, w: number, h: number): string {
  return `<rect x="${n(x)}" y="${n(y)}" width="${n(w)}" height="${n(h)}" rx="16" fill="${WOOD}" stroke="${WOOD_EDGE}" stroke-width="1.2"/>`;
}

function fingerSpec(id: Exclude<Digit, "thumb">): DigitSpec {
  const spec = FINGERS.find((f) => f.id === id);
  if (!spec) return FINGERS[2];
  return spec;
}

function backRotation(spec: DigitSpec): number {
  if (spec.id === "middle") return 168;
  if (spec.id === "ring") return -168;
  return spec.x >= 0 ? 148 : -148;
}

/** Tip of a finger, in the hand's local space, before the hand's own transform. */
function fingerTip(spec: DigitSpec, pose: "straight" | "back" | "lift"): { x: number; y: number } {
  if (pose === "back") {
    const rot = (backRotation(spec) * Math.PI) / 180;
    return {
      x: spec.x + spec.h * Math.sin(rot),
      y: FINGER_BASE - spec.h * Math.cos(rot),
    };
  }
  const lift = pose === "lift" ? -26 : 0;
  return { x: spec.x, y: FINGER_BASE - spec.h + lift };
}

function thumbTip(pose: "straight" | "back" | "lift"): { x: number; y: number } {
  const rotDeg = pose === "back" ? 128 : pose === "lift" ? 18 : 42;
  const lift = pose === "lift" ? -14 : 0;
  const rad = (rotDeg * Math.PI) / 180;
  const h = 44;
  return {
    x: 32 + h * Math.sin(rad) + (pose === "lift" ? 4 : 0),
    y: -24 - h * Math.cos(rad) + lift,
  };
}

function digitTip(digit: Digit, pose: "straight" | "back" | "lift"): { x: number; y: number } {
  if (digit === "thumb") return thumbTip(pose);
  return fingerTip(fingerSpec(digit), pose);
}

function worldOf(opts: HandOpts, local: { x: number; y: number }): { x: number; y: number } {
  const sc = opts.scale ?? 1;
  const flip = opts.flip || opts.back ? -1 : 1;
  const sx = local.x * flip * sc;
  const sy = local.y * sc;
  const rad = ((opts.rotate ?? 0) * Math.PI) / 180;
  const c = Math.cos(rad);
  const s = Math.sin(rad);
  return {
    x: opts.x + sx * c - sy * s,
    y: opts.y + sx * s + sy * c,
  };
}

function fingerRect(
  spec: DigitSpec,
  fill: string,
  stroke: string,
  sw: number,
  rot: number,
  dy: number,
): string {
  return `<g transform="translate(${n(spec.x)} ${n(FINGER_BASE + dy)}) rotate(${n(rot)})"><rect x="${n(-spec.w / 2)}" y="${n(-spec.h)}" width="${n(spec.w)}" height="${n(spec.h)}" rx="${n(spec.w / 2)}" fill="${fill}" stroke="${stroke}" stroke-width="${n(sw)}" stroke-linejoin="round"/></g>`;
}

function thumbRect(fill: string, stroke: string, sw: number, rot: number, extra: string): string {
  return `<g transform="translate(32 -24) rotate(${n(rot)}) ${extra}"><rect x="-8" y="-44" width="16" height="48" rx="8" fill="${fill}" stroke="${stroke}" stroke-width="${n(sw)}" stroke-linejoin="round"/></g>`;
}

function palmPath(fill: string): string {
  return `<path d="M-36 -44 C-40 -26 -32 -6 -20 4 C-8 12 16 12 28 2 C40 -8 44 -22 38 -36 C34 -48 24 -54 12 -54 L-24 -54 C-34 -54 -34 -50 -36 -44 Z" fill="${fill}" stroke="${INK}" stroke-width="1.75" stroke-linejoin="round"/>`;
}

function creases(): string {
  return `<path d="M-16 -30 C-2 -22 12 -24 22 -34" fill="none" stroke="${CREASE}" stroke-width="1.25" stroke-linecap="round"/><path d="M-12 -18 C2 -12 12 -14 18 -22" fill="none" stroke="${CREASE}" stroke-width="1.25" stroke-linecap="round"/><path d="M-6 -8 C2 -4 10 -6 14 -12" fill="none" stroke="${CREASE}" stroke-width="1.15" stroke-linecap="round"/>`;
}

function knuckles(): string {
  return FINGERS.map(
    (f) => `<circle cx="${n(f.x)}" cy="-46" r="2.5" fill="${CREASE}" stroke="none"/>`,
  ).join("");
}

function handInner(opts: HandOpts): string {
  const fill = opts.fill ?? SKIN;
  const focus = opts.focus ?? null;
  const parts: string[] = [];
  if (opts.arm) parts.push(limb(0, 6, 0, 62, 28, fill));
  parts.push(palmPath(fill));
  for (const spec of FINGERS) {
    if (focus?.digit === spec.id) continue;
    parts.push(fingerRect(spec, fill, INK, 1.7, 0, 0));
  }
  if (focus && focus.digit !== "thumb") {
    const spec = fingerSpec(focus.digit);
    if (focus.pose === "lift") {
      parts.push(fingerRect(spec, SKIN, BRASS, 2.3, 0, -26));
    } else {
      parts.push(fingerRect(spec, SKIN, BRASS, 2.3, backRotation(spec), 0));
    }
  }
  if (focus?.digit === "thumb" && focus.pose === "lift") {
    parts.push(thumbRect(SKIN, BRASS, 2.3, 18, "translate(4 -14)"));
  } else if (focus?.digit === "thumb" && focus.pose === "back") {
    parts.push(thumbRect(SKIN, BRASS, 2.3, 128, ""));
  } else {
    parts.push(thumbRect(fill, INK, 1.7, 42, ""));
  }
  parts.push(opts.back ? knuckles() : creases());
  return parts.join("");
}

function hand(opts: HandOpts): string {
  const sc = opts.scale ?? 1;
  const rot = opts.rotate ?? 0;
  const flip = opts.flip || opts.back ? -1 : 1;
  return `<g transform="translate(${n(opts.x)} ${n(opts.y)}) rotate(${n(rot)}) scale(${n(flip * sc)} ${n(sc)})">${handInner(opts)}</g>`;
}

/** Other hand, reduced to a pinch. Origin is the fingertips. */
function pinch(x: number, y: number, rot: number): string {
  return `<g transform="translate(${n(x)} ${n(y)}) rotate(${n(rot)})"><rect x="-6.5" y="-6" width="13" height="38" rx="6.5" fill="${HELP}" stroke="${INK}" stroke-width="1.6"/><rect x="-6" y="-4" width="12" height="34" rx="6" fill="${HELP}" stroke="${INK}" stroke-width="1.6" transform="rotate(36)"/><rect x="-7" y="16" width="22" height="16" rx="8" fill="${HELP}" stroke="${INK}" stroke-width="1.6"/></g>`;
}

function sideInner(open: boolean, fill: string, hot: boolean): string {
  const stroke = hot ? BRASS : INK;
  const sw = hot ? 2.2 : 1.7;
  if (!open) {
    return `<ellipse cx="26" cy="1" rx="23" ry="18" fill="${fill}" stroke="${INK}" stroke-width="1.75"/><ellipse cx="12" cy="-14" rx="9" ry="7.5" fill="${fill}" stroke="${INK}" stroke-width="1.6"/><path d="M16 0 C24 5 32 5 38 0" fill="none" stroke="${CREASE}" stroke-width="1.2" stroke-linecap="round"/>`;
  }
  const slots = [
    { y: -17, w: 20, h: 7 },
    { y: -9, w: 26, h: 7 },
    { y: -1, w: 23, h: 7 },
    { y: 7, w: 16, h: 6 },
  ];
  const fingers = slots
    .map(
      (s) =>
        `<rect x="46" y="${s.y}" width="${s.w}" height="${s.h}" rx="3" fill="${fill}" stroke="${stroke}" stroke-width="${sw}"/>`,
    )
    .join("");
  return `<path d="M0 -16 C20 -19 38 -16 48 -13 L48 12 C32 17 14 16 0 12 Z" fill="${fill}" stroke="${INK}" stroke-width="1.75" stroke-linejoin="round"/><path d="M16 -14 C18 -30 38 -33 44 -18 C34 -16 22 -14 16 -14 Z" fill="${fill}" stroke="${INK}" stroke-width="1.7" stroke-linejoin="round"/>${fingers}`;
}

function sideHand(
  x: number,
  y: number,
  angle: number,
  open: boolean,
  fill = SKIN,
  hot = false,
  mirrorY = false,
): string {
  const sy = mirrorY ? -1 : 1;
  return `<g transform="translate(${n(x)} ${n(y)}) rotate(${n(angle)}) scale(1 ${sy})">${sideInner(open, fill, hot)}</g>`;
}

function along(x: number, y: number, angle: number, dist: number): { x: number; y: number } {
  const rad = (angle * Math.PI) / 180;
  return { x: x + dist * Math.cos(rad), y: y + dist * Math.sin(rad) };
}

function drawIntro(): string {
  return (
    hand({ x: 150, y: 168, rotate: -8, scale: 0.72, arm: true }) +
    hand({ x: 330, y: 168, rotate: 8, scale: 0.72, arm: true, flip: true })
  );
}

function drawShake(): string {
  return (
    swoosh(150, 118, 52, 200, 340) +
    swoosh(150, 118, 68, 210, 20) +
    swoosh(330, 118, 52, 200, 340) +
    swoosh(330, 118, 68, 160, 330) +
    hand({ x: 150, y: 78, rotate: 168, scale: 0.7 }) +
    hand({ x: 330, y: 78, rotate: 192, scale: 0.7, back: true })
  );
}

function drawRise(up: boolean): string {
  const wristX = 268;
  const wristY = 108;
  return (
    ledge(36, 250, 121) +
    limb(58, wristY, wristX, wristY, 26, SKIN) +
    (up
      ? sideHand(wristX, wristY, -68, false)
      : sideHand(wristX, wristY, 78, true, SKIN, true))
  );
}

function drawFlip(palmUp: boolean): string {
  return (
    `<ellipse cx="250" cy="176" rx="188" ry="40" fill="${WOOD}" stroke="${WOOD_EDGE}" stroke-width="1.2"/>` +
    `<ellipse cx="250" cy="168" rx="150" ry="22" fill="#6b5140" stroke="none"/>` +
    limb(48, 132, 236, 142, 24, SKIN) +
    hand({ x: 248, y: 136, rotate: 78, scale: 0.58, back: !palmUp })
  );
}

function drawSides(thumbSide: boolean): string {
  const rot = thumbSide ? 62 : 118;
  return (
    plane(24, 40, 432, 160) +
    limb(36, 118, 214, 118, 30, SKIN) +
    hand({ x: 228, y: 118, rotate: rot, scale: 0.56, back: true })
  );
}

function drawOneFinger(mark: string): string {
  const digit = digitFrom(mark) ?? "index";
  const opts: HandOpts = {
    x: 214,
    y: 176,
    scale: 0.92,
    arm: true,
    focus: { digit, pose: "back" },
  };
  const tip = worldOf(opts, digitTip(digit, "back"));
  const fromRight = tip.x >= opts.x;
  return hand(opts) + pinch(tip.x, tip.y, fromRight ? -78 : 78);
}

function drawWristUp(): string {
  const wristX = 286;
  const wristY = 118;
  const angle = -148;
  const tip = along(wristX, wristY, angle, 74);
  const pull = along(wristX, wristY, angle, 36);
  return (
    limb(70, 156, 170, 132, 26, SKIN) +
    limb(170, 132, wristX, wristY, 24, SKIN) +
    sideHand(wristX, wristY, angle, true, SKIN, true) +
    pinch(tip.x - 6, tip.y + 4, -20) +
    arrow(tip.x + 8, tip.y - 16, pull.x, pull.y - 8)
  );
}

function drawWristDown(): string {
  const wristX = 250;
  const wristY = 78;
  const angle = 112;
  const tip = along(wristX, wristY, angle, 72);
  const further = along(wristX, wristY, angle + 18, 98);
  return (
    limb(48, 92, 150, 78, 26, SKIN) +
    limb(150, 78, wristX, wristY, 24, SKIN) +
    sideHand(wristX, wristY, angle, true, SKIN, true) +
    pinch(tip.x + 8, tip.y - 6, 70) +
    arrow(tip.x - 4, tip.y + 6, further.x, further.y)
  );
}

function drawPrayer(): string {
  return (
    `<path d="M150 210 C150 132 188 96 240 92 C292 96 330 132 330 210" fill="#3a2b1e" stroke="#6b5340" stroke-width="1.4"/>` +
    limb(78, 196, 214, 148, 22, SKIN) +
    limb(402, 196, 266, 148, 22, SKIN) +
    sideHand(226, 132, -90, true) +
    sideHand(254, 132, -90, true, SKIN, false, true) +
    arrow(240, 118, 240, 168)
  );
}

function weaveClasp(cx: number, cy: number, s: number): string {
  const parts: string[] = [];
  for (let i = 0; i < 4; i++) {
    const x = cx - 40 * s + i * 20 * s;
    const h = (58 - (i === 0 || i === 3 ? 10 : 0)) * s;
    const lean = -16;
    parts.push(
      `<g transform="translate(${n(x)} ${n(cy)}) rotate(${lean})"><rect x="${n(-6 * s)}" y="${n(-h)}" width="${n(12 * s)}" height="${n(h)}" rx="${n(6 * s)}" fill="${SKIN}" stroke="${INK}" stroke-width="1.6"/></g>`,
    );
  }
  for (let i = 0; i < 4; i++) {
    const x = cx - 30 * s + i * 20 * s;
    const h = (52 - (i === 3 ? 8 : 0)) * s;
    parts.push(
      `<g transform="translate(${n(x)} ${n(cy - 4 * s)}) rotate(16)"><rect x="${n(-5.5 * s)}" y="${n(-h)}" width="${n(11 * s)}" height="${n(h)}" rx="${n(5.5 * s)}" fill="${HELP}" stroke="${INK}" stroke-width="1.6"/></g>`,
    );
  }
  parts.push(
    `<ellipse cx="${n(cx - 16 * s)}" cy="${n(cy + 8 * s)}" rx="${n(28 * s)}" ry="${n(18 * s)}" fill="${SKIN}" stroke="${INK}" stroke-width="1.7"/>`,
    `<ellipse cx="${n(cx + 18 * s)}" cy="${n(cy + 10 * s)}" rx="${n(26 * s)}" ry="${n(17 * s)}" fill="${HELP}" stroke="${INK}" stroke-width="1.7"/>`,
    thumbRectAt(cx - 46 * s, cy + 4 * s, -40, s, SKIN),
    thumbRectAt(cx + 48 * s, cy + 6 * s, 40, s, HELP),
  );
  return parts.join("");
}

function thumbRectAt(x: number, y: number, rot: number, s: number, fill: string): string {
  return `<g transform="translate(${n(x)} ${n(y)}) rotate(${n(rot)}) scale(${n(s)})"><rect x="-7" y="-28" width="14" height="32" rx="7" fill="${fill}" stroke="${INK}" stroke-width="1.7"/></g>`;
}

function drawWeave(up: boolean): string {
  if (!up) {
    return weaveClasp(240, 118, 1.15) + arrow(240, 168, 240, 132);
  }
  return (
    `<circle cx="240" cy="162" r="26" fill="${SKIN}" stroke="${INK}" stroke-width="1.7"/>` +
    `<ellipse cx="210" cy="164" rx="6" ry="9" fill="${SKIN}" stroke="${INK}" stroke-width="1.5"/>` +
    `<ellipse cx="270" cy="164" rx="6" ry="9" fill="${SKIN}" stroke="${INK}" stroke-width="1.5"/>` +
    limb(168, 78, 188, 148, 18, SKIN) +
    limb(312, 78, 292, 148, 18, SKIN) +
    weaveClasp(240, 62, 0.82)
  );
}

function drawThumb(): string {
  const opts: HandOpts = {
    x: 200,
    y: 178,
    scale: 0.95,
    arm: true,
    focus: { digit: "thumb", pose: "back" },
  };
  const tip = worldOf(opts, digitTip("thumb", "back"));
  return hand(opts) + pinch(tip.x + 4, tip.y, -60);
}

function drawLift(mark: string): string {
  const digit = digitFrom(mark) ?? "index";
  return (
    plane(48, 36, 384, 168) +
    hand({
      x: 240,
      y: 168,
      scale: 0.78,
      back: true,
      focus: { digit, pose: "lift" },
    })
  );
}

function drawPress(palmUp: boolean): string {
  const table = plane(64, 48, 352, 150);
  const lower: HandOpts = {
    x: 246,
    y: 158,
    scale: 0.62,
    back: !palmUp,
  };
  const upper: HandOpts = {
    x: 246,
    y: 132,
    rotate: palmUp ? -8 : 6,
    scale: 0.5,
    back: true,
    fill: HELP,
  };
  const cue = along(246, 150, -90, 36);
  return table + hand(lower) + arrow(236, 176, cue.x - 8, cue.y) + hand(upper);
}

function drawBreath(mark: string): string {
  const phase = mark.startsWith("In") ? "in" : mark.startsWith("Out") ? "out" : "ready";
  const bellyRx = phase === "in" ? 40 : phase === "out" ? 24 : 32;
  const bellyRy = phase === "in" ? 28 : phase === "out" ? 16 : 22;
  const bellyFill = phase === "in" ? BRASS : "#d9cbb4";
  const hiss =
    phase === "out"
      ? `<path d="M292 78 q14 2 18 10" fill="none" stroke="${BRASS}" stroke-width="1.6" stroke-linecap="round"/><path d="M296 90 q16 3 20 12" fill="none" stroke="${BRASS}" stroke-width="1.6" stroke-linecap="round"/><path d="M292 102 q12 4 14 12" fill="none" stroke="${BRASS}" stroke-width="1.6" stroke-linecap="round"/>`
      : "";
  const swell =
    phase === "in" ? arrow(240 + bellyRx - 4, 148, 240 + bellyRx + 22, 148) : "";
  return (
    `<rect x="150" y="196" width="180" height="14" rx="4" fill="${WOOD}" stroke="${WOOD_EDGE}" stroke-width="1.2"/>` +
    `<path d="M176 196 L186 118 C190 100 210 92 240 92 C270 92 290 100 294 118 L304 196 Z" fill="#e7d7bc" stroke="${INK}" stroke-width="1.75" stroke-linejoin="round"/>` +
    `<path d="M168 124 C150 132 142 150 148 168" fill="none" stroke="${INK}" stroke-width="10" stroke-linecap="round"/>` +
    `<path d="M168 124 C150 132 142 150 148 168" fill="none" stroke="#e7d7bc" stroke-width="6.5" stroke-linecap="round"/>` +
    `<path d="M312 124 C330 132 338 150 332 168" fill="none" stroke="${INK}" stroke-width="10" stroke-linecap="round"/>` +
    `<path d="M312 124 C330 132 338 150 332 168" fill="none" stroke="#e7d7bc" stroke-width="6.5" stroke-linecap="round"/>` +
    `<circle cx="240" cy="70" r="24" fill="${SKIN}" stroke="${INK}" stroke-width="1.75"/>` +
    `<path d="M232 76 q8 6 16 0" fill="none" stroke="${INK}" stroke-width="1.4" stroke-linecap="round"/>` +
    `<ellipse cx="240" cy="150" rx="${bellyRx}" ry="${bellyRy}" fill="${bellyFill}" stroke="${INK}" stroke-width="1.6" opacity="${phase === "in" ? "0.9" : "1"}"/>` +
    swell +
    hiss
  );
}

function digitFrom(mark: string): Digit | null {
  if (mark === "Thumb") return "thumb";
  if (mark === "Index") return "index";
  if (mark === "Middle") return "middle";
  if (mark === "Ring") return "ring";
  if (mark === "Little") return "little";
  return null;
}

function scene(beat: FigureBeat): string {
  switch (beat.exercise) {
    case "intro":
      return drawIntro();
    case "shake":
      return drawShake();
    case "rise":
      return drawRise(beat.mark === "Up");
    case "flip":
      return drawFlip(beat.mark === "Palm up");
    case "sides":
      return drawSides(beat.mark === "Thumb side");
    case "fingers":
      return drawOneFinger(beat.mark);
    case "wrist-up":
      return drawWristUp();
    case "wrist-down":
      return drawWristDown();
    case "prayer":
      return drawPrayer();
    case "weave-out":
      return drawWeave(false);
    case "weave-up":
      return drawWeave(true);
    case "thumb":
      return drawThumb();
    case "lifts":
      return drawLift(beat.mark);
    case "press":
      return drawPress(beat.mark === "Palm up");
    case "breath":
      return drawBreath(beat.mark);
    default:
      return drawIntro();
  }
}

function aria(beat: FigureBeat, hand: HandName | null): string {
  const who = hand ? `${hand} hand` : "";
  switch (beat.exercise) {
    case "intro":
      return "Open hands, resting.";
    case "shake":
      return "Both hands loose, shaking out.";
    case "rise":
      return beat.mark === "Up"
        ? `${who}, loose fist up past the table edge.`
        : `${who}, dropped past the table edge, fingers open.`;
    case "flip":
      return `${who} on the thigh, ${beat.mark === "Palm up" ? "palm up" : "palm down"}.`;
    case "sides":
      return `${who} on the table, bent to the ${beat.mark === "Thumb side" ? "thumb side" : "little-finger side"}.`;
    case "fingers":
      return `${who}, ${beat.mark.toLowerCase()} finger drawn back.`;
    case "wrist-up":
      return `${who}, fingers drawn back.`;
    case "wrist-down":
      return `${who}, wrist eased further down.`;
    case "prayer":
      return "Palms together, hands lowering, elbows wide.";
    case "weave-out":
      return "Fingers woven, palms turned out, reaching forward.";
    case "weave-up":
      return "Fingers woven, arms up beside the ears.";
    case "thumb":
      return `${who}, thumb drawn back toward the wrist.`;
    case "lifts":
      return `${who} flat on the table, ${beat.mark.toLowerCase()} finger lifted.`;
    case "press":
      return `${who}, ${beat.mark === "Palm up" ? "palm up" : "palm down"}, the other hand holding it.`;
    case "breath":
      if (beat.mark.startsWith("In")) return "Breathing in. Belly forward, shoulders down.";
      if (beat.mark.startsWith("Out")) return "Breathing out on a hiss. Belly in, shoulders down.";
      return "Sitting tall, ready to breathe.";
    default:
      return "Warm-up.";
  }
}

export function warmIntroFigure(): string {
  return warmFigureMarkup({ exercise: "intro", mark: "", line: "" });
}

export function warmFigureMarkup(beat: FigureBeat): string {
  const hand = bilateral(beat.exercise) ? null : workingHand(beat.line);
  const label = aria(beat, hand);
  const id = `fig-${slug(figureId(beat))}`;
  const body =
    hand === "Left"
      ? `<g transform="translate(${VB_W} 0) scale(-1 1)">${scene(beat)}</g>`
      : scene(beat);
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${VB_W} ${VB_H}" class="warm-figure-svg" role="img" aria-labelledby="${id}"><title id="${id}">${esc(label)}</title>${body}</svg>`;
}
