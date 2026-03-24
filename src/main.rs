mod analyzer;
mod brain;
mod config;
mod generator;
mod neural;
mod scanner;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

/// 🌲 TreeC — Tree + Content Exporter & AI Neural Brain
#[derive(Parser, Debug)]
#[command(
    name = "treec",
    version,
    about = "🌲 TreeC — Repository documentation, structure export & AI Neural Link tool",
    long_about = "TreeC - Project Scanner and AI Neural Brain\n\n\
Commands:\n  \
treec                          Scan project and generate Tree.md\n  \
treec --neural-link            Create AI Second Brain (.brain/)\n  \
treec --update-brain           Update existing brain with latest scan\n  \
treec --config-neural <KEY>    Configure AI API key\n  \
treec --config                 Create default TreeC.toml config\n  \
treec --obsidian               Setup Obsidian vault for .brain/\n  \
treec --clean                  Clean generated files\n  \
treec --help                   Show this help"
)]
struct Cli {
    /// Root directory to scan (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// 🧠 Neural Link: scan project and generate .brain/ with AI
    #[arg(long = "neural-link")]
    neural_link: bool,

    /// 🔄 Update existing .brain/ with latest project changes
    #[arg(long = "update-brain")]
    update_brain: bool,

    /// ⚙️ Configure Neural Link: treec --config-neural <PROVIDER> <API_KEY>
    /// Providers: gemini, openai, claude
    #[arg(long = "config-neural", num_args = 2, value_names = ["PROVIDER", "API_KEY"])]
    config_neural: Option<Vec<String>>,

    /// 📄 Create default TreeC.toml configuration file
    #[arg(long = "config")]
    init_config: bool,

    /// 🔮 Setup Obsidian vault for .brain/
    #[arg(long = "obsidian")]
    obsidian: bool,

    /// 🧹 Clean all generated files (Tree.md, Structure.*, .brain/)
    #[arg(long = "clean")]
    clean: bool,

    /// 🔑 Remove Neural Link API configuration from TreeC.toml
    #[arg(long = "neural-link-remove-api")]
    remove_api: bool,
}

fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    let root = std::fs::canonicalize(&cli.path).unwrap_or_else(|_| {
        eprintln!("[TreeC] Error: Cannot resolve path '{}'", cli.path.display());
        std::process::exit(1);
    });

    // ── Handle --config (create TreeC.toml) ──
    if cli.init_config {
        handle_init_config(&root);
        return;
    }

    // ── Handle --config-neural ──
    if let Some(args) = &cli.config_neural {
        if args.len() == 2 {
            handle_config_neural(&root, &args[0], &args[1]);
        } else {
            eprintln!("[TreeC] Usage: treec --config-neural <PROVIDER> <API_KEY>");
            eprintln!("   Providers: gemini, openai, claude");
            std::process::exit(1);
        }
        return;
    }

    // ── Handle --neural-link-remove-api ──
    if cli.remove_api {
        println!("🔑 Removing Neural Link API configuration...");
        match config::Config::remove_neural_config(&root) {
            Ok(_) => {
                println!("✅ [NeuralLink] section removed from TreeC.toml");
                println!("   API key, provider, and model cleared.");
            }
            Err(e) => eprintln!("[TreeC] Error: {}", e),
        }
        return;
    }

    // ── Handle --obsidian ──
    if cli.obsidian {
        handle_obsidian(&root);
        return;
    }

    // ── Handle --clean ──
    if cli.clean {
        handle_clean(&root);
        return;
    }

    // ── Main scan pipeline ──
    let project_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Project".to_string());

    println!(
        "🌲 TreeC v{} | Scanning '{}'...",
        env!("CARGO_PKG_VERSION"),
        project_name
    );

    let config = config::Config::load(&root);
    let (md_content, text_files, total_folders, total_loc) =
        run_scan_pipeline(&root, &project_name, &config);

    // ── Neural Link (full brain creation) ──
    if cli.neural_link {
        handle_neural_link(&root, &md_content, &config, &start);
    }

    // ── Update Brain (incremental update) ──
    if cli.update_brain {
        handle_update_brain(&root, &md_content, &config, text_files, total_folders, total_loc, &start);
    }

    let elapsed = start.elapsed();
    println!("\n✅ Completed in {:.2?}", elapsed);
}

