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

        cfg
    }
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
