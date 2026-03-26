use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Metadata extracted from analyzing a single file.
#[derive(Debug, Clone)]
pub struct FileMeta {
    pub relative_path: String,
    pub is_binary: bool,
    pub line_count: usize,
    pub language: String,
    pub size_kb: f64,
}

/// Analyze a file: binary detection, LOC counting, language mapping.
/// Gracefully handles errors — returns `None` for unreadable files.
pub fn analyze_file(
    path: &Path,
    relative_path: &str,
    size_bytes: u64,
    detect_language: bool,
    count_lines: bool,
) -> Option<FileMeta> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[TreeC] Warning: Cannot open '{}': {}", relative_path, e);
            return None;
        }
    };

    let mut reader = BufReader::new(file);

    // ── 1. Binary Detection (first 1024 bytes) ──
    // Uses two heuristics:
    //   a) Any null byte (0x00) → strong binary indicator
    //   b) >30% non-printable bytes (excluding common control chars) → binary
    // This avoids false positives on UTF-16 text and false negatives on
    // binary files whose first bytes happen to be ASCII.
    let mut header = [0u8; 1024];
    let bytes_read = reader.read(&mut header).unwrap_or(0);
    let is_binary = detect_binary(&header[..bytes_read]);

    // ── 2. Language Detection ──
    let language = if detect_language {
        detect_lang(path)
    } else {
        "text".to_string()
    };

    // ── 3. Fast LOC Counting (byte-count approach for \n) ──
    let line_count = if !is_binary && count_lines {
        count_lines_fast(path, bytes_read, &header)
    } else {
        0
    };

    Some(FileMeta {
        relative_path: relative_path.to_string(),
        is_binary,
        line_count,
        language,
        size_kb: size_bytes as f64 / 1024.0,
    })
}

/// Read file content as a String. Returns None if binary or unreadable.
pub fn read_file_content(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

// ─── Dependency Detection ────────────────────────────────────────

/// A detected project dependency from any supported manifest.
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub manifest: String,
}

/// Scan the project root for known dependency manifests and extract dependencies.
///
/// Supports:
/// - `Cargo.toml` (Rust)
/// - `package.json` (Node.js)
/// - `requirements.txt` (Python)
/// - `go.mod` (Go)
/// - `*.csproj` (C# / .NET)
pub fn detect_dependencies(root: &Path) -> Vec<Dependency> {
    let mut deps = Vec::new();

    // Cargo.toml — Rust
    let cargo = root.join("Cargo.toml");
    if cargo.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo) {
            deps.extend(parse_cargo_toml(&content));
        }
    }

    // package.json — Node.js
    let package_json = root.join("package.json");
    if package_json.exists() {
        if let Ok(content) = std::fs::read_to_string(&package_json) {
            deps.extend(parse_package_json(&content));
        }
    }

    // requirements.txt — Python
    let requirements = root.join("requirements.txt");
    if requirements.exists() {
        if let Ok(content) = std::fs::read_to_string(&requirements) {
            deps.extend(parse_requirements_txt(&content));
        }
    }

    // go.mod — Go
    let go_mod = root.join("go.mod");
    if go_mod.exists() {
        if let Ok(content) = std::fs::read_to_string(&go_mod) {
            deps.extend(parse_go_mod(&content));
        }
    }

    // *.csproj — C# / .NET (search in root dir)
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "csproj").unwrap_or(false) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    deps.extend(parse_csproj(&content));
                }
            }
        }
    }

    deps
}

/// Format detected dependencies as a Markdown document for perception/dependencies.md.
pub fn format_dependencies_md(deps: &[Dependency], project_name: &str) -> String {
    if deps.is_empty() {
        return format!(
            "# 📦 Dependencies — {}\n\n> No dependency manifests detected.\n",
            project_name
        );
    }

    // Group by manifest
    let mut by_manifest: HashMap<&str, Vec<&Dependency>> = HashMap::new();
    for dep in deps {
        by_manifest
            .entry(dep.manifest.as_str())
            .or_default()
            .push(dep);
    }

    let mut md = format!(
        "# 📦 Dependencies — {}\n\n> Auto-detected by TreeC v{}\n> Updated: {}\n\n",
        project_name,
        env!("CARGO_PKG_VERSION"),
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    );

    // Sort manifests for deterministic output
    let mut manifests: Vec<&str> = by_manifest.keys().copied().collect();
    manifests.sort();

    for manifest in manifests {
        let manifest_deps = &by_manifest[manifest];
        md.push_str(&format!(
            "## {} ({} dependencies)\n\n",
            manifest,
            manifest_deps.len()
        ));
        md.push_str("| Package | Version |\n|---|---|\n");
        for dep in manifest_deps.iter() {
            md.push_str(&format!("| `{}` | `{}` |\n", dep.name, dep.version));
        }
        md.push('\n');
    }

    md
}

