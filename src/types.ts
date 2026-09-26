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
  session: string;
  pulse: string;
  pulse_beats: number;
  playable: boolean;
  open: boolean;
  settled: boolean;
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

export const NODE_COPY: Record<
  string,
  { title: string; body: string; body_high_d?: string }
> = {
  first_sound: {
    title: "First sound",
    body: "Whistle to the mouth. Pads on all six holes. A steady low D. Hold until it sits still.",
  },
  staircase: {
    title: "Staircase",
    body: "One finger lifts at a time — only the notes the air uses. Hear it, find it, play it back.",
    body_high_d: "Fingerings you know. Find them on this tube, one lift at a time.",
  },
  breath_octave: {
    title: "Breath and octave",
    body: "Same fingering as low D. Soft air keeps the low one. More air finds the octave — then come home.",
    body_high_d: "The low whistle wants slower air than the small one. Soft first, then find the octave on purpose.",
  },
  hedwig: {
    title: "Hedwig's Theme",
    body: "A tune you already know. Open the letter notes and use the key of D — that is this whistle. The key of C on the same page is for piano, flute, and recorder. Play the opening from those D letters, one note at a time. A high mark is the same holes and more air. The tune stays on that page.",
  },
  on_the_breath: {
    title: "On the breath",
    body: "A short phrase. The tongue starts the line, then stays off. Let the notes join.",
  },
  air_bare: {
    title: "The air, bare",
    body: "Hear the phrase. See the holes. Play from the start of the chunk.",
  },
  orn_cut: {
    title: "Cut",
    body: "On one long note: a finger snaps open and shut, too short to hum, on the pulse.",
  },
  orn_tap: {
    title: "Tap",
    body: "A finger below the sounding note flicks shut and open. Pitch dips and returns.",
  },
  orn_roll: {
    title: "Roll",
    body: "Cut near the start, tap later. The note itself is still the one you meant.",
  },
  air_may_morning_dew: {
    title: "The May Morning Dew",
    body: "The same air, with cut, tap, and roll only where marked. Pictures can hide.",
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
