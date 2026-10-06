use STT_core::STT::{get_mic_audio};
#[cfg(test)]
mod tests {
    use super::*; // Brings the 'add' function from the parent scope into focus

    // 3. The individual test function
    #[tokio:: test]
    async fn test_mic() {
        let res1test = get_mic_audio(2).await;
        println!("Test results : {:?}", res1test);

    }
}