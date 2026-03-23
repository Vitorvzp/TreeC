mod analyzer;
mod config;
mod generator;
mod scanner;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

/// 🌲 TreeC — Tree + Content Exporter
/// High-performance repository documentation tool.
#[derive(Parser, Debug)]
#[command(name = "treec", version, about = "🌲 TreeC — Repository documentation & structure export tool")]
struct Cli {
    /// Root directory to scan (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    let root = std::fs::canonicalize(&cli.path).unwrap_or_else(|_| {
        eprintln!("[TreeC] Error: Cannot resolve path '{}'", cli.path.display());
        std::process::exit(1);
    });

    let project_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Project".to_string());

    println!("🌲 TreeC v{} | Scanning '{}'...", env!("CARGO_PKG_VERSION"), project_name);

    // ── 1. Load Configuration ──
    let config = config::Config::load(&root);

    // ── 2. Scan Project ──
    let scan_result = scanner::scan_project(&root, &config);
    let total_files = scan_result.files.len();
    let total_folders = scan_result.dirs.len();
    println!("   📂 Found {} files in {} folders", total_files, total_folders);

    // ── 3. Analyze Files ──
    let mut file_metas: Vec<analyzer::FileMeta> = Vec::with_capacity(total_files);
    let mut total_loc: usize = 0;
    let mut binary_count: usize = 0;

    for entry in &scan_result.files {
        if let Some(meta) = analyzer::analyze_file(
            &entry.path,
            &entry.relative_path,
            entry.size_bytes,
            config.detect_language,
            config.count_lines,
        ) {
            if meta.is_binary {
                binary_count += 1;
            } else {
                total_loc += meta.line_count;
            }
            file_metas.push(meta);
        }
    }

    let text_files = total_files - binary_count;
    println!("   📄 {} text files, {} binary files skipped", text_files, binary_count);
    println!("   📊 {} total lines of code", total_loc);

    // ── 4. Build ASCII Tree ──
    let tree_string = generator::build_tree_string(
        &project_name,
        &scan_result.files,
        &scan_result.dirs,
    );

    // ── 5. Generate Outputs ──
    let mut artifacts_written: Vec<&str> = Vec::new();

    if config.generate_markdown {
        let md = generator::generate_markdown(
            &project_name,
            &tree_string,
            &file_metas,
            text_files,
            total_folders,
            total_loc,
            &root,
        );
        if let Err(e) = std::fs::write(root.join("Tree.md"), &md) {
            eprintln!("[TreeC] Error writing Tree.md: {}", e);
        } else {
            artifacts_written.push("Tree.md");
        }
    }

    if config.generate_json {
        let json = generator::generate_json(
            &project_name,
            &file_metas,
            text_files,
            total_folders,
            total_loc,
        );
        if let Err(e) = std::fs::write(root.join("Structure.json"), &json) {
            eprintln!("[TreeC] Error writing Structure.json: {}", e);
        } else {
            artifacts_written.push("Structure.json");
        }
    }

    if config.generate_txt {
        let txt = generator::generate_txt(&project_name, &tree_string);
        if let Err(e) = std::fs::write(root.join("Structure.txt"), &txt) {
            eprintln!("[TreeC] Error writing Structure.txt: {}", e);
        } else {
            artifacts_written.push("Structure.txt");
        }
    }

    // ── 6. Report ──
    let elapsed = start.elapsed();
    println!("\n✅ Documentation generated in {:.2?}", elapsed);
    println!("   Artifacts: {}", artifacts_written.join(", "));
}
