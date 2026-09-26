export type ListenState = "idle" | "wait" | "sounding" | "feedback";

export type Reads = "no" | "some" | "yes";
export type Background = "none" | "wind" | "other" | "high_d";

export type Evidence =
  | "sealed"
  | "low_d_held"
  | "early_break"
  | "octave_there"
  | "restarted_notes"
  | "couldnt_hear"
  | "cut_too_long"
  | "tap_missing"
  | "abstain"
  | "phrase_ok"
  | "note_found"
  | "still_d"
  | "let_them_join"
  | "broke_early"
  | "breath_chops"
  | "cracked"
  | "became_notes"
  | "slide_missed";

export interface WhistleProfile {
  profile_id: string;
  label: string;
  break_hz: number;
  rms_floor: number;
  cal_as_of: string;
  reads: Reads;
  background: Background;
  cnat_fingering?: string | null;
  /** Hear starts at the whistle when a book recording is the model. */
  skip_book_talk?: boolean;
  /** Hands-and-breath pass when this profile opens the app. Missing means show it. */
  warmup_on_launch?: boolean;
  /** Song packs added from those already open. Missing means none. */
  lesson_packs?: string[];
  /** After a settled section. Missing means the next one in this part. */
  auto_advance?: "inside" | "across" | "highlight" | "off";
}

export interface BookClip {
  file: string;
  spans: number[][];
}

export interface ProgressEntry {
  node_id: string;
  profile_id?: string;
  pack_id?: string;
  state_reached: string;
  as_of: string;
  via?: string;
}

export interface AppStore {
  profiles: WhistleProfile[];
  active_profile_id: string | null;
  progress: ProgressEntry[];
}

export interface PackNode {
  id: string;
  mode: string;
  title: string;
  body: string;
  high_d_body?: string;
  gesture?: string;
  note?: string;
  hide_pictures?: boolean;
  grade_marks?: boolean;
}

export interface PackSummary {
  id: string;
  title: string;
  wave: number;
  after: string;
  book_ref: string;
  shelf: string;
  rights: string;
  aka: string[];
  source: string;
  licence?: string;
  author?: string;
  site?: string;
  placement?: string;
  session: string;
  pulse: string;
  pulse_beats: number;
  playable: boolean;
  open: boolean;
  settled: boolean;
  /** A page, or a phrase titled as the pack. Drills stay false. */
  song: boolean;
}

export interface CatalogView {
  shelf_open: boolean;
  desk_open: boolean;
  packs: PackSummary[];
  desk: PackSummary[];
  refused: Array<{ folder: string; reason: string }>;
}

export interface Pack {
  manifest: {
    id: string;
    version: string;
    track: string;
    title: string;
    node_ids: string[];
    nodes?: PackNode[];
    pages?: Record<string, { title: string; url: string; key: string }>;
    wave?: number;
    after?: string;
    book_ref?: string;
    shelf?: string;
    rights?: string;
    aka?: string[];
    source?: string;
    licence?: string;
    author?: string;
    site?: string;
    placement?: string;
    session?: string;
    pulse?: string;
    pulse_beats?: number;
    cnat_id?: string;
  };
  phrases: {
    air_title: string;
    meter?: string;
    key?: string;
    chunks: Array<{
      id: string;
      label: string;
      notes: string[];
      abc: string;
      ref: string;
      breaths?: number[];
    }>;
    staircase_notes: string[];
    on_the_breath: { notes: string[]; abc: string; ref: string };
  };
  fingering: {
    notes: Record<
      string,
      { holes: string[]; octave: number; label: string; half?: boolean }
    >;
  };
  ornaments: {
    demo_note: string;
    marks: Array<{ chunk_id: string; note_index: number; gesture: string }>;
  };
  remarks: Record<string, Record<string, string>>;
  tune_abc: string;
  words?: { lines: Array<{ chunk_id: string; text: string }> };
}

