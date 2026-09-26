use serde::{Deserialize, Serialize};

/// Concert D4 ≈ 293.66 Hz — only used before a profile exists, with wide tolerance.
pub const DEFAULT_LOW_D_HZ: f32 = 293.66;
pub const SETTLE_SECONDS: f32 = 10.0;
pub const BAND_LOW_HZ: f32 = 250.0;
pub const BAND_HIGH_HZ: f32 = 1600.0;
pub const CENTS_TOLERANCE_PRE_CAL: f32 = 80.0;
pub const CENTS_TOLERANCE_POST_CAL: f32 = 50.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhistleProfile {
    pub profile_id: String,
    pub label: String,
    pub break_hz: f32,
    pub rms_floor: f32,
    pub cal_as_of: String,
    pub reads: Reads,
    pub background: Background,
    /// Which C-natural fingering held on this stick. Empty until that pack settles.
    #[serde(default)]
    pub cnat_fingering: Option<String>,
    /// Hear starts at the whistle when a book recording is the model.
    #[serde(default = "default_true")]
    pub skip_book_talk: bool,
    /// Hands-and-breath pass when a stored profile opens the app.
    #[serde(default = "default_true")]
    pub warmup_on_launch: bool,
    /// Song packs this whistle has added. Empty until one is added.
    #[serde(default)]
    pub lesson_packs: Vec<String>,
    /// What happens after a settled section. Missing means the next one in this part.
    #[serde(default)]
    pub auto_advance: AutoAdvance,
}

/// How far a settled section walks without another click.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AutoAdvance {
    #[default]
    Inside,
    Across,
    Highlight,
    Off,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reads {
    No,
    Some,
    Yes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Background {
    None,
    Wind,
    Other,
    #[serde(rename = "high_d")]
    HighD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoteName {
    D4,
    E4,
    Fs4,
    G4,
    A4,
    B4,
    C5,
    Cs5,
    D5,
    E5,
    Fs5,
    G5,
    A5,
    B5,
}

impl NoteName {
    pub const ALL: [NoteName; 14] = [
        NoteName::D4,
        NoteName::E4,
        NoteName::Fs4,
        NoteName::G4,
        NoteName::A4,
        NoteName::B4,
        NoteName::C5,
        NoteName::Cs5,
        NoteName::D5,
        NoteName::E5,
        NoteName::Fs5,
        NoteName::G5,
        NoteName::A5,
        NoteName::B5,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            NoteName::D4 => "D4",
            NoteName::E4 => "E4",
            NoteName::Fs4 => "F#4",
            NoteName::G4 => "G4",
            NoteName::A4 => "A4",
            NoteName::B4 => "B4",
            NoteName::C5 => "C5",
            NoteName::Cs5 => "C#5",
            NoteName::D5 => "D5",
            NoteName::E5 => "E5",
            NoteName::Fs5 => "F#5",
            NoteName::G5 => "G5",
            NoteName::A5 => "A5",
            NoteName::B5 => "B5",
        }
    }

    pub fn semitones_from_d4(self) -> i32 {
        match self {
            NoteName::D4 => 0,
            NoteName::E4 => 2,
            NoteName::Fs4 => 4,
            NoteName::G4 => 5,
            NoteName::A4 => 7,
            NoteName::B4 => 9,
            NoteName::C5 => 10,
            NoteName::Cs5 => 11,
            NoteName::D5 => 12,
            NoteName::E5 => 14,
            NoteName::Fs5 => 16,
            NoteName::G5 => 17,
            NoteName::A5 => 19,
            NoteName::B5 => 21,
        }
    }

    /// Same fingering, one octave down. Notes at the break and below have none.
    pub fn octave_down(self) -> Option<NoteName> {
        match self {
            NoteName::D5 => Some(NoteName::D4),
            NoteName::E5 => Some(NoteName::E4),
            NoteName::Fs5 => Some(NoteName::Fs4),
            NoteName::G5 => Some(NoteName::G4),
            NoteName::A5 => Some(NoteName::A4),
            NoteName::B5 => Some(NoteName::B4),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "D4" => Some(NoteName::D4),
            "E4" => Some(NoteName::E4),
            "F#4" | "Fs4" => Some(NoteName::Fs4),
            "G4" => Some(NoteName::G4),
            "A4" => Some(NoteName::A4),
            "B4" => Some(NoteName::B4),
            "C5" => Some(NoteName::C5),
            "C#5" | "Cs5" => Some(NoteName::Cs5),
            "D5" | "D'" => Some(NoteName::D5),
            "E5" => Some(NoteName::E5),
            "F#5" | "Fs5" => Some(NoteName::Fs5),
            "G5" => Some(NoteName::G5),
            "A5" => Some(NoteName::A5),
            "B5" => Some(NoteName::B5),
            _ => None,
        }
    }
}

/// Target Hz from this whistle's break, not A440.
pub fn target_hz(break_hz: f32, note: NoteName) -> f32 {
    break_hz * 2f32.powf(note.semitones_from_d4() as f32 / 12.0)
}

pub fn cents_between(a: f32, b: f32) -> f32 {
    if a <= 0.0 || b <= 0.0 {
        return f32::INFINITY;
    }
    1200.0 * (a / b).log2()
}
