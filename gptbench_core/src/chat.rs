use reqwest::{Client};
use crate::models::{ChatRequest, ChatResponse};

pub async fn send_chat_request(api_key: &str, chat_request: ChatRequest) -> Result<ChatResponse, reqwest::Error> {
    let client = Client::new();
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&chat_request)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use crate::models::{ChatMessage, ChatRole};

    #[tokio::test]
    async fn test_chat() {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        let chat_request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                ChatMessage {
                    role: ChatRole::System,
                    content: "You are a helpful assistant.".to_string(),
                    name: None,
                },
                ChatMessage {
                    role: ChatRole::User,
                    content: "Who won the world series in 2020?".to_string(),
                    name: None,
                },
            ],
        };

        let task = send_chat_request(api_key.as_str(), chat_request);
        let response = task.await.unwrap();
        // assert!(response.status().is_success());

        assert!(!response.choices.is_empty());
        println!("{:?}", response.choices[0].message);
    }
}
