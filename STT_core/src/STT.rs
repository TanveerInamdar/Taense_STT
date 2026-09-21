// This file is main.rs without the print statements.

use std::path::PathBuf;
use std::time::Instant;

use transcribe_rs::onnx::parakeet::{ParakeetModel, ParakeetParams, TimestampGranularity};
use transcribe_rs::onnx::Quantization;

fn get_audio_duration(path: &PathBuf) -> Result<f64, Box<dyn std::error::Error>> {
    let reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let duration = reader.duration() as f64 / spec.sample_rate as f64;
    Ok(duration)
}

fn STT() -> Result<(), Box<dyn std::error::Error>> {

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
    println!("{:#?}", result);
    let STT_result = result.text;
    Ok(())
}