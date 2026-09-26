export type HoleState = "closed" | "open" | "half";
export type Reads = "no" | "some" | "yes";

export interface HoleChart {
  holes: string[];
  octave: number;
  label: string;
  half?: boolean;
}

export interface Fingering {
  notes: Record<string, HoleChart>;
}

export interface ColumnModel {
  note: string;
  holes: HoleState[];
  octave: number;
  label: string;
  current: boolean;
  mark: string | null;
}

export interface PictureModel {
  whistle: {
    holes: HoleState[];
    octave: number;
    label: string;
    leakHole: number | null;
  };
  columns: ColumnModel[];
  letters: string[];
  /** Fixed-do syllables. Absent unless the player reads and the phrase has been heard. */
  solfege: string[] | null;
}

const SOLFEGE: Record<string, string> = {
  D4: "Re",
  E4: "Mi",
  "F#4": "Fa♯",
  G4: "Sol",
  A4: "La",
  B4: "Si",
  C5: "Do",
  "C#5": "Do♯",
  D5: "Re′",
  E5: "Mi′",
  "F#5": "Fa♯′",
  G5: "Sol′",
  A5: "La′",
  B5: "Si′",
};

const CLOSED: HoleState[] = ["closed", "closed", "closed", "closed", "closed", "closed"];

export function showSolfege(reads: Reads): boolean {
  return reads === "some" || reads === "yes";
}

export function solfegeFor(note: string): string {
  return SOLFEGE[note] ?? "";
}

/** A hole is half-covered only when that hole says so. A note-level flag does not paint the row. */
export function holeStates(chart: HoleChart | undefined): HoleState[] {
  const raw = chart?.holes ?? CLOSED;
  const states: HoleState[] = [];
  for (let i = 0; i < 6; i++) {
    const hole = raw[i];
    states.push(hole === "open" || hole === "half" ? hole : "closed");
  }
  return states;
}

export function pictureModel(opts: {
  fingering: Fingering;
  notes: string[];
  currentIndex: number;
  leakHole?: number | null;
  showSolfege: boolean;
  marks?: Array<string | null>;
}): PictureModel {
  const notes = opts.notes.length > 0 ? opts.notes : ["D4"];
  const index = Math.max(0, Math.min(opts.currentIndex, notes.length - 1));
  const columns: ColumnModel[] = notes.map((note, i) => {
    const chart = opts.fingering.notes[note];
    return {
      note,
      holes: holeStates(chart),
      octave: chart?.octave ?? 1,
      label: chart?.label ?? note,
      current: i === index,
      mark: opts.marks?.[i] ?? null,
    };
  });
  const current = columns[index];
  const leak = opts.leakHole;
  return {
    whistle: {
      holes: current.holes,
      octave: current.octave,
      label: current.label,
      leakHole: leak != null && leak >= 0 && leak < 6 ? leak : null,
    },
    columns,
    letters: columns.map((c) => c.label),
    solfege: opts.showSolfege ? columns.map((c) => solfegeFor(c.note)) : null,
  };
}

export function renderPicture(container: HTMLElement, model: PictureModel): void {
  container.replaceChildren(pictureElement(model));
}

export function pictureElement(model: PictureModel): HTMLElement {
  const wrap = document.createElement("div");
  wrap.className = "picture";

  const hero = document.createElement("div");
  hero.className = "whistle-hero";
  hero.append(whistleSvg(model), callout(model));
  wrap.append(hero);

  if (model.columns.length > 1) {
    wrap.append(columnStrip(model));
  }
  return wrap;
}

function callout(model: PictureModel): HTMLElement {
  const lab = document.createElement("div");
  lab.className = "hole-label";
  lab.textContent = model.whistle.label;
  return lab;
}

function columnStrip(model: PictureModel): HTMLElement {
  const strip = document.createElement("div");
  strip.className = "column-strip";
  strip.setAttribute("role", "list");
  for (const column of model.columns) {
    const col = document.createElement("div");
    col.className = column.current ? "column current" : "column";
    col.setAttribute("role", "listitem");
    col.dataset.note = column.note;
    col.setAttribute(
      "aria-label",
      column.current ? `${column.label}, current` : column.label,
    );
    column.holes.forEach((state) => {
      const dot = document.createElement("i");
      dot.className = `dot ${state}`;
      if (column.octave === 2) dot.classList.add("octave");
      col.append(dot);
    });
    const name = document.createElement("span");
    name.className = "column-name";
    name.textContent = column.label;
    col.append(name);
    if (column.mark) {
      const mark = document.createElement("span");
      mark.className = "column-mark";
      mark.textContent = column.mark;
      col.append(mark);
    }
    strip.append(col);
  }
  return strip;
}

