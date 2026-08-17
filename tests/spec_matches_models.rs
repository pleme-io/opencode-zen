//! The spec and the generated models must describe the same API.
//!
//! `openapi.yaml` was reconstructed from `src/models/` because this repo shipped
//! generated code with no spec, and that absence let the client go a long way
//! without applying authentication at all. Adding the spec fixes that — but it
//! introduces a new way to be wrong: a spec that no longer matches the code it
//! claims to describe is worse than no spec, because it is trusted.
//!
//! So the two are compared here, in both directions. A field added to a model
//! without a matching spec property fails; a spec property with no model field
//! fails; and `required` on the spec side must agree with non-`Option` on the
//! Rust side, which is where an optionality mistake would otherwise hide.
//!
//! When the vendor publishes a real spec and this one is replaced, this test
//! becomes the thing that tells you what the swap changed.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The Rust side of one model: serde field name -> is it Option<…>.
fn model_fields(src: &str) -> BTreeMap<String, bool> {
    let mut out = BTreeMap::new();
    let mut pending_rename: Option<String> = None;

    for line in src.lines() {
        let t = line.trim();

        // `#[serde(rename = "x")]` — the wire name, which is what the spec
        // describes. The Rust identifier may differ and is not what matters.
        if let Some(i) = t.find("rename = \"") {
            let rest = &t[i + "rename = \"".len()..];
            if let Some(j) = rest.find('"') {
                pending_rename = Some(rest[..j].to_string());
            }
            continue;
        }

        if let Some(decl) = t.strip_prefix("pub ") {
            let Some((ident, ty)) = decl.split_once(':') else {
                continue;
            };
            // Skip enum variants and anything that is not a struct field.
            if !ty.trim_end().ends_with(',') {
                continue;
            }
            let name = pending_rename
                .take()
                .unwrap_or_else(|| ident.trim().to_string());
            let ty = ty.trim().trim_end_matches(',');
            out.insert(name, ty.starts_with("Option<"));
        }
    }
    out
}

