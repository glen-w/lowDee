//! listen CLI — synthetic fixtures + JSON traces for the bench.

use std::env;
use std::fs;
use std::path::PathBuf;

use low_d_lib::listen::attempt::{AttemptConfig, AttemptEngine, AttemptMode};
use low_d_lib::listen::ornaments::GestureKind;
use low_d_lib::listen::synth;
use low_d_lib::listen::take;
use low_d_lib::listen::types::{NoteName, DEFAULT_LOW_D_HZ};
use low_d_lib::listen::Evidence;
use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    match cmd {
        "fixtures" => run_fixtures(),
        "grade" => {
            let name = args.get(2).map(|s| s.as_str()).unwrap_or("steady_low_d");
            grade_named(name);
        }
        "gate" => run_gate_protocol(),
        "grade-take" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if path.is_empty() {
                eprintln!("listen grade-take <sidecar.json>");
                std::process::exit(2);
            }
            match take::grade_spec(std::path::Path::new(path)) {
                Ok(grade) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&json!({
                            "evidence": grade.evidence,
                            "settled": grade.settled,
                            "pass": grade.pass,
                        }))
                        .unwrap()
                    );
                    if !grade.pass {
                        std::process::exit(1);
                    }
                }
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(2);
                }
            }
        }
        "help" | _ => {
            eprintln!("listen fixtures | listen grade <name> | listen gate | listen grade-take <sidecar.json>");
            eprintln!("fixtures: write wav + traces under bench/fixtures and bench/out");
            eprintln!("grade: run one named fixture and print JSON");
            eprintln!(
                "gate: beginner protocol on synthetic audio (settle, early break, retry, phrase)"
            );
            eprintln!("grade-take: score a wav named by a sidecar (corpus/ or bench/takes/)");
        }
    }
}

fn out_dirs() -> (PathBuf, PathBuf) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    let repo = root.join("..");
    let fixtures = repo.join("bench/fixtures");
    let out = repo.join("bench/out");
    fs::create_dir_all(&fixtures).ok();
    fs::create_dir_all(&out).ok();
    (fixtures, out)
}

fn run_engine(cfg: AttemptConfig, samples: &[f32]) -> low_d_lib::listen::attempt::AttemptResult {
    let mut eng = AttemptEngine::new(cfg);
    for chunk in samples.chunks(1024) {
        eng.push_samples(chunk);
    }
    eng.finish()
}