// ═══════════════════════════════════════════════════════════════════
// Scan Pipeline
// ═══════════════════════════════════════════════════════════════════

/// Run the full scan → analyze → generate pipeline. Returns (md_content, text_files, folders, loc).
fn run_scan_pipeline(
    root: &std::path::Path,
    project_name: &str,
    config: &config::Config,
) -> (String, usize, usize, usize) {
    let scan_result = scanner::scan_project(root, config);
    let total_files = scan_result.files.len();
    let total_folders = scan_result.dirs.len();
    println!(
        "   📂 Found {} files in {} folders",
        total_files, total_folders
    );

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
    println!(
        "   📄 {} text files, {} binary files skipped",
        text_files, binary_count
    );
    println!("   📊 {} total lines of code", total_loc);

    let tree_string =
        generator::build_tree_string(project_name, &scan_result.files, &scan_result.dirs);

    let md_content = generator::generate_markdown(
        project_name,
        &tree_string,
        &file_metas,
        text_files,
        total_folders,
        total_loc,
        root,
    );

    // Write artifacts
    let mut artifacts: Vec<&str> = Vec::new();

    if config.generate_markdown {
        if std::fs::write(root.join("Tree.md"), &md_content).is_ok() {
            artifacts.push("Tree.md");
        }
    }

    if config.generate_json {
        let json = generator::generate_json(
            project_name, &file_metas, text_files, total_folders, total_loc,
        );
        if std::fs::write(root.join("Structure.json"), &json).is_ok() {
            artifacts.push("Structure.json");
        }
    }

    if config.generate_txt {
        let txt = generator::generate_txt(project_name, &tree_string);
        if std::fs::write(root.join("Structure.txt"), &txt).is_ok() {
            artifacts.push("Structure.txt");
        }
    }

    println!("   📦 Artifacts: {}", artifacts.join(", "));

    (md_content, text_files, total_folders, total_loc)
}

// ═══════════════════════════════════════════════════════════════════
// Command Handlers
// ═══════════════════════════════════════════════════════════════════

/// `treec --config` — Create default TreeC.toml
fn handle_init_config(root: &std::path::Path) {
    let toml_path = root.join("TreeC.toml");
    if toml_path.exists() {
        println!("⚙️  TreeC.toml already exists.");
        println!("   Edit it manually or delete and re-run.");
        return;
    }

    let default_config = r#"# ─────────────────────────────────────────────
# TreeC.toml — Configuration for TreeC
# ─────────────────────────────────────────────

[General]
MaxFileSizeKB = 1024
UseGitIgnore = true
DetectLanguage = true
CountLines = true

[Exports]
GenerateMarkdown = true
GenerateJson = true
GenerateTxt = true

[Ignore]
Folders = ["target", "node_modules", ".git", "dist", "build", ".obsidian", ".brain"]
Extensions = [".exe", ".dll", ".so", ".dylib", ".o", ".obj", ".bin", ".img", ".iso", ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".mp3", ".mp4", ".avi", ".mov", ".zip", ".tar", ".gz", ".rar", ".7z", ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".woff", ".woff2", ".ttf", ".eot"]
Files = []

# [NeuralLink]
# ApiKey = ""
# Model = "gemini-2.0-flash"
# Provider = "gemini"
"#;

    match std::fs::write(&toml_path, default_config) {
        Ok(_) => {
            println!("✅ TreeC.toml created!");
            println!("   Edit [NeuralLink] section to configure AI.");
        }
        Err(e) => eprintln!("[TreeC] Error: {}", e),
    }
}

