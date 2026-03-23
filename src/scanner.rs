use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;

/// Hardcoded automatic exclusions (must never be counted or exported).
const AUTO_EXCLUDED_FILES: &[&str] = &[
    "Tree.md",
    "Structure.json",
    "Structure.txt",
    "TreeC.toml",
    "treec",
    "treec.exe",
    ".git",
    ".gitignore",
];

/// A discovered filesystem entry (file or directory).
#[derive(Debug, Clone)]
pub struct ScanEntry {
    pub path: PathBuf,
    pub relative_path: String,
    pub is_dir: bool,
    pub size_bytes: u64,
}

/// Result of scanning a project directory.
#[derive(Debug)]
pub struct ScanResult {
    pub files: Vec<ScanEntry>,
    pub dirs: Vec<ScanEntry>,
}

/// Scan the project rooted at `root` according to the given `Config`.
pub fn scan_project(root: &Path, config: &Config) -> ScanResult {
    let gitignore_patterns = if config.use_gitignore {
        load_gitignore(root)
    } else {
        Vec::new()
    };

    // Build lookup sets for fast matching
    let ignored_folders: HashSet<&str> = config.ignore_folders.iter().map(|s| s.as_str()).collect();
    let ignored_extensions: HashSet<String> = config
        .ignore_extensions
        .iter()
        .map(|s| {
            if s.starts_with('.') {
                s.to_lowercase()
            } else {
                format!(".{}", s.to_lowercase())
            }
        })
        .collect();
    let ignored_files: HashSet<&str> = config.ignore_files.iter().map(|s| s.as_str()).collect();
    let auto_excluded: HashSet<&str> = AUTO_EXCLUDED_FILES.iter().copied().collect();

    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            let name_str = name.as_ref();

            // Skip auto-excluded
            if auto_excluded.contains(name_str) {
                return false;
            }

            // Skip hidden directories (starting with .)
            if e.file_type().is_dir() && name_str.starts_with('.') {
                return false;
            }

            // Skip config-ignored folders
            if e.file_type().is_dir() && ignored_folders.contains(name_str) {
                return false;
            }

            true
        })
        .filter_map(|e| e.ok())
    {
        // Skip the root directory itself
        if entry.path() == root {
            continue;
        }

        let relative = entry
            .path()
            .strip_prefix(root)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .replace('\\', "/");

        let name = entry.file_name().to_string_lossy().to_string();

        if entry.file_type().is_dir() {
            dirs.push(ScanEntry {
                path: entry.path().to_path_buf(),
                relative_path: relative,
                is_dir: true,
                size_bytes: 0,
            });
            continue;
        }

        // --- File-level filters ---

        // Auto-excluded files
        if auto_excluded.contains(name.as_str()) {
            continue;
        }

        // Config-ignored files
        if ignored_files.contains(name.as_str()) {
            continue;
        }

        // Extension filter
        if let Some(ext) = entry.path().extension() {
            let dot_ext = format!(".{}", ext.to_string_lossy().to_lowercase());
            if ignored_extensions.contains(&dot_ext) {
                continue;
            }
        }

        // Gitignore filter
        if is_gitignored(&relative, &gitignore_patterns) {
            continue;
        }

        // Size filter
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        if config.max_file_size_kb > 0 && size > config.max_file_size_kb * 1024 {
            continue;
        }

        files.push(ScanEntry {
            path: entry.path().to_path_buf(),
            relative_path: relative,
            is_dir: false,
            size_bytes: size,
        });
    }

    // Sort: alphabetical
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    dirs.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    ScanResult { files, dirs }
}

// ─── Gitignore Support ───────────────────────────────────────────

/// Load .gitignore patterns and compile them to regex.
fn load_gitignore(root: &Path) -> Vec<Regex> {
    let gitignore_path = root.join(".gitignore");
    let content = match fs::read_to_string(&gitignore_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|pattern| gitignore_to_regex(pattern))
        .collect()
}

/// Convert a single gitignore glob pattern to a compiled Regex.
fn gitignore_to_regex(pattern: &str) -> Option<Regex> {
    let mut pat = pattern.to_string();

    // Remove trailing slash (directory marker — we handle dirs elsewhere)
    if pat.ends_with('/') {
        pat.pop();
    }

    // Remove leading slash (root-relative)
    if pat.starts_with('/') {
        pat.remove(0);
    }

    // Escape regex specials, then convert globs
    let escaped = regex::escape(&pat);
    let converted = escaped
        .replace(r"\*\*", ".*")      // ** → match everything
        .replace(r"\*", "[^/]*")     // * → match within path segment
        .replace(r"\?", "[^/]");     // ? → single char

    let regex_str = format!("(^|/){}", converted);
    Regex::new(&regex_str).ok()
}

/// Check if a relative path matches any gitignore pattern.
fn is_gitignored(relative_path: &str, patterns: &[Regex]) -> bool {
    patterns.iter().any(|re| re.is_match(relative_path))
}