// ─── Manifest Parsers ────────────────────────────────────────────

fn parse_cargo_toml(content: &str) -> Vec<Dependency> {
    let mut deps = Vec::new();
    let mut in_deps = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect [dependencies] / [dev-dependencies] / [build-dependencies] sections
        if trimmed == "[dependencies]"
            || trimmed == "[dev-dependencies]"
            || trimmed == "[build-dependencies]"
        {
            in_deps = true;
            continue;
        }

        // Stop at next section
        if trimmed.starts_with('[') && in_deps {
            in_deps = false;
        }

        if !in_deps || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Parse: name = "version" or name = { version = "x", ... }
        if let Some((name, rest)) = trimmed.split_once('=') {
            let name = name.trim().to_string();
            let rest = rest.trim();
            let version = extract_version_from_toml_value(rest);
            deps.push(Dependency {
                name,
                version,
                manifest: "Cargo.toml".to_string(),
            });
        }
    }

    deps
}

fn extract_version_from_toml_value(value: &str) -> String {
    // Simple string: "1.0"
    if value.starts_with('"') {
        return value.trim_matches('"').to_string();
    }
    // Inline table: { version = "1.0", ... }
    if value.contains("version") {
        if let Some(start) = value.find("version") {
            let after = &value[start + 7..];
            if let Some(eq) = after.find('=') {
                let v = after[eq + 1..].trim();
                let v = v.trim_start_matches('"');
                if let Some(end) = v.find('"') {
                    return v[..end].to_string();
                }
            }
        }
    }
    "*".to_string()
}

fn parse_package_json(content: &str) -> Vec<Dependency> {
    let mut deps = Vec::new();

    let Ok(val) = serde_json::from_str::<serde_json::Value>(content) else {
        return deps;
    };

    for section in &["dependencies", "devDependencies", "peerDependencies"] {
        if let Some(obj) = val[section].as_object() {
            for (name, version) in obj {
                deps.push(Dependency {
                    name: name.clone(),
                    version: version.as_str().unwrap_or("*").to_string(),
                    manifest: "package.json".to_string(),
                });
            }
        }
    }

    deps
}

fn parse_requirements_txt(content: &str) -> Vec<Dependency> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
                return None;
            }
            // pkg==1.0 / pkg>=1.0 / pkg~=1.0 / pkg
            let (name, version) = if let Some(idx) = line.find(['=', '>', '<', '~']) {
                let name = line[..idx]
                    .trim_end_matches(['=', '>', '<', '~'])
                    .to_string();
                let version = line[idx..]
                    .trim_start_matches(['=', '>', '<', '~'])
                    .to_string();
                (name, version)
            } else {
                (line.to_string(), "*".to_string())
            };
            Some(Dependency {
                name,
                version,
                manifest: "requirements.txt".to_string(),
            })
        })
        .collect()
}

fn parse_go_mod(content: &str) -> Vec<Dependency> {
    let mut deps = Vec::new();
    let mut in_require = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "require (" {
            in_require = true;
            continue;
        }
        if trimmed == ")" && in_require {
            in_require = false;
            continue;
        }
        // Single-line: require github.com/foo/bar v1.0.0
        if trimmed.starts_with("require ") && !trimmed.ends_with('(') {
            let parts: Vec<&str> = trimmed[8..].split_whitespace().collect();
            if parts.len() >= 2 {
                deps.push(Dependency {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    manifest: "go.mod".to_string(),
                });
            }
            continue;
        }

        if in_require && !trimmed.is_empty() {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 && !parts[0].starts_with("//") {
                deps.push(Dependency {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    manifest: "go.mod".to_string(),
                });
            }
        }
    }

    deps
}

