pub mod attempt;
pub mod evidence;
pub mod fingering;
pub mod mic;
pub mod ornaments;
pub mod pitch;
pub mod rms;
pub mod synth;
pub mod types;

pub use attempt::{AttemptConfig, AttemptEngine, Frame};
pub use evidence::{Evidence, ListenState};
pub use types::{NoteName, WhistleProfile};
