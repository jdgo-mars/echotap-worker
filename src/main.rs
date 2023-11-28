use std::{fs::{self, File}, str::Bytes};

use reqwest::{header, Client, multipart::Part};


struct OpenAiClient {
    client: reqwest::Client
}

impl OpenAiClient {
    fn new() -> OpenAiClient {
        let mut headers: reqwest::header::HeaderMap = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_static(
                "Bearer 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",
            ),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("multipart/form-data"),
        );

        let client: Client = Client::builder().default_headers(headers).build().unwrap();
        client
        OpenAiClient { client }
    }

    async fn request_transcription(&self) {
        let response = self.client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .multipart(form)
        .send() 
        .await?;
    }

}



#[tokio::main]
async fn main() -> anyhow::Result<()> {



    
    let f = fs::read("/Users/mdj/projects/echotap/fixtures/alloy.mp3")?;
    
    let form = reqwest::multipart::Form::new()
        .part("file", Part::bytes(f).file_name("allow.mp3").mime_str("audio/mp3")?)
        .text("model", "whisper-1");


    println!("{:?}", response.text().await?);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test function to validate the 'add' function
    #[test]
    fn it_works() {
        // Define the input values
        let a = 5;
        let b = 10;

        // Call the function being tested
        let result = add(a, b);

        // Assert that the result matches the expected value
        assert_eq!(result, 15);
    }
}