fn parse_csproj(content: &str) -> Vec<Dependency> {
    let mut deps = Vec::new();

    // Simple line-by-line search for <PackageReference Include="..." Version="..." />
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("PackageReference") {
            continue;
        }

        let name = extract_xml_attr(trimmed, "Include").unwrap_or_default();
        let version = extract_xml_attr(trimmed, "Version").unwrap_or_else(|| "*".to_string());

        if !name.is_empty() {
            deps.push(Dependency {
                name,
                version,
                manifest: "*.csproj".to_string(),
            });
        }
    }

    deps
}

fn extract_xml_attr(line: &str, attr: &str) -> Option<String> {
    let needle = format!("{}=\"", attr);
    let start = line.find(&needle)?;
    let after = &line[start + needle.len()..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

// ─── Binary Detection ────────────────────────────────────────────

/// Determine if a byte slice represents binary content.
///
/// A buffer is considered binary if:
/// - It contains any null byte (0x00), OR
/// - More than 30% of bytes are non-printable (not tab/LF/CR and not 0x20–0x7E)
///
/// This handles UTF-16 files (which have null bytes) correctly, and catches
/// binary files that start with printable bytes but contain non-text data.
fn detect_binary(buf: &[u8]) -> bool {
    if buf.is_empty() {
        return false;
    }

    let mut null_count: usize = 0;
    let mut non_printable: usize = 0;

    for &b in buf {
        if b == 0x00 {
            null_count += 1;
        }
        // Printable ASCII: 0x20–0x7E, plus tab (0x09), LF (0x0A), CR (0x0D)
        if b != 0x09 && b != 0x0A && b != 0x0D && !(0x20..=0x7E).contains(&b) {
            non_printable += 1;
        }
    }

    // Any null byte → binary
    if null_count > 0 {
        return true;
    }

    // >30% non-printable → binary
    non_printable * 100 / buf.len() > 30
}

// ─── Fast Line Counting ─────────────────────────────────────────

/// Count lines by scanning for 0x0A bytes — avoids full UTF-8 validation.
fn count_lines_fast(path: &Path, header_bytes: usize, header: &[u8; 1024]) -> usize {
    // Count \n in header first
    let mut count = header[..header_bytes]
        .iter()
        .filter(|&&b| b == 0x0A)
        .count();

    // If file is larger than header, read the rest
    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);

        // Skip past the header we already counted
        let mut skip = vec![0u8; header_bytes];
        let _ = reader.read_exact(&mut skip);

        // Read in chunks
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    count += buf[..n].iter().filter(|&&b| b == 0x0A).count();
                }
                Err(_) => break,
            }
        }
    }

    // At least 1 line if file is non-empty
    if count == 0 && header_bytes > 0 {
        1
    } else {
        count
    }
}

// ─── Language Detection ──────────────────────────────────────────

/// Map file extension to language identifier for Markdown code blocks.
fn detect_lang(path: &Path) -> String {
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let map: HashMap<&str, &str> = HashMap::from([
        ("rs", "rust"),
        ("py", "python"),
        ("js", "javascript"),
        ("ts", "typescript"),
        ("jsx", "jsx"),
        ("tsx", "tsx"),
        ("go", "go"),
        ("cs", "csharp"),
        ("java", "java"),
        ("cpp", "cpp"),
        ("c", "c"),
        ("h", "c"),
        ("hpp", "cpp"),
        ("sh", "bash"),
        ("bat", "batch"),
        ("cmd", "batch"),
        ("ps1", "powershell"),
        ("toml", "toml"),
        ("yml", "yaml"),
        ("yaml", "yaml"),
        ("json", "json"),
        ("xml", "xml"),
        ("html", "html"),
        ("htm", "html"),
        ("css", "css"),
        ("scss", "scss"),
        ("sql", "sql"),
        ("md", "markdown"),
        ("txt", "text"),
        ("rb", "ruby"),
        ("php", "php"),
        ("swift", "swift"),
        ("kt", "kotlin"),
        ("r", "r"),
        ("lua", "lua"),
        ("dart", "dart"),
        ("vue", "vue"),
        ("svelte", "svelte"),
        ("dockerfile", "dockerfile"),
        ("makefile", "makefile"),
    ]);

    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    // Handle special filenames without extensions
    if file_name == "dockerfile" {
        return "dockerfile".to_string();
    }
    if file_name == "makefile" {
        return "makefile".to_string();
    }

    map.get(ext.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "text".to_string())
}
