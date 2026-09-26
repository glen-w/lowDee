//! Mic capture thread — feeds AttemptEngine. Playback forces wait (no grading).
//!
//! cpal::Stream is !Send on macOS, so the stream lives on a dedicated thread.
//! AppState only holds Arc control handles (Send + Sync).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;

use super::attempt::{AttemptConfig, AttemptEngine, AttemptResult, Frame};

pub struct MicSession {
    pub engine: Arc<Mutex<AttemptEngine>>,
    pub latest_frame: Arc<Mutex<Option<Frame>>>,
    pub grading_enabled: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    _worker: Option<JoinHandle<()>>,
}

impl MicSession {
    pub fn start(cfg: AttemptConfig) -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| "No input device".to_string())?;
        let supported = device.default_input_config().map_err(|e| e.to_string())?;

        let sample_rate = supported.sample_rate().0;
        let mut cfg = cfg;
        cfg.sample_rate = sample_rate;

        let engine = Arc::new(Mutex::new(AttemptEngine::new(cfg)));
        let latest_frame = Arc::new(Mutex::new(None));
        let grading_enabled = Arc::new(AtomicBool::new(true));
        let stop = Arc::new(AtomicBool::new(false));

        let eng = engine.clone();
        let frame_slot = latest_frame.clone();
        let grading = grading_enabled.clone();
        let stop_flag = stop.clone();
        let sample_format = supported.sample_format();
        let stream_config: cpal::StreamConfig = supported.into();

        let worker = thread::Builder::new()
            .name("low-d-mic".into())
            .spawn(move || {
                let host = cpal::default_host();
                let Ok(device) = host.default_input_device().ok_or(()) else {
                    return;
                };
                let stream = match sample_format {
                    cpal::SampleFormat::F32 => {
                        build_stream::<f32>(&device, &stream_config, eng, frame_slot, grading)
                    }
                    cpal::SampleFormat::I16 => {
                        build_stream::<i16>(&device, &stream_config, eng, frame_slot, grading)
                    }
                    cpal::SampleFormat::U16 => {
                        build_stream::<u16>(&device, &stream_config, eng, frame_slot, grading)
                    }
                    _ => return,
                };
                let Ok(stream) = stream else {
                    return;
                };
                let _ = stream.play();
                while !stop_flag.load(Ordering::SeqCst) {
                    thread::sleep(std::time::Duration::from_millis(50));
                }
                drop(stream);
            })
            .map_err(|e| e.to_string())?;

        Ok(Self {
            engine,
            latest_frame,
            grading_enabled,
            stop,
            _worker: Some(worker),
        })
    }

    pub fn set_grading(&self, on: bool) {
        self.grading_enabled.store(on, Ordering::SeqCst);
    }

    pub fn finish(&self) -> AttemptResult {
        self.stop.store(true, Ordering::SeqCst);
        self.engine.lock().finish()
    }

    pub fn frozen_target(&self) -> Option<f32> {
        self.engine.lock().frozen_target_hz()
    }
}

impl Drop for MicSession {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    engine: Arc<Mutex<AttemptEngine>>,
    frame_slot: Arc<Mutex<Option<Frame>>>,
    grading: Arc<AtomicBool>,
) -> Result<cpal::Stream, String>
where
    T: cpal::SizedSample + Send + 'static,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels as usize;
    let stream = device
        .build_input_stream(
            config,
            move |data: &[T], _| {
                if !grading.load(Ordering::SeqCst) {
                    return;
                }
                let mono: Vec<f32> = data
                    .chunks(channels)
                    .map(|c| {
                        let sum: f32 = c.iter().map(|s| cpal::Sample::to_sample::<f32>(*s)).sum();
                        sum / channels as f32
                    })
                    .collect();
                let mut eng = engine.lock();
                if let Some(frame) = eng.push_samples(&mono) {
                    *frame_slot.lock() = Some(frame);
                }
            },
            |err| eprintln!("mic stream error: {err}"),
            None,
        )
        .map_err(|e| e.to_string())?;
    Ok(stream)
}
