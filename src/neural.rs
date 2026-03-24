use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::brain;

// ═══════════════════════════════════════════════════════════════════
// Gemini API Types
// ═══════════════════════════════════════════════════════════════════

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GeminiGenerationConfig {
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: Option<GeminiResponseContent>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponseContent {
    parts: Option<Vec<GeminiResponsePart>>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponsePart {
    text: Option<String>,
}

/// JSON schema we expect from the AI
#[derive(Deserialize, Debug)]
pub struct BrainOutput {
    pub context: Option<String>,
    pub architecture: Option<String>,
    pub readme: Option<String>,
    pub roadmap: Option<String>,
    pub decisions: Option<String>,
    pub tasks: Option<String>,
    pub modules: Option<String>,
    pub functions: Option<String>,
    pub api: Option<String>,
    pub database: Option<String>,
    pub models: Option<String>,
    pub services: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════
// Neural Link Execution
// ═══════════════════════════════════════════════════════════════════

/// Execute the Neural Link: send project data to AI and populate .brain/
pub fn execute_neural_link(
    root: &std::path::Path,
    tree_md_content: &str,
    api_key: &str,
    model: &str,
    provider: &str,
) -> Result<(), String> {
    println!("   🧠 Initializing .brain/ structure...");
    brain::init_brain(root)?;

    // Update tree.md with current scan
    brain::update_tree(root, tree_md_content)?;

    println!("   🔗 Sending project to AI ({} / {})...", provider, model);
    println!("   ⏳ This may take a moment...");

    let system_prompt = build_system_prompt();
    let user_content = format!(
        "Here is the complete project structure and code. Analyze it and generate the brain files.\n\n{}",
        tree_md_content
    );

    let brain_output = match provider {
        "gemini" | "google" => call_gemini_api(&system_prompt, &user_content, api_key, model)?,
        "openai" | "gpt" => call_openai_api(&system_prompt, &user_content, api_key, model)?,
        "claude" | "anthropic" => call_claude_api(&system_prompt, &user_content, api_key, model)?,
        _ => return Err(format!("Unknown provider: {}", provider)),
    };

    println!("   📝 Writing brain files...");
    write_brain_output(root, &brain_output)?;

    println!("   ✅ Neural Link complete! .brain/ is ready.");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════
// Gemini API
// ═══════════════════════════════════════════════════════════════════

fn call_gemini_api(system_prompt: &str, user_content: &str, api_key: &str, model: &str) -> Result<BrainOutput, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let request_body = json!({
        "contents": [{
            "parts": [{ "text": format!("{}\n\n{}", system_prompt, user_content) }]
        }],
        "generationConfig": {
            "responseMimeType": "application/json"
        }
    });

    // Retry with backoff for rate limits
    let max_retries = 3;
    let backoff_secs = [5, 15, 30];

    for attempt in 0..=max_retries {
        let result = ureq::post(&url)
            .set("Content-Type", "application/json")
            .send_json(&request_body);

        match result {
            Ok(response) => {
                let response_text = response.into_string()
                    .map_err(|e| format!("Failed to read Gemini response: {}", e))?;

                let gemini_resp: GeminiResponse = serde_json::from_str(&response_text)
                    .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

                let text = gemini_resp.candidates
                    .and_then(|c| c.into_iter().next())
                    .and_then(|c| c.content)
                    .and_then(|c| c.parts)
                    .and_then(|p| p.into_iter().next())
                    .and_then(|p| p.text)
                    .ok_or_else(|| "Gemini returned empty response".to_string())?;

                return serde_json::from_str(&text)
                    .map_err(|e| format!("Failed to parse brain JSON: {}.\nRaw: {}", e, &text[..text.len().min(500)]));
            }
            Err(ureq::Error::Status(status, _response)) => {
                match status {
                    429 => {
                        if attempt < max_retries {
                            let wait = backoff_secs[attempt as usize];
                            println!("   ⚠️  Rate limited (429). Retrying in {}s... (attempt {}/{})", wait, attempt + 1, max_retries);
                            std::thread::sleep(std::time::Duration::from_secs(wait));
                            continue;
                        }
                        return Err("Gemini API rate limit exceeded (429). Try again in a few minutes.\n   💡 Tip: The free Gemini API has a limit of 15 requests/minute.".to_string());
                    }
                    400 => return Err("Gemini API: Bad request (400). Check your API key and model name.".to_string()),
                    401 | 403 => return Err("Gemini API: Invalid or unauthorized API key (401/403).\n   Run: treec --config-neural gemini <VALID_KEY>".to_string()),
                    404 => return Err(format!("Gemini API: Model '{}' not found (404). Check the model name.", model)),
                    500..=599 => return Err(format!("Gemini API: Server error ({}). Try again later.", status)),
                    _ => return Err(format!("Gemini API: HTTP error {}", status)),
                }
            }
            Err(e) => {
                // Mask API key in transport errors
                let err_msg = format!("{}", e);
                let safe_msg = err_msg.replace(api_key, "***");
                return Err(format!("Gemini API connection failed: {}", safe_msg));
            }
        }
    }

    Err("Gemini API: Max retries exceeded".to_string())
}

// ═══════════════════════════════════════════════════════════════════
// OpenAI API
// ═══════════════════════════════════════════════════════════════════

fn call_openai_api(system_prompt: &str, user_content: &str, api_key: &str, model: &str) -> Result<BrainOutput, String> {
    let url = "https://api.openai.com/v1/chat/completions";

    let request_body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_content }
        ],
        "response_format": { "type": "json_object" },
        "temperature": 0.2
    });

    let result = ureq::post(url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(&request_body);

    let response = match result {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            let msg = extract_api_error(&body).unwrap_or(body);
            return Err(format!("OpenAI API error ({}): {}", status, msg));
        }
        Err(e) => return Err(format!("OpenAI API connection failed: {}", e)),
    };

    let response_text = response.into_string()
        .map_err(|e| format!("Failed to read OpenAI response: {}", e))?;

    let resp: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    let text = resp["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "OpenAI returned empty response".to_string())?;

    serde_json::from_str(text)
        .map_err(|e| format!("Failed to parse brain JSON: {}.\nRaw: {}", e, &text[..text.len().min(500)]))
}

