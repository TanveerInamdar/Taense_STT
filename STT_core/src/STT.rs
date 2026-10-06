// This file is main.rs without the print statements.
use cpal::{Data, Sample, SampleFormat, FromSample, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::{sleep, Duration};

// use std::time::Instant;
use reqwest::StatusCode;
use transcribe_rs::onnx::parakeet::{ParakeetModel, ParakeetParams, TimestampGranularity};
use transcribe_rs::onnx::Quantization;

fn to_mono_16k(input: &[f32], channels: u16, sample_rate: u32) -> Vec<f32> {
    const TARGET_RATE: u32 = 16_000;

    // Downmix: average the channels of each interleaved frame
    let ch = channels as usize;
    let mono: Vec<f32> = if ch > 1 {
        input
            .chunks_exact(ch)
            .map(|frame| frame.iter().sum::<f32>() / ch as f32)
            .collect()
    } else {
        input.to_vec()
    };

    if sample_rate == TARGET_RATE || mono.is_empty() {
        return mono;
    }

    // Resample to 16 kHz with linear interpolation
    let ratio = sample_rate as f64 / TARGET_RATE as f64;
    let out_len = (mono.len() as f64 / ratio).floor() as usize;

    (0..out_len)
        .map(|i| {
            let pos = i as f64 * ratio;
            let idx = pos as usize;
            let frac = (pos - idx as f64) as f32;
            let a = mono[idx];
            let b = *mono.get(idx + 1).unwrap_or(&a);
            a + (b - a) * frac
        })
        .collect()
}
 
pub async fn get_mic_audio(seconds: u64) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let audio_data = Arc::new(Mutex::new(Vec::<f32>::new()));
    let host = cpal::default_host();    

    let device = host.default_input_device().expect("no input device available");
    let config = device.default_input_config()?.config();
    println!("Sample rate: {}", config.sample_rate);
    println!("Channels: {}", config.channels);
    let in_channels = config.channels;
    let in_rate = config.sample_rate;


    let audio_data_clone = Arc::clone(&audio_data);

    let record = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buffer = audio_data_clone.lock().unwrap();
            buffer.extend_from_slice(data);
        },
        move |err| {
            // react to errors here.
        },
        None // Timeout for stream initialization: None = wait indefinitely,
    )?;
    record.play();
    sleep(Duration::from_secs(seconds)).await; // 5 seconds async wait time to record audio
    record.pause();
    let raw_audio = audio_data.lock().unwrap().to_vec();
    Ok(to_mono_16k(&raw_audio, in_channels, in_rate))
}
pub async fn check_cleanup_server_status() -> Result<bool, reqwest::Error> { // Checks if the STT server is running
    let response = reqwest::get("http://localhost:8081/health").await?;
    // response.unwrap().status() == StatusCode::OK
    Ok(response.status().is_success())
}
pub fn STT(audio:Vec<f32>) -> Result<String, Box<dyn std::error::Error>> {

    let model_path = PathBuf::from("models/parakeet-tdt-0.6b-v3-int8"); // This is the path to the model

    // let wav_path = PathBuf::from("C:/Coding_Projects/Taense_STT/samples/jfk.wav");  // This is the path to the audio file

    let mut model = ParakeetModel::load(&model_path, &Quantization::Int8)?;

    // let samples = transcribe_rs::audio::read_wav_samples(&wav_path)?;
    let result = model.transcribe_with(
        &audio,
        &ParakeetParams {
            timestamp_granularity: Some(TimestampGranularity::Segment),
            ..Default::default()
        },
    )?;
    // println!("{:#?}", result);
    let STT_result = result.text;
    Ok(STT_result)
}