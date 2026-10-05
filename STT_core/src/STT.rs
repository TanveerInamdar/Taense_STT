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

// fn get_audio_duration(path: &PathBuf) -> Result<f64, Box<dyn std::error::Error>> {
//     let reader = hound::WavReader::open(path)?;
//     let spec = reader.spec();
//     let duration = reader.duration() as f64 / spec.sample_rate as f64;
//     Ok(duration)
// }
 
pub async fn get_mic_audio(seconds: u64) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let audio_data = Arc::new(Mutex::new(Vec::<f32>::new()));
    let host = cpal::default_host();    

    let device = host.default_input_device().expect("no input device available");
    let config = device.default_input_config()?.config();
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

    Ok(audio_data.lock().unwrap().to_vec())
}
pub fn check_cleanup_server_status() -> Result<bool, reqwest::Error> { // Checks if the STT server is running
    let response = reqwest::blocking::get("http://localhost:8081/health")?;
    // response.unwrap().status() == StatusCode::OK
    Ok(response.status().is_success())
}
pub fn STT() -> Result<String, Box<dyn std::error::Error>> {

    let model_path = PathBuf::from("models/parakeet-tdt-0.6b-v3-int8"); // This is the path to the model

    let wav_path = PathBuf::from("C:/Coding_Projects/Taense_STT/samples/jfk.wav");  // This is the path to the audio file

    let mut model = ParakeetModel::load(&model_path, &Quantization::Int8)?;

    let samples = transcribe_rs::audio::read_wav_samples(&wav_path)?;
    let result = model.transcribe_with(
        &samples,
        &ParakeetParams {
            timestamp_granularity: Some(TimestampGranularity::Segment),
            ..Default::default()
        },
    )?;
    // println!("{:#?}", result);
    let STT_result = result.text;
    Ok(STT_result)
}