export interface SongSheet {
  pack_id: string;
  title: string;
  verses: string[];
  credit: string;
  licence: string;
  source_name: string;
  source_url: string;
  also_name?: string;
  also_url?: string;
  retrieved: string;
  melody: Array<{ note: string; beats: number }>;
  playable: boolean;
  folded: boolean;
  trimmed?: boolean;
}

export const NODE_COPY: Record<
  string,
  { title: string; body: string; body_high_d?: string }
> = {
  first_sound: {
    title: "First sound",
    body: "Whistle to the mouth. The pads cover the holes, not the tips: the second joint on the first two fingers of each hand, the first joint on the third. The little finger rests below the last hole and steadies the tube. A steady low D. Too much air goes shrill. Too little is a thin noise. Hold until it sits still.",
  },
  staircase: {
    title: "Staircase",
    body: "One finger lifts at a time — only the notes the air uses. Relax the hands so the holes can seal. Hear it, find it, play it back.",
    body_high_d: "Fingerings you know. On this tube the pads cover the holes, not the tips. Relax the hands so they can seal. Find them one lift at a time.",
  },
  breath_octave: {
    title: "Breath and octave",
    body: "Same fingering as low D. Soft air keeps the low one. More air finds a point where it jumps the octave — then come home. On a high note, too much air pushes it sharp.",
    body_high_d: "The low whistle wants slower air than the small one. Soft first, then find the octave on purpose. The jump is a change in pressure. On a high note, too much air pushes it sharp.",
  },
  hedwig: {
    title: "Hedwig's Theme",
    body: "A tune you already know. Open the letter notes and use the key of D — that is this whistle. The key of C on the same page is for piano, flute, and recorder. Play the opening from those D letters, one note at a time. A high mark is the same holes and more air. The tune stays on that page.",
  },
  on_the_breath: {
    title: "On the breath",
    body: "A short phrase. Start the first note with a T or a D, then the tongue stays off. Let the notes join. After the start, the air holds the pitch: more is louder, and past a point it goes sharp.",
  },
  air_bare: {
    title: "The air, bare",
    body: "Hear the phrase. See the holes. Play from the start of the chunk.",
  },
  orn_cut: {
    title: "Cut",
    body: "Hold A. Hole 2, the second from the window, is closed. Snap that hole open and shut, too short to hum. The pitch flicks up and A returns. If you can hum the flick, it has become a note.",
  },
  orn_tap: {
    title: "Tap",
    body: "Hold A. Hole 3 is open. Flick it shut and open, and lift it before the lower note has time to sound. The pitch dips and comes back to A.",
  },
  orn_roll: {
    title: "Roll",
    body: "Still A. Give the note a moment, then snap hole 2 open and shut. Later, flick hole 3 shut and open. Keep A after the tap. A long note has room for both. A short one does not.",
  },
  air_may_morning_dew: {
    title: "The May Morning Dew",
    body: "The same air, with cut, tap, and roll only where marked. Two of the same note in a row can take a cut or a tap between them. Pictures can hide.",
  },
};

export function remarkFor(
  remarks: Pack["remarks"],
  evidence: Evidence,
  profile: WhistleProfile | null,
  note?: string | null,
): string {
  const table = remarks[evidence] ?? remarks[evidence.replace(/_/g, "")] ?? {};
  if (!table || Object.keys(table).length === 0) {
    // snake_case keys in JSON
  }
  const block = remarks[evidence];
  if (!block) return "";

  if (note && block[note]) return block[note];
  if (profile?.background === "high_d" && block.high_d) return block.high_d;
  if (profile?.background === "wind" && block.wind) return block.wind;
  if ((profile?.reads === "yes" || profile?.reads === "some") && block.reads_yes) {
    return block.reads_yes;
  }
  return block.default ?? "";
}

export function holesFor(
  fingering: Pack["fingering"],
  note: string,
): { holes: string[]; octave: number; label: string; half?: boolean } {
  return (
    fingering.notes[note] ?? {
      holes: ["closed", "closed", "closed", "closed", "closed", "closed"],
      octave: 1,
      label: note,
    }
  );
}
