use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::config::Config;

/// Hardcoded automatic exclusions (must never be counted or exported).
const AUTO_EXCLUDED: &[&str] = &[
    "Tree.md",
    "Structure.json",
    "Structure.txt",
    "TreeC.toml",
    "treec",
    "treec.exe",
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
///
/// Uses the `ignore` crate for directory walking, which provides:
/// - Full `.gitignore` semantics (including negation patterns with `!`)
/// - `.git/info/exclude` support
/// - Configurable hidden directory handling
pub fn scan_project(root: &Path, config: &Config) -> ScanResult {
    // Build owned sets to move into the filter closure
    let auto_excluded: HashSet<String> = AUTO_EXCLUDED.iter().map(|s| s.to_string()).collect();
    let ignored_folders: HashSet<String> = config.ignore_folders.iter().cloned().collect();
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
    let ignored_files: HashSet<String> = config.ignore_files.iter().cloned().collect();

    // Clones for the filter closure (must be 'static)
    let auto_excl_f = auto_excluded.clone();
    let ignored_folders_f = ignored_folders.clone();

    let walker = WalkBuilder::new(root)
        // Full .gitignore support (negation, anchoring, etc.)
        .git_ignore(config.use_gitignore)
        .git_global(false)
        .git_exclude(config.use_gitignore)
        // .ignore files (like .gitignore but for non-git tools) — disabled for predictability
        .ignore(false)
        // Hidden directory handling respects IncludeHiddenDirs setting
        .hidden(!config.include_hidden_dirs)
        // Prune auto-excluded entries and config-ignored folders early
        .filter_entry(move |e| {
            let name = e.file_name().to_string_lossy();
            if auto_excl_f.contains(name.as_ref()) {
                return false;
            }
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false)
                && ignored_folders_f.contains(name.as_ref())
            {
                return false;
            }
            true
        })
        .build();

    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let max_size_bytes = config.max_file_size_kb * 1024;

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Skip the root itself
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
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

        if is_dir {
            dirs.push(ScanEntry {
                path: entry.path().to_path_buf(),
                relative_path: relative,
                is_dir: true,
                size_bytes: 0,
            });
            continue;
        }

        // ── File-level filters ──

        // Auto-excluded / config-ignored files
        if auto_excluded.contains(&name) || ignored_files.contains(&name) {
            continue;
        }

        // Extension filter
        if let Some(ext) = entry.path().extension() {
            let dot_ext = format!(".{}", ext.to_string_lossy().to_lowercase());
            if ignored_extensions.contains(&dot_ext) {
                continue;
            }
        }

        // Size filter
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        if config.max_file_size_kb > 0 && size > max_size_bytes {
            continue;
        }

        files.push(ScanEntry {
            path: entry.path().to_path_buf(),
            relative_path: relative,
            is_dir: false,
            size_bytes: size,
        });
    }

    // Sort alphabetically
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    dirs.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    ScanResult { files, dirs }
}
