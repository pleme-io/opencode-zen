# opencode-zen

Auto-generated Rust SDK for OpenCode Zen API (chat completions endpoint).

## Usage

```rust
use opencode_zen::apis::configuration::Configuration;
use opencode_zen::apis::default_api;
use opencode_zen::models::{CreateChatCompletionRequest, ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.base_path = "https://opencode.ai/zen/v1".to_string();
    config.api_key = Some("z-xxxxxxxxxxxxx".to_string());

    let request = CreateChatCompletionRequest {
        model: "big-pickle".to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        }],
        temperature: Some(0.7),
        ..Default::default()
    };

    let response = default_api::create_chat_completion(&config, request).await?;
    println!("{}", response.choices[0].message.content);

    Ok(())
}
```

## Available Models

- `big-pickle` (free)
- `minimax-m2.5-free` (free)
- `nemotron-3-super-free` (free)
- `glm-4.7-free` (free)
- `kimi-k2.5-free` (free)
- Plus paid models (gpt-5, claude, gemini, etc.)