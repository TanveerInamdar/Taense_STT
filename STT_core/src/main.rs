mod STT;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::signal::windows::ctrl_break;
// for struct formatting
use STT::{STT, check_cleanup_server_status, get_mic_audio};
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
    #[serde(rename = "type")]
    pub format_type: String,
}

fn response_parser(payload: &str){
    let parsed_response: Value = match serde_json::from_str(payload){
        Ok(json) => json,
        Err(error) => {
            println!("Failed to parse response: {}", error);
            return;
        }
    };

}


#[tokio::main] // allows async main functions
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let get_audio = get_mic_audio(5).await?;
    let res = STT(get_audio)?; // The ? extracts the String. Without this, it would return Ok(String)

    match check_cleanup_server_status().await { // no await? here because that would unwrap the result and wont work for match statements
        Ok(true) => {}
        Ok(false) => {
            eprintln!("Cleanup server responded, but is not healthy.");
            return Ok(());
        }
        Err(e) => {
            eprintln!("Cleanup server responded with error: {}", e);
            return Ok(());
        }
    }
    let client = reqwest::Client::new();
    println!("STT RESULT: {:?}", res);
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
        response_format: None,
    };
    let response = client.post("http://127.0.0.1:8081/v1/chat/completions")
        .json(&CleanupRequest)
        .send()
        .await?;

    println!("{:?}", response.status());
    let extracted_text = response.text().await?;
    println!("{}", extracted_text);
    println!("{extracted_text}");

    Ok(())
}