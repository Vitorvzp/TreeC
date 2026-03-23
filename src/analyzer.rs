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

    // ── 1. Null Byte Detection (first 1024 bytes) ──
    let mut header = [0u8; 1024];
    let bytes_read = reader.read(&mut header).unwrap_or(0);
    let is_binary = header[..bytes_read].iter().any(|&b| b == 0x00);

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
