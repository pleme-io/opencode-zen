# opencode-zen

Auto-generated Rust SDK for OpenCode Zen API (chat completions endpoint).

## Usage

The example lives at [`examples/chat.rs`](examples/chat.rs) and is compiled by
`cargo check --examples`, so it cannot drift from the API:

```sh
OPENCODE_ZEN_API_KEY=z-xxxxxxxxxxxxx cargo run --example chat
```

```rust
let mut config = Configuration::new();
// base_path already defaults to https://opencode.ai/zen/v1
config.bearer_access_token = Some(std::env::var("OPENCODE_ZEN_API_KEY")?);

let request = CreateChatCompletionRequest {
    model: "big-pickle".to_string(),
    messages: vec![ChatMessage { role: Role::User, content: "Hello!".to_string() }],
    temperature: Some(0.7),
    ..Default::default()
};

let response = default_api::create_chat_completion(
    &config,
    CreateChatCompletionParams { create_chat_completion_request: request },
).await?;
```

Three things to know, all of which the previous version of this section got
wrong — it was prose, nothing compiled it, and it had four type errors:

- **Auth is `bearer_access_token`,** not `api_key`. The service wants
  `Authorization: Bearer <key>`. `api_key` is an `Option<ApiKey>` struct and is
  not consulted by the request path.
- **`role` is a `Role` enum** (`Role::User`), not a string — an invalid role is
  unrepresentable rather than a runtime 400.
- **Every response field is `Option`.** `choices[0]` panics on an
  error-shaped reply; the example unwraps deliberately.

> **Fixed 2026-08-17: auth was never applied.** `create_chat_completion` did
> not read `bearer_access_token`, so every request went out unauthenticated and
> this crate could not have worked against the live service. That is why it
> went unused: `tend` depended on it and then hand-rolled the same call with
> reqwest, re-declaring the request and response types inline. Both sides are
> now fixed — the auth here, and the caller there.
>
> This is generated code and there is no OpenAPI spec in this repo, so a future
> regeneration will drop the fix. The durable answer is a `security` scheme on
> the spec, which makes the generator emit it.

## Available Models

- `big-pickle` (free)
- `minimax-m2.5-free` (free)
- `nemotron-3-super-free` (free)
- `glm-4.7-free` (free)
- `kimi-k2.5-free` (free)
- Plus paid models (gpt-5, claude, gemini, etc.)