fn run_fixtures() {
    let sr = 44100u32;
    let hz = DEFAULT_LOW_D_HZ;
    let (fixtures, out) = out_dirs();

    let cases: Vec<(&str, Vec<f32>, AttemptConfig)> = vec![
        (
            "steady_low_d",
            synth::fixture_steady_low_d(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
        ),
        (
            "early_break",
            synth::fixture_early_break(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::BreathOctave { want_octave: false },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "retry_settle",
            synth::fixture_retry_settle(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
        ),
        (
            "noise",
            synth::fixture_noise(sr),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
        ),
        (
            "octave",
            synth::fixture_octave(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::BreathOctave { want_octave: true },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "cut_ok",
            synth::fixture_cut(sr, target(hz, NoteName::A4)),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 10.0,
                mode: AttemptMode::Ornament {
                    note: NoteName::A4,
                    gesture: GestureKind::Cut,
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "cut_too_long",
            synth::fixture_cut_too_long(sr, target(hz, NoteName::A4)),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 10.0,
                mode: AttemptMode::Ornament {
                    note: NoteName::A4,
                    gesture: GestureKind::Cut,
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "tongued",
            synth::fixture_tongued_phrase(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::OnTheBreath {
                    notes: vec![NoteName::D4, NoteName::E4, NoteName::Fs4, NoteName::G4],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "joined",
            synth::fixture_joined_phrase(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::OnTheBreath {
                    notes: vec![NoteName::D4, NoteName::E4, NoteName::Fs4, NoteName::G4],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "leaked_c",
            synth::fixture_leaked_c(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::C5 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "octave_e_crack",
            synth::fixture_octave_e_cracks(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::E5 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "a_dorian",
            synth::fixture_a_dorian(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::A4, NoteName::G4, NoteName::C5, NoteName::A4],
                    breaths: vec![],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "breath_gap",
            synth::fixture_two_notes_gap(sr, hz, 0.4),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::D4, NoteName::E4],
                    breaths: vec![0],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "breath_chop",
            synth::fixture_two_notes_gap(sr, hz, 0.4),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::D4, NoteName::E4],
                    breaths: vec![1],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        (
            "short_roll_split",
            synth::fixture_roll_split(sr, target(hz, NoteName::A4)),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 10.0,
                mode: AttemptMode::Ornament {
                    note: NoteName::A4,
                    gesture: GestureKind::ShortRoll,
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
    ];

    let mut summary = Vec::new();
    for (name, samples, cfg) in cases {
        let wav = fixtures.join(format!("{name}.wav"));
        synth::write_wav(&wav, sr, &samples).expect("wav");
        let result = run_engine(cfg, &samples);
        let target_before = result.target_hz;
        let trace = json!({
            "name": name,
            "evidence": result.evidence,
            "settled": result.settled,
            "break_hz": result.break_hz,
            "rms_floor": result.rms_floor,
            "target_hz": target_before,
            "frame_count": result.frames.len(),
            "frames": result.frames.iter().map(|f| json!({
                "t_ms": f.t_ms,
                "hz": f.hz,
                "rms": f.rms,
                "confidence": f.confidence,
                "near_target": f.near_target,
                "early_break": f.early_break,
            })).collect::<Vec<_>>(),
        });
        let trace_path = out.join(format!("{name}.trace.json"));
        fs::write(&trace_path, serde_json::to_string_pretty(&trace).unwrap()).ok();
        println!(
            "{name}: {:?} settled={} target={:?}",
            result.evidence, result.settled, target_before
        );
        summary.push(json!({
            "name": name,
            "evidence": result.evidence,
            "settled": result.settled,
            "target_hz": target_before,
        }));
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .ok();
}

fn target(break_hz: f32, note: NoteName) -> f32 {
    low_d_lib::listen::types::target_hz(break_hz, note)
}

fn grade_named(name: &str) {
    let sr = 44100u32;
    let hz = DEFAULT_LOW_D_HZ;
    let (samples, cfg) = match name {
        "steady_low_d" => (
            synth::fixture_steady_low_d(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
        ),
        "early_break" => (
            synth::fixture_early_break(sr, hz),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::BreathOctave { want_octave: false },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
        ),
        "noise" => (
            synth::fixture_noise(sr),
            AttemptConfig {
                sample_rate: sr,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
        ),
        other => {
            eprintln!("unknown fixture {other}");
            return;
        }
    };
    let result = run_engine(cfg, &samples);
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "name": name,
            "evidence": result.evidence,
            "settled": result.settled,
            "break_hz": result.break_hz,
            "target_hz": result.target_hz,
        }))
        .unwrap()
    );
}

/// Beginner gate: low D settle → early break remark → retry same target → first phrase.
fn run_gate_protocol() {
    let sr = 44100u32;
    let hz = DEFAULT_LOW_D_HZ;

    // 1. Settle
    let settle = run_engine(
        AttemptConfig {
            sample_rate: sr,
            hop_ms: 30.0,
            mode: AttemptMode::FirstSound,
            break_hz: None,
            rms_floor: None,
            recalibrate: false,
        },
        &synth::fixture_steady_low_d(sr, hz),
    );
    assert_eq!(
        settle.evidence,
        Evidence::LowDHeld,
        "settle must hold low D"
    );
    let break_hz = settle.break_hz.expect("calibration hz");
    let rms_floor = settle.rms_floor.expect("calibration rms");
    let _target_1 = settle.target_hz;
    println!("1 settle: {:?} break_hz={break_hz:.2}", settle.evidence);

    // 2. Early break — target frozen at low
    let early = run_engine(
        AttemptConfig {
            sample_rate: sr,
            hop_ms: 30.0,
            mode: AttemptMode::BreathOctave { want_octave: false },
            break_hz: Some(break_hz),
            rms_floor: Some(rms_floor),
            recalibrate: false,
        },
        &synth::fixture_early_break(sr, break_hz),
    );
    assert_eq!(
        early.evidence,
        Evidence::EarlyBreak,
        "must remark early break"
    );
    assert_eq!(early.target_hz, Some(break_hz), "target must not drift");
    println!(
        "2 early_break: {:?} target={:?}",
        early.evidence, early.target_hz
    );

    // 3. Retry settle — same break_hz
    let retry = run_engine(
        AttemptConfig {
            sample_rate: sr,
            hop_ms: 30.0,
            mode: AttemptMode::BreathOctave { want_octave: false },
            break_hz: Some(break_hz),
            rms_floor: Some(rms_floor),
            recalibrate: false,
        },
        &synth::fixture_retry_settle(sr, break_hz),
    );
    assert!(
        matches!(retry.evidence, Evidence::LowDHeld),
        "retry should settle low: {:?}",
        retry.evidence
    );
    assert_eq!(retry.target_hz, Some(break_hz), "retry target unchanged");
    println!("3 retry: {:?} target={:?}", retry.evidence, retry.target_hz);

    // 4. First phrase against same target
    let phrase = run_engine(
        AttemptConfig {
            sample_rate: sr,
            hop_ms: 30.0,
            mode: AttemptMode::Phrase {
                notes: vec![NoteName::D4, NoteName::E4, NoteName::Fs4, NoteName::G4],
                breaths: vec![],
                marks: vec![],
            },
            break_hz: Some(break_hz),
            rms_floor: Some(rms_floor),
            recalibrate: false,
        },
        &synth::fixture_joined_phrase(sr, break_hz),
    );
    assert!(
        matches!(phrase.evidence, Evidence::PhraseOk | Evidence::Sealed),
        "phrase should be hearable: {:?}",
        phrase.evidence
    );
    // Stronger: prefer PhraseOk when all notes speak
    println!(
        "4 phrase: {:?} target={:?} settled={}",
        phrase.evidence, phrase.target_hz, phrase.settled
    );
    assert!(
        (phrase.target_hz.unwrap_or(0.0) - break_hz).abs() < 1.0,
        "phrase target must be profile break"
    );

    // Noise must not advance
    let noise = run_engine(
        AttemptConfig {
            sample_rate: sr,
            hop_ms: 30.0,
            mode: AttemptMode::FirstSound,
            break_hz: None,
            rms_floor: None,
            recalibrate: false,
        },
        &synth::fixture_noise(sr),
    );
    assert_eq!(noise.evidence, Evidence::CouldntHear);
    assert!(!noise.settled);
    println!("5 noise: {:?} (no advance)", noise.evidence);

    println!("GATE PASS");
}
