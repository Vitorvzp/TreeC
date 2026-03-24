use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

use crate::analyzer::{self, FileMeta};
use crate::scanner::ScanEntry;

// ═══════════════════════════════════════════════════════════════════
// Data Structures
// ═══════════════════════════════════════════════════════════════════

/// JSON output schema for Structure.json
#[derive(Serialize)]
pub struct JsonOutput {
    pub project: String,
    pub stats: JsonStats,
    pub files: Vec<JsonFile>,
}

#[derive(Serialize)]
pub struct JsonStats {
    pub files: usize,
    pub folders: usize,
    pub loc: usize,
}

#[derive(Serialize)]
pub struct JsonFile {
    pub path: String,
    #[serde(rename = "sizeKB")]
    pub size_kb: f64,
    pub lines: usize,
    pub language: String,
}

// ═══════════════════════════════════════════════════════════════════
// Tree.md Generation
// ═══════════════════════════════════════════════════════════════════

/// Generate the full Tree.md content.
pub fn generate_markdown(
    project_name: &str,
    tree_string: &str,
    file_metas: &[FileMeta],
    total_files: usize,
    total_folders: usize,
    total_loc: usize,
    root: &Path,
) -> String {
    let mut md = String::with_capacity(total_files * 512);

    // # Header
    md.push_str(&format!("# {}\n\n", project_name));

    // ## Summary
    md.push_str("## Summary\n\n");
    md.push_str(&format!("- Total Files: {}\n", total_files));
    md.push_str(&format!("- Total Folders: {}\n", total_folders));
    md.push_str(&format!("- Total Lines of Code (LOC): {}\n\n", total_loc));

    // ## Tree
    md.push_str("## Tree\n\n");
    md.push_str("```\n");
    md.push_str(tree_string);
    md.push_str("\n```\n\n");

    // ---
    md.push_str("---\n\n");

    // ## Files
    md.push_str("## Files\n\n");

    for meta in file_metas {
        if meta.is_binary {
            continue;
        }

        md.push_str(&format!("### {}\n\n", meta.relative_path));
        md.push_str(&format!(
            "> **Language:** {} | **Size:** {:.1} KB | **Lines:** {}\n\n",
            meta.language, meta.size_kb, meta.line_count
        ));

        // Read file content
        let file_path = root.join(
            meta.relative_path
                .replace('/', std::path::MAIN_SEPARATOR_STR),
        );
        if let Some(content) = analyzer::read_file_content(&file_path) {
            md.push_str(&format!("```{}\n", meta.language));
            md.push_str(&content);
            if !content.ends_with('\n') {
                md.push('\n');
            }
            md.push_str("```\n\n");
        }

        md.push_str("---\n\n");
    }

    md
}

// ═══════════════════════════════════════════════════════════════════
// Structure.json Generation
// ═══════════════════════════════════════════════════════════════════

/// Generate Structure.json content.
pub fn generate_json(
    project_name: &str,
    file_metas: &[FileMeta],
    total_files: usize,
    total_folders: usize,
    total_loc: usize,
) -> String {
    let output = JsonOutput {
        project: project_name.to_string(),
        stats: JsonStats {
            files: total_files,
            folders: total_folders,
            loc: total_loc,
        },
        files: file_metas
            .iter()
            .filter(|m| !m.is_binary)
            .map(|m| JsonFile {
                path: m.relative_path.clone(),
                size_kb: (m.size_kb * 10.0).round() / 10.0,
                lines: m.line_count,
                language: m.language.clone(),
            })
            .collect(),
    };

    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

// ═══════════════════════════════════════════════════════════════════
// Structure.txt Generation
// ═══════════════════════════════════════════════════════════════════

/// Generate Structure.txt content (ASCII tree only).
pub fn generate_txt(project_name: &str, tree_string: &str) -> String {
    format!("{}\n{}", project_name, tree_string)
}

// ═══════════════════════════════════════════════════════════════════
// ASCII Tree Builder
// ═══════════════════════════════════════════════════════════════════

/// Represents a node in the directory tree.
#[derive(Debug)]
struct TreeNode {
    name: String,
    is_dir: bool,
    children: BTreeMap<String, TreeNode>,
}

impl TreeNode {
    fn new(name: &str, is_dir: bool) -> Self {
        Self {
            name: name.to_string(),
            is_dir,
            children: BTreeMap::new(),
        }
    }
}

/// Build the ASCII tree string from scan entries.
pub fn build_tree_string(project_name: &str, files: &[ScanEntry], dirs: &[ScanEntry]) -> String {
    // Build tree structure
    let mut root = TreeNode::new(project_name, true);

    // Insert directories
    for dir in dirs {
        insert_path(&mut root, &dir.relative_path, true);
    }

    // Insert files
    for file in files {
        insert_path(&mut root, &file.relative_path, false);
    }

    // Render
    let mut output = String::with_capacity(files.len() * 64);
    output.push_str(project_name);
    output.push('\n');

    let children: Vec<&TreeNode> = sorted_children(&root);
    let count = children.len();
    for (i, child) in children.iter().enumerate() {
        let is_last = i == count - 1;
        render_node(child, "", is_last, &mut output);
    }

    output
}

/// Insert a relative path into the tree structure.
fn insert_path(root: &mut TreeNode, relative_path: &str, is_dir: bool) {
    let parts: Vec<&str> = relative_path.split('/').collect();
    let mut current = root;

    for (i, part) in parts.iter().enumerate() {
        let is_leaf = i == parts.len() - 1;
        let node_is_dir = if is_leaf { is_dir } else { true };

        current = current
            .children
            .entry(part.to_string())
            .or_insert_with(|| TreeNode::new(part, node_is_dir));

        // Ensure intermediate nodes are marked as dirs
        if !is_leaf {
            current.is_dir = true;
        }
    }
}

/// Get children sorted: directories first (alphabetical), then files (alphabetical).
fn sorted_children(node: &TreeNode) -> Vec<&TreeNode> {
    let mut dirs: Vec<&TreeNode> = node.children.values().filter(|n| n.is_dir).collect();
    let mut files: Vec<&TreeNode> = node.children.values().filter(|n| !n.is_dir).collect();

    dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    dirs.append(&mut files);
    dirs
}

/// Recursively render a tree node into the output string.
fn render_node(node: &TreeNode, prefix: &str, is_last: bool, output: &mut String) {
    let connector = if is_last { "└── " } else { "├── " };
    output.push_str(prefix);
    output.push_str(connector);
    output.push_str(&node.name);
    output.push('\n');

    let child_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    let children = sorted_children(node);
    let count = children.len();
    for (i, child) in children.iter().enumerate() {
        let child_is_last = i == count - 1;
        render_node(child, &child_prefix, child_is_last, output);
    }
}
