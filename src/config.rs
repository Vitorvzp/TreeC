use serde::Deserialize;
use std::fs;

/// Configuration loaded from TreeC.toml via the `toml` crate.
#[derive(Debug)]
pub struct Config {
    // [General]
    pub max_file_size_kb: u64,
    pub use_gitignore: bool,
    pub detect_language: bool,
    pub count_lines: bool,
    pub include_hidden_dirs: bool,

    // [Exports]
    pub generate_json: bool,
    pub generate_txt: bool,
    pub generate_markdown: bool,

    // [Ignore]
    pub ignore_folders: Vec<String>,
    pub ignore_extensions: Vec<String>,
    pub ignore_files: Vec<String>,

    // [NeuralLink]
    /// API key — resolved from env var TREEC_API_KEY first, then TreeC.toml.
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
            include_hidden_dirs: false,
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

// ─── TOML Deserialization Structs ────────────────────────────────

#[derive(Deserialize, Default)]
struct TomlConfig {
    #[serde(rename = "General")]
    general: Option<TomlGeneral>,
    #[serde(rename = "Exports")]
    exports: Option<TomlExports>,
    #[serde(rename = "Ignore")]
    ignore: Option<TomlIgnore>,
    #[serde(rename = "NeuralLink")]
    neural_link: Option<TomlNeuralLink>,
}

#[derive(Deserialize, Default)]
struct TomlGeneral {
    #[serde(rename = "MaxFileSizeKB")]
    max_file_size_kb: Option<u64>,
    #[serde(rename = "UseGitIgnore")]
    use_gitignore: Option<bool>,
    #[serde(rename = "DetectLanguage")]
    detect_language: Option<bool>,
    #[serde(rename = "CountLines")]
    count_lines: Option<bool>,
    #[serde(rename = "IncludeHiddenDirs")]
    include_hidden_dirs: Option<bool>,
}

#[derive(Deserialize, Default)]
struct TomlExports {
    #[serde(rename = "GenerateMarkdown")]
    generate_markdown: Option<bool>,
    #[serde(rename = "GenerateJson")]
    generate_json: Option<bool>,
    #[serde(rename = "GenerateTxt")]
    generate_txt: Option<bool>,
}

#[derive(Deserialize, Default)]
struct TomlIgnore {
    #[serde(rename = "Folders")]
    folders: Option<Vec<String>>,
    #[serde(rename = "Extensions")]
    extensions: Option<Vec<String>>,
    #[serde(rename = "Files")]
    files: Option<Vec<String>>,
}

#[derive(Deserialize, Default)]
struct TomlNeuralLink {
    #[serde(rename = "ApiKey")]
    api_key: Option<String>,
    #[serde(rename = "Model")]
    model: Option<String>,
    #[serde(rename = "Provider")]
    provider: Option<String>,
}

// ─── Config Implementation ───────────────────────────────────────

impl Config {
    /// Load configuration from TreeC.toml using the `toml` crate.
    /// Falls back to defaults if the file is missing or unparseable.
    ///
    /// API key resolution order:
    ///   1. `TREEC_API_KEY` environment variable (most secure)
    ///   2. `ApiKey` field in `[NeuralLink]` section of TreeC.toml
    pub fn load(root: &std::path::Path) -> Self {
        let toml_path = root.join("TreeC.toml");
        let content = match fs::read_to_string(&toml_path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("[TreeC] Warning: TreeC.toml not found, using defaults.");
                return Self::resolve_api_key(Self::default());
            }
        };

        let parsed: TomlConfig = match toml::from_str(&content) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[TreeC] Warning: Failed to parse TreeC.toml: {}. Using defaults.", e);
                return Self::resolve_api_key(Self::default());
            }
        };

        let defaults = Self::default();
        let general = parsed.general.unwrap_or_default();
        let exports = parsed.exports.unwrap_or_default();
        let ignore = parsed.ignore.unwrap_or_default();
        let neural = parsed.neural_link.unwrap_or_default();

        let cfg = Self {
            max_file_size_kb: general.max_file_size_kb.unwrap_or(defaults.max_file_size_kb),
            use_gitignore: general.use_gitignore.unwrap_or(defaults.use_gitignore),
            detect_language: general.detect_language.unwrap_or(defaults.detect_language),
            count_lines: general.count_lines.unwrap_or(defaults.count_lines),
            include_hidden_dirs: general.include_hidden_dirs.unwrap_or(defaults.include_hidden_dirs),

            generate_markdown: exports.generate_markdown.unwrap_or(defaults.generate_markdown),
            generate_json: exports.generate_json.unwrap_or(defaults.generate_json),
            generate_txt: exports.generate_txt.unwrap_or(defaults.generate_txt),

            ignore_folders: ignore.folders.unwrap_or(defaults.ignore_folders),
            ignore_extensions: ignore.extensions.unwrap_or(defaults.ignore_extensions),
            ignore_files: ignore.files.unwrap_or(defaults.ignore_files),

            neural_api_key: neural.api_key.filter(|k| !k.is_empty()),
            neural_model: neural.model.unwrap_or(defaults.neural_model),
            neural_provider: neural.provider.unwrap_or(defaults.neural_provider),
        };

        Self::resolve_api_key(cfg)
    }

    /// Resolve API key: env var TREEC_API_KEY overrides the value in TreeC.toml.
    fn resolve_api_key(mut cfg: Self) -> Self {
        if let Ok(env_key) = std::env::var("TREEC_API_KEY") {
            if !env_key.is_empty() {
                cfg.neural_api_key = Some(env_key);
            }
        }
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

        // Remove existing [NeuralLink] section
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
