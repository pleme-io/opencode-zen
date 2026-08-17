//! The crate's usage example, as code the compiler checks.
//!
//! The README carried this as prose and it had FOUR compile errors: it set
//! `config.api_key` to a `String` (the field is `Option<ApiKey>`), it passed
//! the request straight to `create_chat_completion` (which takes a
//! `CreateChatCompletionParams`), it built a `ChatMessage` with a `String`
//! role (the field is a `Role` enum), and it used a field the request auth
//! never read anyway. A prose example is unverified documentation; this one
//! fails the build if it drifts.
//!
//! Run it with:  OPENCODE_ZEN_API_KEY=z-… cargo run --example chat

use opencode_zen::apis::configuration::Configuration;
use opencode_zen::apis::default_api::{self, CreateChatCompletionParams};
use opencode_zen::models::{chat_message::Role, ChatMessage, CreateChatCompletionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    // base_path already defaults to https://opencode.ai/zen/v1.
    // Auth is `bearer_access_token` — the service wants `Authorization: Bearer`.
    config.bearer_access_token = Some(std::env::var("OPENCODE_ZEN_API_KEY")?);

    let request = CreateChatCompletionRequest {
        model: "big-pickle".to_string(),
        messages: vec![ChatMessage {
            role: Role::User,
            content: "Hello!".to_string(),
        }],
        temperature: Some(0.7),
        ..Default::default()
    };

    let response = default_api::create_chat_completion(
        &config,
        CreateChatCompletionParams {
            create_chat_completion_request: request,
        },
    )
    .await?;

    // Every field on the response is optional, so there is no indexing here:
    // `choices[0]` would panic on an empty or error-shaped reply.
    let text = response
        .choices
        .unwrap_or_default()
        .into_iter()
        .next()
        .and_then(|c| c.message)
        .map(|m| m.content);

    match text {
        Some(t) => println!("{t}"),
        None => eprintln!("the response carried no choices"),
    }

    Ok(())
}
