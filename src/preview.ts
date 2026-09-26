import abcjs from "abcjs";
import fingeringJson from "../pack/may-morning-dew/fingering-low-d.json";
import {
  appendNameRows,
  pictureModel,
  renderPicture,
  showSolfege,
  type Fingering,
  type Reads,
} from "./picture.ts";

const fingering = fingeringJson as Fingering;
const stair = ["D4", "E4", "F#4", "G4", "A4", "B4"];
const phrase = ["D4", "E4", "F#4", "G4", "A4"];

export function mountPreview(root: HTMLElement): void {
  let stairIndex = 0;
  let reads: Reads = "no";
  let heard = false;
  let hidden = false;
  let micDenied = false;

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
        pictureModel({ fingering, notes: stair, currentIndex: stairIndex, showSolfege: false }),
      );
    }

    const phraseHost = root.querySelector("#phrase-whistle") as HTMLElement;
    if (!hidden) {
      renderPicture(
        phraseHost,
        pictureModel({ fingering, notes: phrase, currentIndex: 0, showSolfege: false }),
      );
    }

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
      heard = true;
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
