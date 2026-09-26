/** Plain tones from the pack's own notes. A recording replaces them when a wav is there. */

export interface ToneEvent {
  hz: number;
  seconds: number;
  /** Stay in the same breath as the next event. */
  join?: boolean;
}

export interface ToneHandle {
  stop: () => void;
}

const NAMES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const QUARTER_BPM = 66;

export function noteHz(note: string, breakHz: number): number {
  const match = /^([A-G])(#)?(\d)$/.exec(note);
  if (!match) return breakHz;
  const name = match[1] + (match[2] ?? "");
  const semi = NAMES.indexOf(name);
  if (semi < 0) return breakHz;
  const octave = Number(match[3]);
  const fromD4 = semi + octave * 12 - (NAMES.indexOf("D") + 4 * 12);
  return breakHz * 2 ** (fromD4 / 12);
}

interface AbcToken {
  eighths: number;
  rest: boolean;
}

function abcTokens(abc: string): AbcToken[] {
  const out: AbcToken[] = [];
  const re = /([_=^]?)([A-Ga-gzZ])([,']*)(\d+)?(\/\d*)?/g;
  let match: RegExpExecArray | null;
  while ((match = re.exec(abc))) {
    let eighths = match[4] ? Number.parseInt(match[4], 10) : 1;
    if (match[5] === "/") eighths /= 2;
    else if (match[5]?.startsWith("/")) eighths /= Number.parseInt(match[5].slice(1), 10) || 2;
    const rest = match[2] === "z" || match[2] === "Z";
    out.push({ eighths, rest });
  }
  return out;
}

/** Pitch comes from the pack's note list when it lines up with the ABC. Rhythm comes from the ABC. */
export function phraseEvents(abc: string, notes: string[], breakHz: number): ToneEvent[] {
  const tokens = abcTokens(abc);
  const sounded = tokens.filter((token) => !token.rest);
  if (sounded.length === 0 || sounded.length !== notes.length) return [];
  const eighth = 60 / QUARTER_BPM / 2;
  let index = 0;
  return tokens.map((token) => {
    const seconds = token.eighths * eighth;
    if (token.rest) return { hz: 0, seconds };
    const note = notes[index++];
    return { hz: noteHz(note, breakHz), seconds };
  });
}

/** One tone line after another, with a breath between lines. */
export function songToneEvents(
  chunks: readonly { abc: string; notes: readonly string[] }[],
  breakHz: number,
): ToneEvent[] {
  const events: ToneEvent[] = [];
  for (const chunk of chunks) {
    const part = phraseEvents(chunk.abc, [...chunk.notes], breakHz);
    if (part.length === 0) continue;
    if (events.some((event) => event.hz > 0)) events.push({ hz: 0, seconds: 0.4 });
    events.push(...part);
  }
  return events;
}

export function ornamentEvents(gesture: string, hz: number): ToneEvent[] {
  const name = gesture.trim().toLowerCase().replaceAll("_", " ");
  const up = hz * 2 ** (2 / 12);
  const down = hz * 2 ** (-2 / 12);
  const blip = 0.045;
  if (name === "cut") {
    return [
      { hz, seconds: 0.35, join: true },
      { hz: up, seconds: blip, join: true },
      { hz, seconds: 0.9 },
    ];
  }
  if (name === "tap") {
    return [
      { hz, seconds: 0.35, join: true },
      { hz: down, seconds: blip, join: true },
      { hz, seconds: 0.9 },
    ];
  }
  if (name === "double tap") {
    return [
      { hz, seconds: 0.3, join: true },
      { hz: down, seconds: blip, join: true },
      { hz, seconds: 0.22, join: true },
      { hz: down, seconds: blip, join: true },
      { hz, seconds: 0.7 },
    ];
  }
  if (name === "roll" || name === "short roll" || name === "long roll") {
    return [
      { hz, seconds: 0.12, join: true },
      { hz: up, seconds: blip, join: true },
      { hz, seconds: 0.45, join: true },
      { hz: down, seconds: blip, join: true },
      { hz, seconds: 0.7 },
    ];
  }
  return [];
}

function groups(events: ToneEvent[]): ToneEvent[][] {
  const phrases: ToneEvent[][] = [];
  let current: ToneEvent[] = [];
  for (const event of events) {
    if (event.hz <= 0) {
      if (current.length) phrases.push(current);
      current = [];
      phrases.push([event]);
      continue;
    }
    current.push(event);
    if (!event.join) {
      phrases.push(current);
      current = [];
    }
  }
  if (current.length) phrases.push(current);
  return phrases;
}

export function playTones(events: ToneEvent[], rate: number, onended: () => void): ToneHandle {
  const ctx = new AudioContext();
  void ctx.resume();
  const master = ctx.createGain();
  master.gain.value = 0.18;
  master.connect(ctx.destination);
  const speed = rate > 0 ? rate : 1;
  let t = ctx.currentTime + 0.05;
  const oscillators: OscillatorNode[] = [];
  for (const phrase of groups(events)) {
    const sounded = phrase.filter((event) => event.hz > 0);
    const dur = phrase.reduce((sum, event) => sum + event.seconds / speed, 0);
    if (sounded.length === 0 || dur <= 0) {
      t += dur;
      continue;
    }
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = "sine";
    osc.connect(gain);
    gain.connect(master);
    let local = t;
    for (const event of sounded) {
      osc.frequency.setValueAtTime(event.hz, local);
      local += event.seconds / speed;
    }
    const attack = Math.min(0.02, dur / 4);
    const release = Math.min(0.04, dur / 3);
    gain.gain.setValueAtTime(0.0001, t);
    gain.gain.exponentialRampToValueAtTime(1, t + attack);
    const releaseAt = Math.max(t + attack, t + dur - release);
    gain.gain.setValueAtTime(1, releaseAt);
    gain.gain.exponentialRampToValueAtTime(0.0001, t + dur);
    osc.start(t);
    osc.stop(t + dur + 0.02);
    oscillators.push(osc);
    t += dur;
  }
  let alive = true;
  const ms = Math.max(0, (t - ctx.currentTime) * 1000 + 30);
  const timer = window.setTimeout(() => {
    if (!alive) return;
    alive = false;
    void ctx.close();
    onended();
  }, ms);
  return {
    stop() {
      if (!alive) return;
      alive = false;
      window.clearTimeout(timer);
      for (const osc of oscillators) {
        try {
          osc.stop();
        } catch {
          /* already stopped */
        }
      }
      void ctx.close();
    },
  };
}
