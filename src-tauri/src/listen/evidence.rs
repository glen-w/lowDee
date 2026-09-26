use serde::{Deserialize, Serialize};

/// Sit-in evidence — UI picks the remark string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    Sealed,
    LowDHeld,
    EarlyBreak,
    OctaveThere,
    RestartedNotes,
    CouldntHear,
    CutTooLong,
    TapMissing,
    Abstain,
    PhraseOk,
    NoteFound,
    StillD,
    LetThemJoin,
    BrokeEarly,
    /// A gap landed inside a line, not on a breath mark.
    BreathChops,
    /// A note above the break fell back to the same fingering an octave down.
    Cracked,
    /// A short roll, or another tight gesture, split into separate notes.
    BecameNotes,
    /// A slide moved and did not arrive.
    SlideMissed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListenState {
    Idle,
    Wait,
    Sounding,
    Feedback,
}
