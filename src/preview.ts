import abcjs from "abcjs";
import fingeringJson from "../pack/may-morning-dew/fingering-low-d.json";
import {
  appendNameRows,
  pictureModel,
  renderPicture,
  showSolfege,
  solfegeFor,
  type Fingering,
  type Reads,
} from "./picture.ts";
import { noteHz, ornamentEvents, phraseEvents, playTones, type ToneHandle } from "./tones.ts";

const fingering = fingeringJson as Fingering;
const stair = ["D4", "E4", "F#4", "G4", "A4", "B4"];
const phrase = ["D4", "E4", "F#4", "G4", "A4"];

export function mountPreview(root: HTMLElement): void {
  let stairIndex = 0;
  let reads: Reads = "no";
  let heard = false;
  let hidden = false;
  let micDenied = false;
  let continued = false;
  let tones: ToneHandle | null = null;

  const draw = () => {
    root.innerHTML = `
      <div class="app-shell">
        <p class="eyebrow">Picture fixture</p>
        <h1>This horn</h1>
        <div class="card" id="card-block"><div id="card-whistle"></div></div>
        <h2>Staircase</h2>
        <div class="card">
          <div id="stair-whistle"></div>
          <div class="row" style="margin-top:1rem">
            <button class="primary" id="stair-next" type="button">Next note</button>
          </div>
        </div>
        <h2>Cut</h2>
        <div class="card">
          <p class="lede">Hold A. Hole 2, the second from the window, is closed. Snap that hole open and shut, too short to hum. The note you hear is still A.</p>
          <div id="cut-whistle"></div>
          <div class="row" style="margin-top:1rem">
            <button id="cut-hear" type="button">Hear</button>
          </div>
          <p class="meta">Hear plays the notes. A recording replaces them when one is here.</p>
        </div>
        <h2 id="page-title">${continued ? "On the breath" : "Hedwig's Theme"}</h2>
        <div class="card">
          <div class="letter-row">${stair
            .map((note) => {
              const label = fingering.notes[note]?.label ?? note;
              const syllable = solfegeFor(note);
              return `<span>${label} · ${syllable}</span>`;
            })
            .join("")}</div>
          ${continued ? "" : `<button id="continue" type="button">Not yet — continue</button>`}
        </div>
        <h2>Phrase</h2>
        <div class="card">
          <div class="field">
            <label for="reads">Do you read music?</label>
            <select id="reads">
              <option value="no">No</option>
              <option value="some">A little</option>
              <option value="yes">Yes</option>
            </select>
          </div>
          <div id="phrase-whistle"></div>
          <div id="staff" class="staff" hidden></div>
          <div class="row" style="margin-top:1rem">
            <button class="primary" id="hear" type="button">Hear</button>
            <button id="hide" type="button">${hidden ? "Show pictures" : "Hide pictures"}</button>
          </div>
        </div>
        <h2>Microphone</h2>
        <div class="card" id="mic-card">
          <span class="state-pill idle" id="mic-pill"><i class="dot"></i>idle</span>
          <p class="remark" id="mic-remark"></p>
          <button id="mic-deny" type="button">Mic won’t open</button>
        </div>
      </div>`;

    const readsEl = root.querySelector("#reads") as HTMLSelectElement;
    readsEl.value = reads;

    renderPicture(
      root.querySelector("#card-whistle") as HTMLElement,
      pictureModel({ fingering, notes: ["D4"], currentIndex: 0, showSolfege: false }),
    );

    const stairHost = root.querySelector("#stair-whistle") as HTMLElement;
    if (!hidden) {
      renderPicture(
        stairHost,
        pictureModel({ fingering, notes: stair, currentIndex: stairIndex, showSolfege: true }),
      );
    }

    const phraseHost = root.querySelector("#phrase-whistle") as HTMLElement;
    if (!hidden) {
      renderPicture(
        phraseHost,
        pictureModel({
          fingering,
          notes: phrase,
          currentIndex: 0,
          showSolfege: true,
          marks: ["cut", null, null, null, null],
        }),
      );
    }

    renderPicture(
      root.querySelector("#cut-whistle") as HTMLElement,
      pictureModel({
        fingering,
        notes: ["A4"],
        currentIndex: 0,
        showSolfege: true,
        marks: ["cut"],
      }),
    );

    const staff = root.querySelector("#staff") as HTMLElement;
    const showStaff = showSolfege(reads) && heard && !hidden;
    staff.hidden = !showStaff;
    if (showStaff) {
      abcjs.renderAbc(staff, "X:1\nM:3/4\nL:1/8\nK:D\nD2 E | F2 G | A3", {
        responsive: "resize",
        staffwidth: 480,
      });
      appendNameRows(
        staff,
        pictureModel({
          fingering,
          notes: phrase,
          currentIndex: 0,
          showSolfege: true,
        }),
      );
    }

    const remark = root.querySelector("#mic-remark") as HTMLElement;
    const pill = root.querySelector("#mic-pill") as HTMLElement;
    if (micDenied) {
      remark.textContent = "Couldn’t hear. Come a little closer to the mic and play it again.";
      pill.className = "state-pill idle";
      pill.innerHTML = `<i class="dot"></i>idle`;
    }

    root.querySelector("#stair-next")?.addEventListener("click", () => {
      stairIndex = (stairIndex + 1) % stair.length;
      draw();
    });
    readsEl.addEventListener("change", () => {
      reads = readsEl.value as Reads;
      draw();
    });
    root.querySelector("#hear")?.addEventListener("click", () => {
      tones?.stop();
      heard = true;
      const events = phraseEvents("D2 E | F2 G | A3", phrase, 293.66);
      tones = playTones(events, 1, () => {
        tones = null;
      });
      draw();
    });
    root.querySelector("#cut-hear")?.addEventListener("click", () => {
      tones?.stop();
      tones = playTones(ornamentEvents("cut", noteHz("A4", 293.66)), 1, () => {
        tones = null;
      });
    });
    root.querySelector("#continue")?.addEventListener("click", () => {
      continued = true;
      draw();
    });
    root.querySelector("#hide")?.addEventListener("click", () => {
      hidden = !hidden;
      draw();
    });
    root.querySelector("#mic-deny")?.addEventListener("click", () => {
      micDenied = true;
      draw();
    });
  };

  draw();
}