// ═══════════════════════════════════════════════════════════════════
// Claude / Anthropic API
// ═══════════════════════════════════════════════════════════════════

fn call_claude_api(system_prompt: &str, user_content: &str, api_key: &str, model: &str) -> Result<BrainOutput, String> {
    let url = "https://api.anthropic.com/v1/messages";

    let request_body = json!({
        "model": model,
        "max_tokens": 16384,
        "system": format!("{}\n\nIMPORTANT: Return ONLY a valid JSON object, no markdown fences.", system_prompt),
        "messages": [
            { "role": "user", "content": user_content }
        ]
    });

    let result = ureq::post(url)
        .set("Content-Type", "application/json")
        .set("x-api-key", api_key)
        .set("anthropic-version", "2023-06-01")
        .send_json(&request_body);

    let response = match result {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            let msg = extract_api_error(&body).unwrap_or(body);
            return Err(format!("Claude API error ({}): {}", status, msg));
        }
        Err(e) => return Err(format!("Claude API connection failed: {}", e)),
    };

    let response_text = response.into_string()
        .map_err(|e| format!("Failed to read Claude response: {}", e))?;

    // Claude response: { content: [{ type: "text", text: "..." }] }
    let resp: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse Claude response: {}", e))?;

    let text = resp["content"][0]["text"]
        .as_str()
        .ok_or_else(|| "Claude returned empty response".to_string())?;

    // Claude may wrap in ```json ... ``` — strip if present
    let clean = text.trim();
    let clean = if clean.starts_with("```") {
        let inner = clean.trim_start_matches("```json").trim_start_matches("```");
        inner.trim_end_matches("```").trim()
    } else {
        clean
    };

    serde_json::from_str(clean)
        .map_err(|e| format!("Failed to parse brain JSON: {}.\nRaw: {}", e, &clean[..clean.len().min(500)]))
}

