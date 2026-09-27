mod STT;

use serde::{Serialize, Deserialize};
use tokio::signal::windows::ctrl_break;
// for struct formatting
use STT::{STT, check_STTserver_status};
// {
// "model": "your-model",
// "messages": [
// { "role": "system", "content": "You are a helpful assistant that outputs only valid JSON." },
// { "role": "user", "content": "Extract the name and age from: John is 30 years old." }
// ],
// "response_format": { "type": "json_object" }
// }

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String, // You can also use an enum here (e.g., System, User, Assistant)
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostRequestBody {
    pub model: String,
    pub messages: Vec<Message>,
    pub response_format: Option<ResponseFormat>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseFormat {
    pub format_type: String,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let res = STT()?; // The ? extracts the String. Without this, it would return Ok(String)
    // println!("{:?}", res);
    if check_STTserver_status() == false{
        return Err("The STT server aint runnin".into());
    }
    let client = reqwest::blocking::Client::new();
    let CleanupRequest = PostRequestBody{ // should be moved out later
        model: "your-model".to_string(),
        messages: vec![
            Message{ 
            role: "system".to_string(),
            content: "You are a helpful assistant that outputs only valid JSON. Convert the following Speech to text result into a valid text. Add proper grammar.".to_string()},
            Message{ 
            role: "user".to_string(),
            content: res}
        ],
        response_format: Some(ResponseFormat{format_type: "json_object".to_string()})
    };
    let response = client.post("http://127.0.0.1:8081/v1/chat/completions")
        .json(&CleanupRequest)
        .send();

    // println!("{:}", response.status());
    println!("{:?}", response.unwrap().text());


    // # async fn run() -> Result<(), Error> {
    // let client = reqwest::Client::new();

    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()
    //     .await?;
    //  Ok(())
    //  }
    Ok(())


    // // env_logger::init();   ONLY USE WHILE DEBUGGING
    //
    // let model_path = PathBuf::from("models/parakeet-tdt-0.6b-v3-int8"); // This is the path to the model
    //
    // let wav_path = PathBuf::from("C:/Coding_Projects/Taense_STT/samples/jfk.wav");  // This is the path to the audio file
    //
    //
    // let audio_duration = get_audio_duration(&wav_path)?;
    // println!("Audio duration: {:.2}s", audio_duration);
    //
    // println!("Using Parakeet engine");
    // println!("Loading model: {:?}", model_path);
    //
    // let load_start = Instant::now();
    // let mut model = ParakeetModel::load(&model_path, &Quantization::Int8)?;
    // let load_duration = load_start.elapsed();
    // println!("Model loaded in {:.2?}", load_duration);
    //
    // println!("Transcribing file: {:?}", wav_path);
    // let transcribe_start = Instant::now();
    //
    // let samples = transcribe_rs::audio::read_wav_samples(&wav_path)?;
    // let result = model.transcribe_with(
    //     &samples,
    //     &ParakeetParams {
    //         timestamp_granularity: Some(TimestampGranularity::Segment),
    //         ..Default::default()
    //     },
    // )?;
    // let transcribe_duration = transcribe_start.elapsed();
    // println!("Transcription completed in {:.2?}", transcribe_duration);
    //
    // let speedup_factor = audio_duration / transcribe_duration.as_secs_f64();
    // println!(
    //     "Real-time speedup: {:.2}x faster than real-time",
    //     speedup_factor
    // );
    //
    // println!("Transcription result:");
    // println!("{}", result.text);
    // let STT_result = result.text;
    //
    // if let Some(segments) = result.segments {
    //     println!("\nSegments:");
    //     for segment in segments {
    //         println!(
    //             "[{:.2}s - {:.2}s]: {}",
    //             segment.start, segment.end, segment.text
    //         );
    //     }
    // }
    //
    // Ok(())
}