/// `create_chat_completion_request` -> `CreateChatCompletionRequest`
fn pascal(stem: &str) -> String {
    stem.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn read_models() -> BTreeMap<String, BTreeMap<String, bool>> {
    let dir = repo_root().join("src/models");
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(&dir)
        .expect("src/models must exist")
        .flatten()
    {
        let path = entry.path();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if stem == "mod" || path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let src = std::fs::read_to_string(&path).expect("model file readable");
        out.insert(pascal(stem), model_fields(&src));
    }
    assert!(
        out.len() >= 5,
        "found only {} models — the reader is broken, and a comparison over \
         nothing passes trivially",
        out.len()
    );
    out
}

fn spec() -> serde_yaml::Value {
    let raw =
        std::fs::read_to_string(repo_root().join("openapi.yaml")).expect("openapi.yaml must exist");
    serde_yaml::from_str(&raw).expect("openapi.yaml must be valid YAML")
}

fn schemas(spec: &serde_yaml::Value) -> BTreeMap<String, serde_yaml::Value> {
    spec["components"]["schemas"]
        .as_mapping()
        .expect("components.schemas")
        .iter()
        .map(|(k, v)| (k.as_str().unwrap_or_default().to_string(), v.clone()))
        .collect()
}

fn keys_of(v: &serde_yaml::Value, field: &str) -> BTreeSet<String> {
    v[field]
        .as_mapping()
        .map(|m| {
            m.keys()
                .filter_map(|k| k.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn every_model_has_a_schema_and_every_schema_has_a_model() {
    let models = read_models();
    let spec = spec();
    let schemas = schemas(&spec);

    let m: BTreeSet<_> = models.keys().cloned().collect();
    let s: BTreeSet<_> = schemas.keys().cloned().collect();

    assert_eq!(
        m,
        s,
        "the spec and src/models/ describe different type sets.\n  \
         models only: {:?}\n  spec only: {:?}",
        m.difference(&s).collect::<Vec<_>>(),
        s.difference(&m).collect::<Vec<_>>()
    );
}

#[test]
fn fields_and_optionality_agree() {
    let models = read_models();
    let spec = spec();
    let schemas = schemas(&spec);
    let mut problems = Vec::new();

    for (name, fields) in &models {
        let Some(schema) = schemas.get(name) else {
            continue; // reported by the test above
        };
        let props = keys_of(schema, "properties");
        let required: BTreeSet<String> = schema["required"]
            .as_sequence()
            .map(|s| {
                s.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let declared: BTreeSet<String> = fields.keys().cloned().collect();
        for missing in declared.difference(&props) {
            problems.push(format!(
                "{name}.{missing}: in the model, absent from the spec"
            ));
        }
        for extra in props.difference(&declared) {
            problems.push(format!(
                "{name}.{extra}: in the spec, absent from the model"
            ));
        }

        // required <-> non-Option. This is the half that would otherwise rot
        // quietly: a field can exist on both sides and still disagree about
        // whether a caller may omit it.
        for (field, is_option) in fields {
            if !props.contains(field) {
                continue;
            }
            let is_required = required.contains(field);
            if is_required == *is_option {
                problems.push(format!(
                    "{name}.{field}: spec says {}, model says {}",
                    if is_required { "required" } else { "optional" },
                    if *is_option {
                        "Option<…>"
                    } else {
                        "non-Option"
                    },
                ));
            }
        }
    }

    assert!(
        problems.is_empty(),
        "spec/model drift:\n  {}",
        problems.join("\n  ")
    );
}

/// The reason this spec exists at all: the generator emits the auth block from
/// a declared security scheme. Lose the declaration and the client silently
/// goes back to sending unauthenticated requests — which is exactly the state
/// this repo was in before 2026-08-17.
#[test]
fn the_security_scheme_is_declared_and_is_bearer() {
    let spec = spec();

    let top = spec["security"]
        .as_sequence()
        .expect("a top-level `security` block must exist");
    assert!(
        top.iter().any(|e| e.get("bearerAuth").is_some()),
        "`bearerAuth` must be applied at the top level, or the generator emits \
         no auth for the operation"
    );

    let scheme = &spec["components"]["securitySchemes"]["bearerAuth"];
    assert_eq!(scheme["type"].as_str(), Some("http"), "scheme type");
    assert_eq!(
        scheme["scheme"].as_str(),
        Some("bearer"),
        "the service wants `Authorization: Bearer`, not an api-key header"
    );
}

/// `role` must stay an enum. As a free-form string an invalid role becomes a
/// runtime 400 instead of a compile error, which is a real property of the
/// current generated client that a careless spec edit would throw away.
#[test]
fn chat_message_role_is_a_closed_enum() {
    let spec = spec();
    let role = &spec["components"]["schemas"]["ChatMessage"]["properties"]["role"];
    let values: BTreeSet<String> = role["enum"]
        .as_sequence()
        .expect("role must declare an enum")
        .iter()
        .filter_map(|v| v.as_str().map(str::to_string))
        .collect();

    let expected: BTreeSet<String> = ["assistant", "system", "user"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        values, expected,
        "role variants must match the generated Role enum"
    );
}

#[test]
fn the_operation_is_the_one_the_client_calls() {
    let spec = spec();
    // src/apis/default_api.rs builds `{base_path}/chat/completions`; a spec that
    // named a different path would regenerate a client that calls elsewhere.
    let op = &spec["paths"]["/chat/completions"]["post"];
    assert_eq!(op["operationId"].as_str(), Some("createChatCompletion"));

    let server = spec["servers"][0]["url"].as_str().unwrap_or_default();
    assert_eq!(
        server, "https://opencode.ai/zen/v1",
        "the server url must match Configuration::default()'s base_path"
    );
    let cfg = std::fs::read_to_string(repo_root().join("src/apis/configuration.rs"))
        .expect("configuration.rs readable");
    assert!(
        cfg.contains(server),
        "configuration.rs no longer defaults to the spec's server url"
    );
}

#[allow(dead_code)]
fn _unused(_: &Path) {}
