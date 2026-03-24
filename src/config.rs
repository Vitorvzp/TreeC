use regex::Regex;
use std::fs;

/// Configuration loaded from TreeC.toml via Regex-based parsing.
#[derive(Debug)]
pub struct Config {
    // [General]
    pub max_file_size_kb: u64,
    pub use_gitignore: bool,
    pub detect_language: bool,
    pub count_lines: bool,

    // [Exports]
    pub generate_json: bool,
    pub generate_txt: bool,
    pub generate_markdown: bool,

    // [Ignore]
    pub ignore_folders: Vec<String>,
    pub ignore_extensions: Vec<String>,
    pub ignore_files: Vec<String>,

    // [NeuralLink]
    pub neural_api_key: Option<String>,
    pub neural_model: String,
    pub neural_provider: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_file_size_kb: 1024,
            use_gitignore: true,
            detect_language: true,
            count_lines: true,
            generate_json: true,
            generate_txt: true,
            generate_markdown: true,
            ignore_folders: vec!["target".into(), "node_modules".into(), ".git".into()],
            ignore_extensions: vec![],
            ignore_files: vec![],
            neural_api_key: None,
            neural_model: "gemini-2.0-flash".into(),
            neural_provider: "gemini".into(),
        }
    }
}

impl Config {
    /// Load configuration from TreeC.toml using Regex-based parsing.
    /// Falls back to defaults if the file is missing or unparseable.
    pub fn load(root: &std::path::Path) -> Self {
        let toml_path = root.join("TreeC.toml");
        let content = match fs::read_to_string(&toml_path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("[TreeC] Warning: TreeC.toml not found, using defaults.");
                return Self::default();
            }
        };

        let mut cfg = Self::default();

        // --- [General] ---
        cfg.max_file_size_kb = parse_int(&content, "MaxFileSizeKB").unwrap_or(cfg.max_file_size_kb);
        cfg.use_gitignore = parse_bool(&content, "UseGitIgnore").unwrap_or(cfg.use_gitignore);
        cfg.detect_language = parse_bool(&content, "DetectLanguage").unwrap_or(cfg.detect_language);
        cfg.count_lines = parse_bool(&content, "CountLines").unwrap_or(cfg.count_lines);

        // --- [Exports] ---
        cfg.generate_json = parse_bool(&content, "GenerateJson").unwrap_or(cfg.generate_json);
        cfg.generate_txt = parse_bool(&content, "GenerateTxt").unwrap_or(cfg.generate_txt);
        cfg.generate_markdown =
            parse_bool(&content, "GenerateMarkdown").unwrap_or(cfg.generate_markdown);

        // --- [Ignore] ---
        cfg.ignore_folders = parse_string_array(&content, "Folders").unwrap_or(cfg.ignore_folders);
        cfg.ignore_extensions =
            parse_string_array(&content, "Extensions").unwrap_or(cfg.ignore_extensions);
        cfg.ignore_files = parse_string_array(&content, "Files").unwrap_or(cfg.ignore_files);

        // --- [NeuralLink] ---
        cfg.neural_api_key = parse_string(&content, "ApiKey");
        cfg.neural_model = parse_string(&content, "Model").unwrap_or(cfg.neural_model);
        cfg.neural_provider = parse_string(&content, "Provider").unwrap_or(cfg.neural_provider);

        cfg
    }

    /// Save full neural config (provider, model, API key) to TreeC.toml [NeuralLink].
    pub fn save_neural_config(
        root: &std::path::Path,
        api_key: &str,
        provider: &str,
        model: &str,
    ) -> Result<(), String> {
        let toml_path = root.join("TreeC.toml");
        let mut content = fs::read_to_string(&toml_path).unwrap_or_default();

        // Remove existing [NeuralLink] section (line-by-line, no lookahead)
        content = strip_neural_section(&content);

        // Append clean [NeuralLink] section
        content.push_str(&format!(
            "\n[NeuralLink]\nProvider = \"{}\"\nModel = \"{}\"\nApiKey = \"{}\"\n",
            provider, model, api_key
        ));

        fs::write(&toml_path, &content)
            .map_err(|e| format!("Failed to write TreeC.toml: {}", e))
    }

    /// Remove the [NeuralLink] section from TreeC.toml.
    pub fn remove_neural_config(root: &std::path::Path) -> Result<(), String> {
        let toml_path = root.join("TreeC.toml");
        let content = fs::read_to_string(&toml_path)
            .map_err(|_| "TreeC.toml not found.".to_string())?;

        let cleaned = strip_neural_section(&content);

        fs::write(&toml_path, &cleaned)
            .map_err(|e| format!("Failed to write TreeC.toml: {}", e))
    }
}

/// Remove the [NeuralLink] section from TOML content using line-by-line parsing.
fn strip_neural_section(content: &str) -> String {
    let mut result = String::new();
    let mut in_neural = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[NeuralLink]" {
            in_neural = true;
            continue;
        }
        // If we hit another section header, stop skipping
        if in_neural && trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_neural = false;
        }
        if !in_neural {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Clean trailing blank lines
    while result.ends_with("\n\n") {
        result.pop();
    }

    result
}

// ─── Regex Helpers ───────────────────────────────────────────────

/// Parse an integer value: `Key = 1024`
fn parse_int(content: &str, key: &str) -> Option<u64> {
    let pattern = format!(r"(?m)^\s*{}\s*=\s*(\d+)", regex::escape(key));
    let re = Regex::new(&pattern).ok()?;
    re.captures(content)
        .and_then(|cap| cap.get(1)?.as_str().parse().ok())
}

/// Parse a boolean value: `Key = true` / `Key = false`
fn parse_bool(content: &str, key: &str) -> Option<bool> {
    let pattern = format!(r"(?m)^\s*{}\s*=\s*(true|false)", regex::escape(key));
    let re = Regex::new(&pattern).ok()?;
    re.captures(content)
        .and_then(|cap| Some(cap.get(1)?.as_str() == "true"))
}

/// Parse a single quoted string value: `Key = "value"`
fn parse_string(content: &str, key: &str) -> Option<String> {
    let pattern = format!(r#"(?m)^\s*{}\s*=\s*"([^"]*)""#, regex::escape(key));
    let re = Regex::new(&pattern).ok()?;
    re.captures(content)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

/// Parse a TOML string array: `Key = ["val1", "val2"]`
fn parse_string_array(content: &str, key: &str) -> Option<Vec<String>> {
    let pattern = format!(r"(?m)^\s*{}\s*=\s*\[([^\]]*)\]", regex::escape(key));
    let re = Regex::new(&pattern).ok()?;
    let caps = re.captures(content)?;
    let inner = caps.get(1)?.as_str();

    let item_re = Regex::new(r#""([^"]+)""#).ok()?;
    let items: Vec<String> = item_re
        .captures_iter(inner)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect();

    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}
