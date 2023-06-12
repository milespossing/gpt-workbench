use std::env;
use std::fs::File;
use std::path::Path;
use clap::{Parser};
use serde::Deserialize;
use gptbench_core::chat::send_chat_request;
use gptbench_core::models::{ChatMessage, ChatRequest};

const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
const DEFAULT_SYSTEM_MESSAGE: &str = "You are a helpful assistant";

#[derive(Parser)]
struct Args {
    prompt: String,
    #[arg(short = 'c', long = "config")]
    config_file: Option<String>,
    #[arg(long = "key")]
    api_key: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "systemPrompt")]
    system_prompt: Option<String>,
    model: Option<String>,
}

const DEFAULT_CONFIG: Config = Config {
    system_prompt: None,
    model: None,
};

impl Config {
    fn into_chat_request(self) -> ChatRequest {
        ChatRequest {
            // system_prompt: self.system_prompt.unwrap_or_else(|| DEFAULT_CONFIG.system_prompt.unwrap()),
            model: self.model.unwrap_or(String::from(DEFAULT_MODEL)),
            messages: vec![
                ChatMessage::system_message(self.system_prompt.unwrap_or(String::from(DEFAULT_SYSTEM_MESSAGE))),

            ],
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();

    let config: Config = match cli.config_file {
        Some(path) => {
            // read path as ConfigFile
            let path = Path::new(&path);
            let file = match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening config file: {}", e);
                    return Ok(());
                }
            };
            match serde_json::from_reader(file) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error parsing config file: {}", e);
                    return Ok(());
                }
            }
        },
        None => {
            DEFAULT_CONFIG
        }
    };

    let chat_request = config.into_chat_request().push_user_message(cli.prompt);

    let api_key = match cli.api_key {
        Some(api_key) => api_key,
        None => {
            match env::var("OPENAPI_KEY") {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("No API key provided.");
                    return Ok(());
                }
            }
        }
    };

    let response = send_chat_request(&api_key, chat_request).await?;

    println!("{}", response.choices[0].message.content);
    Ok(())
}