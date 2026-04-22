# opencode-zen

Auto-generated Rust SDK for OpenCode Zen API (chat completions endpoint).
Generated from a minimal OpenAPI 3.0 spec using `openapi-generator-cli`.

## Generation

```bash
# Minimal spec for chat/completions
SPEC=/tmp/chat-completions.yaml

# Generate Rust SDK
nix shell nixpkgs#openapi-generator-cli -c openapi-generator-cli generate \
  -i $SPEC \
  -g rust \
  -o . \
  --additional-properties=packageName=opencode-zen,packageVersion=0.1.0
```

### Generator Details

| Setting | Value |
|---------|-------|
| Generator | `openapi-generator-cli` |
| Target | `rust` (reqwest client library) |
| Source spec | Minimal OpenAPI 3.0 for chat/completions |
| API endpoint | `https://opencode.ai/zen/v1/chat/completions` |
| TLS | rustls (default), native-tls optional |

## Architecture

```
src/
├── apis/          # Endpoint methods
│   ├── default_api.rs  # create_chat_completion
│   ├── configuration.rs  # Client configuration
│   └── mod.rs
├── models/        # Request/response types
│   ├── chat_message.rs
│   ├── create_chat_completion_request.rs
│   ├── create_chat_completion_response.rs
│   ├── chat_choice.rs
│   ├── usage.rs
│   └── mod.rs
└── lib.rs         # Crate root
```

## Usage

```rust
use opencode_zen::apis::configuration::Configuration;
use opencode_zen::apis::default_api;
use opencode_zen::models::{CreateChatCompletionRequest, ChatMessage};

let mut config = Configuration::new();
config.base_path = "https://opencode.ai/zen/v1".to_string();
// Set your Zen API key
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
```

## Fixes Applied

Generated code required fixes for Rust 2021 + clippy compatibility:
- Removed `extern crate` declarations (lib.rs)
- Added `#[default]` attribute to Role enum
- Removed unnecessary `return` statements
- Added non-empty doc comments

## Stats

- **21** Rust source files (after fixing)
- **~500** lines of generated code
- **1** API endpoint (chat/completions)
- **5** typed models