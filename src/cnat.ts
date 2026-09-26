import type { Pack } from "./types";

/** The pack's first picture. The alternate is one other cross-fingering, not a half-hole. */
export const CNAT_FIRST = "oxxooo";
export const CNAT_ALT = "oxxoxx";

const ALT_HOLES = ["open", "closed", "closed", "open", "closed", "closed"] as const;

export function withAltC(fingering: Pack["fingering"]): Pack["fingering"] {
  const chart = fingering.notes.C5;
  if (!chart) return fingering;
  return {
    ...fingering,
    notes: {
      ...fingering.notes,
      C5: { ...chart, holes: [...ALT_HOLES], half: false },
    },
  };
}