function whistleSvg(model: PictureModel): SVGSVGElement {
  const ns = "http://www.w3.org/2000/svg";
  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("viewBox", "0 0 720 168");
  svg.setAttribute("class", "whistle-svg");
  svg.setAttribute("role", "img");
  const leak =
    model.whistle.leakHole == null ? "" : `, hole ${model.whistle.leakHole + 1} open to the leak`;
  const octave = model.whistle.octave === 2 ? ", octave" : "";
  svg.setAttribute("aria-label", `Low D, ${model.whistle.label}${octave}${leak}`);

  const uid = `w${Math.random().toString(36).slice(2, 8)}`;
  const defs = document.createElementNS(ns, "defs");
  defs.innerHTML = `
    <linearGradient id="${uid}-wood" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#e2c48a"/>
      <stop offset="0.45" stop-color="#c4924e"/>
      <stop offset="1" stop-color="#7a5228"/>
    </linearGradient>
    <linearGradient id="${uid}-half" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0.5" stop-color="#1a140f"/>
      <stop offset="0.5" stop-color="#f3ead8"/>
    </linearGradient>`;
  svg.append(defs);

  const mouth = document.createElementNS(ns, "rect");
  mouth.setAttribute("x", "8");
  mouth.setAttribute("y", "58");
  mouth.setAttribute("width", "78");
  mouth.setAttribute("height", "36");
  mouth.setAttribute("rx", "16");
  mouth.setAttribute("fill", "#3d5c45");
  svg.append(mouth);

  const body = document.createElementNS(ns, "rect");
  body.setAttribute("x", "70");
  body.setAttribute("y", "48");
  body.setAttribute("width", "630");
  body.setAttribute("height", "56");
  body.setAttribute("rx", "28");
  body.setAttribute("fill", `url(#${uid}-wood)`);
  svg.append(body);

  const window = document.createElementNS(ns, "rect");
  window.setAttribute("x", "92");
  window.setAttribute("y", "36");
  window.setAttribute("width", "26");
  window.setAttribute("height", "16");
  window.setAttribute("rx", "3");
  window.setAttribute("fill", "#1a140f");
  svg.append(window);

  if (model.whistle.octave === 2) {
    const mark = document.createElementNS(ns, "circle");
    mark.setAttribute("cx", "105");
    mark.setAttribute("cy", "22");
    mark.setAttribute("r", "7");
    mark.setAttribute("fill", "none");
    mark.setAttribute("stroke", "#c4a574");
    mark.setAttribute("stroke-width", "2");
    svg.append(mark);
  }

  const xs = [210, 280, 350, 440, 510, 580];
  model.whistle.holes.forEach((state, i) => {
    const circle = document.createElementNS(ns, "circle");
    circle.setAttribute("cx", String(xs[i]));
    circle.setAttribute("cy", "76");
    circle.setAttribute("r", "13");
    circle.setAttribute("data-hole", String(i + 1));
    circle.setAttribute("stroke", "#1a140f");
    circle.setAttribute("stroke-width", "2");
    if (state === "closed") circle.setAttribute("fill", "#1a140f");
    else if (state === "half") circle.setAttribute("fill", `url(#${uid}-half)`);
    else circle.setAttribute("fill", "#f3ead8");
    svg.append(circle);

    if (model.whistle.leakHole === i) {
      const ring = document.createElementNS(ns, "circle");
      ring.setAttribute("cx", String(xs[i]));
      ring.setAttribute("cy", "76");
      ring.setAttribute("r", "18");
      ring.setAttribute("fill", "none");
      ring.setAttribute("stroke", "#c4a574");
      ring.setAttribute("stroke-width", "2");
      ring.setAttribute("data-leak", String(i + 1));
      svg.append(ring);
    }

    const num = document.createElementNS(ns, "text");
    num.setAttribute("x", String(xs[i]));
    num.setAttribute("y", "128");
    num.setAttribute("text-anchor", "middle");
    num.setAttribute("fill", "#c9bba3");
    num.setAttribute("font-size", "14");
    num.setAttribute("font-family", "Palatino, Georgia, serif");
    num.textContent = String(i + 1);
    svg.append(num);
  });

  return svg;
}

export function appendNameRows(staff: HTMLElement, model: PictureModel): void {
  const letters = document.createElement("div");
  letters.className = "name-row";
  letters.textContent = model.letters.join("  ");
  staff.append(letters);
  if (model.solfege) {
    const sol = document.createElement("div");
    sol.className = "solfege-row";
    sol.textContent = model.solfege.join("  ");
    staff.append(sol);
  }
}
