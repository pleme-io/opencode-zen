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
> The hand-patch is now backed by [`openapi.yaml`](openapi.yaml), whose
> `security` block is what makes a regeneration emit the auth application by
> itself — see below.

## The spec

[`openapi.yaml`](openapi.yaml) describes the one endpoint this crate calls.

It was **reconstructed on 2026-08-17 from the generated models in `src/`**, not
taken from the vendor — this repo shipped generated code with no spec beside it,
and that is what let the missing-auth bug exist unexamined and uncorrectable at
its source. Every field, type and optionality mirrors what the code already
declared; nothing was invented. If OpenCode publishes a spec of their own, it
should **replace** this one rather than be reconciled with it.

The `security: [{bearerAuth: []}]` block is the point of the file. Given it, a
regeneration emits `req_builder.bearer_auth(...)` on its own, so the bug is not
patched but regenerated away.

`tests/spec_matches_models.rs` holds the two together — 5 tests, both
directions:

| Asserts | Why |
|---|---|
| Model set == schema set | a new model with no schema, or the reverse |
| Field sets match, per type | a field added on one side only |
| `required` == non-`Option` | the half that rots quietly: both sides can list a field and still disagree on whether it may be omitted |
| The security scheme is declared and is `bearer` | lose it and the client silently stops authenticating again |
| `role` is a closed enum | as a free string an invalid role becomes a runtime 400 instead of a compile error |

Both failure modes were red-run: removing the security block fails exactly the
security test, and marking `max_tokens` required fails with
`CreateChatCompletionRequest.max_tokens: spec says required, model says Option<…>`.

**Not yet reproducible.** `openapi-generator-cli` is not pinned anywhere in this
repo or the fleet, so nobody can currently regenerate this crate and get the
same bytes. The spec is the prerequisite for that, not the whole of it; pinning
the generator is the remaining step.

## Available Models

- `big-pickle` (free)
- `minimax-m2.5-free` (free)
- `nemotron-3-super-free` (free)
- `glm-4.7-free` (free)
- `kimi-k2.5-free` (free)
- Plus paid models (gpt-5, claude, gemini, etc.)