/// `treec --config-neural <provider> <key>` — Save provider + API key
fn handle_config_neural(root: &std::path::Path, provider: &str, api_key: &str) {
    let provider_lower = provider.to_lowercase();

    // Validate provider
    let (valid_provider, default_model) = match provider_lower.as_str() {
        "gemini" | "google" => ("gemini", "gemini-2.0-flash"),
        "openai" | "gpt" => ("openai", "gpt-4.1-mini"),
        "claude" | "anthropic" => ("claude", "claude-sonnet-4-20250514"),
        _ => {
            eprintln!("[TreeC] Error: Unknown provider '{}'", provider);
            eprintln!("   Supported: gemini, openai, claude");
            std::process::exit(1);
        }
    };

    println!("⚙️  Configuring Neural Link...");
    println!("   Provider: {}", valid_provider);
    println!("   Model: {}", default_model);

    match config::Config::save_neural_config(root, api_key, valid_provider, default_model) {
        Ok(_) => {
            println!("✅ Neural Link configured in TreeC.toml");
            println!("   You can now run: treec --neural-link");
        }
        Err(e) => {
            eprintln!("[TreeC] Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// `treec --neural-link` — Full brain creation
fn handle_neural_link(
    root: &std::path::Path,
    md_content: &str,
    config: &config::Config,
    start: &Instant,
) {
    println!("\n🧠 Neural Link activated!");
    println!("   Provider: {} | Model: {}", config.neural_provider, config.neural_model);

    let api_key = match &config.neural_api_key {
        Some(key) if !key.is_empty() => key.clone(),
        _ => {
            eprintln!("[TreeC] Error: No API key configured.");
            eprintln!("   Run: treec --config-neural <PROVIDER> <API_KEY>");
            eprintln!("   Providers: gemini, openai, claude");
            std::process::exit(1);
        }
    };

    match neural::execute_neural_link(root, md_content, &api_key, &config.neural_model, &config.neural_provider) {
        Ok(_) => {
            let elapsed = start.elapsed();
            println!("\n🧠 Neural Link completed in {:.2?}", elapsed);
            println!("   Brain: .brain/ directory ready");
            println!("   💡 Tip: Run 'treec --obsidian' to setup the vault");
        }
        Err(e) => {
            eprintln!("\n[TreeC] Neural Link Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// `treec --update-brain` — Incremental brain update
fn handle_update_brain(
    root: &std::path::Path,
    md_content: &str,
    config: &config::Config,
    text_files: usize,
    total_folders: usize,
    total_loc: usize,
    start: &Instant,
) {
    let brain_dir = root.join(".brain");
    if !brain_dir.exists() {
        eprintln!("[TreeC] Error: .brain/ not found. Run 'treec --neural-link' first.");
        std::process::exit(1);
    }

    println!("\n🔄 Updating brain...");

    // 1. Always update tree.md with the latest scan
    if let Err(e) = brain::update_tree(root, md_content) {
        eprintln!("[TreeC] Error updating tree.md: {}", e);
    } else {
        println!("   📝 tree.md updated");
    }

    // 2. Update memory.md with a new entry
    let memory_entry = format!(
        "\n## Memory Entry - {}\n- Project rescanned\n- Files: {}, Folders: {}, LOC: {}\n- Brain updated with latest structure\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
        text_files,
        total_folders,
        total_loc
    );
    let memory_path = brain_dir.join("memory.md");
    if let Ok(mut existing) = std::fs::read_to_string(&memory_path) {
        existing.push_str(&memory_entry);
        if std::fs::write(&memory_path, &existing).is_ok() {
            println!("   📝 memory.md updated");
        }
    }

    // 3. Update changelog.md
    let changelog_entry = format!(
        "\n## Change - {}\n- File modified: .brain/tree.md, .brain/memory.md\n- Description: Brain updated with latest project scan\n- Reason: treec --update-brain\n- Risk: None\n- Status: Complete\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
    );
    let changelog_path = brain_dir.join("changelog.md");
    if let Ok(mut existing) = std::fs::read_to_string(&changelog_path) {
        existing.push_str(&changelog_entry);
        if std::fs::write(&changelog_path, &existing).is_ok() {
            println!("   📝 changelog.md updated");
        }
    }

    // 4. If API key is available, also regenerate context with AI
    if let Some(api_key) = &config.neural_api_key {
        if !api_key.is_empty() {
            println!("   🔗 Sending updated project to AI...");
            match neural::execute_neural_link(root, md_content, api_key, &config.neural_model, &config.neural_provider) {
                Ok(_) => {
                    println!("   ✅ Brain files regenerated");
                }
                Err(e) => {
                    eprintln!("   ⚠️  AI update failed: {} (local files still updated)", e);
                }
            }
        }
    }

    let elapsed = start.elapsed();
    println!("\n🔄 Brain update completed in {:.2?}", elapsed);
}

/// `treec --obsidian` — Setup Obsidian vault
fn handle_obsidian(root: &std::path::Path) {
    println!("🔮 Setting up Obsidian vault...");

    // Ensure .brain/ exists
    let brain_dir = root.join(".brain");
    if !brain_dir.exists() {
        println!("   📁 Creating .brain/ directory...");
        if let Err(e) = brain::init_brain(root) {
            eprintln!("[TreeC] Error: {}", e);
            std::process::exit(1);
        }
    }

    // Create .obsidian/ inside .brain/ for vault config
    let obsidian_dir = brain_dir.join(".obsidian");
    if let Err(e) = std::fs::create_dir_all(&obsidian_dir) {
        eprintln!("[TreeC] Error creating .obsidian/: {}", e);
        std::process::exit(1);
    }

    // Write Obsidian app config
    let app_json = r#"{
  "showLineNumber": true,
  "strictLineBreaks": false,
  "livePreview": true,
  "defaultViewMode": "preview",
  "readableLineLength": true
}"#;
    let _ = std::fs::write(obsidian_dir.join("app.json"), app_json);

    // Write graph config for neural visualization
    let graph_json = r#"{
  "collapse-filter": false,
  "search": "",
  "showTags": false,
  "showAttachments": false,
  "hideUnresolved": false,
  "showOrphans": true,
  "collapse-color": false,
  "colorGroups": [
    { "query": "path:knowledge", "color": { "a": 1, "rgb": 52377 } },
    { "query": "tag:#core", "color": { "a": 1, "rgb": 16711680 } }
  ],
  "collapse-display": false,
  "lineSizeMultiplier": 1,
  "nodeSizeMultiplier": 1,
  "fontSize": 14,
  "textFadeMultiplier": 0,
  "collapse-forces": false,
  "centerStrength": 0.518713248970312,
  "repelStrength": 10,
  "linkStrength": 1,
  "linkDistance": 250
}"#;
    let _ = std::fs::write(obsidian_dir.join("graph.json"), graph_json);

    // Write workspace config
    let workspace_json = r#"{
  "main": {
    "id": "main",
    "type": "split",
    "children": [
      {
        "id": "editor",
        "type": "leaf",
        "state": {
          "type": "markdown",
          "state": { "file": "index.md", "mode": "preview" }
        }
      }
    ],
    "direction": "vertical"
  }
}"#;
    let _ = std::fs::write(obsidian_dir.join("workspace.json"), workspace_json);

    println!("✅ Obsidian vault configured in .brain/");
    println!("   📂 Open '{0}' as an Obsidian vault", brain_dir.display());
    println!("   🔗 Graph view will show all neural connections");
}

/// `treec --clean` — Remove all generated files
fn handle_clean(root: &std::path::Path) {
    println!("🧹 Cleaning generated files...");

    let files_to_remove = ["Tree.md", "Structure.json", "Structure.txt"];
    for file in &files_to_remove {
        let path = root.join(file);
        if path.exists() {
            match std::fs::remove_file(&path) {
                Ok(_) => println!("   🗑️  Removed {}", file),
                Err(e) => eprintln!("   ⚠️  Cannot remove {}: {}", file, e),
            }
        }
    }

    let brain_dir = root.join(".brain");
    if brain_dir.exists() {
        match std::fs::remove_dir_all(&brain_dir) {
            Ok(_) => println!("   🗑️  Removed .brain/"),
            Err(e) => eprintln!("   ⚠️  Cannot remove .brain/: {}", e),
        }
    }

    println!("✅ Clean complete.");
}