/// Write all AI-generated brain files.
fn write_brain_output(root: &std::path::Path, output: &BrainOutput) -> Result<(), String> {
    let files: Vec<(&str, &Option<String>)> = vec![
        ("context.md", &output.context),
        ("architecture.md", &output.architecture),
        ("readme.md", &output.readme),
        ("roadmap.md", &output.roadmap),
        ("decisions.md", &output.decisions),
        ("tasks.md", &output.tasks),
        ("knowledge/modules.md", &output.modules),
        ("knowledge/functions.md", &output.functions),
        ("knowledge/api.md", &output.api),
        ("knowledge/database.md", &output.database),
        ("knowledge/models.md", &output.models),
        ("knowledge/services.md", &output.services),
    ];

    let mut written = 0;
    for (filename, content) in files {
        if let Some(text) = content {
            if !text.is_empty() {
                brain::update_brain_file(root, filename, text)?;
                written += 1;
            }
        }
    }

    println!("   📄 {} brain files populated by AI", written);
    Ok(())
}

/// Build the system prompt that instructs the AI what to generate.
fn build_system_prompt() -> String {
    r##"You are TreeC Neural Link — an AI that analyzes project structures and generates structured documentation.

Given a project's Tree.md (containing file tree, metadata, and source code), you must generate a JSON object with the following keys. Each value should be a complete Markdown document.

Return ONLY a valid JSON object with these keys:

{
  "context": "# Context\n\n(Full project overview: purpose, tech stack, architecture summary, main modules, how the system works, important files, entry points, risks, TODOs, improvement ideas)",
  "architecture": "# Architecture\n\n(System architecture: modules, data flow, APIs, database, services, dependencies, design patterns. Include Mermaid diagrams where helpful)",
  "readme": "# Project Name\n\n(Professional README with: description, features, installation, usage, configuration, contributing guidelines)",
  "roadmap": "# Roadmap\n\n(Suggested features and improvements with Priority/Difficulty/Impact for each)",
  "decisions": "# Technical Decisions\n\n(Document key technical decisions found in the codebase: what was chosen, why, alternatives, impact)",
  "tasks": "# Tasks\n\n(Identify pending tasks, improvements, and technical debt from the codebase)",
  "modules": "# Modules\n\n(Document each module/file: purpose, public API, dependencies, key functions)",
  "functions": "# Functions\n\n(Document key functions: signature, purpose, parameters, return values)",
  "api": "# API\n\n(Document any API endpoints, routes, request/response formats found in the code)",
  "database": "# Database\n\n(Document database schema, models, migrations, queries found in the code)",
  "models": "# Models\n\n(Document data models, structs, types, interfaces found in the code)",
  "services": "# Services\n\n(Document services, external integrations, background jobs found in the code)"
}

Rules:
- Write in the same language as the project's README or comments (if Portuguese, write in Portuguese)
- Be thorough and detailed
- Use Obsidian-compatible Markdown with [[wikilinks]] between brain files
- If a section has no relevant content (e.g., no database), put "No relevant content found for this project."
- Use code examples from the actual source code when documenting
- Return ONLY the JSON object, no extra text"##.to_string()
}

// ═══════════════════════════════════════════════════════════════════
// Error Helpers
// ═══════════════════════════════════════════════════════════════════

/// Extract a human-readable error message from an API JSON error response.
/// Works with Gemini, OpenAI, and Claude error formats.
fn extract_api_error(body: &str) -> Option<String> {
    let val: serde_json::Value = serde_json::from_str(body).ok()?;

    // Claude: { "error": { "type": "...", "message": "..." } }
    if let Some(msg) = val["error"]["message"].as_str() {
        return Some(msg.to_string());
    }
    // OpenAI: same format
    // Gemini: { "error": { "message": "...", "status": "..." } }
    if let Some(msg) = val["message"].as_str() {
        return Some(msg.to_string());
    }

    None
}
