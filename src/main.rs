mod agent;
mod analyzer;
mod brain;
mod config;
mod generator;
mod neural;
mod scanner;
mod tui;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
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
treec --neural-link --dry-run  Preview prompt without calling AI\n  \
treec --update-brain           Update existing brain with latest scan\n  \
treec --config-neural <KEY>    Configure AI API key\n  \
treec --config                 Create default TreeC.toml config\n  \
treec --obsidian               Setup Obsidian vault for .brain/\n  \
treec --status                 Show current TreeC status\n  \
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

    /// ⚙️ Configure Neural Link: treec --config-neural <PROVIDER> [API_KEY]
    /// Providers: gemini, openai, claude, ollama
    /// API key is optional for ollama (local deployment)
    #[arg(long = "config-neural", num_args = 1..=2, value_names = ["PROVIDER", "API_KEY"])]
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

    /// 📊 Show current TreeC status (config, brain, API key, security)
    #[arg(long = "status")]
    status: bool,

    /// 🎯 Selectively regenerate specific brain files (comma-separated keys)
    /// Used with --neural-link or --update-brain
    /// Example: treec --neural-link --brain-files context,architecture,tasks
    /// Available keys: context, architecture, decisions, roadmap, patterns, releases,
    ///                 modules, functions, api, database, models, services,
    ///                 readme, documentation, tasks, backlog, bugs, project, goals
    #[arg(long = "brain-files", value_name = "FILES")]
    brain_files: Option<String>,

    /// 🔍 Dry-run: show what would be sent to the AI without calling the API
    /// Use with --neural-link or --update-brain
    #[arg(long = "dry-run")]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// 🖥️  Open the TUI dashboard
    Tui,

    /// 🤖 Manage specialized agents
    Agent {
        #[command(subcommand)]
        cmd: AgentCmd,
    },

    /// 🎯 Orchestrator operations
    Orchestrator {
        #[command(subcommand)]
        cmd: OrchestratorCmd,
    },
}

#[derive(clap::Subcommand, Debug)]
enum AgentCmd {
    /// Create agent directory with seed files
    Scaffold {
        name: String,
        #[arg(long, default_value = "Custom Agent")]
        role: String,
    },
    /// Write content to a specific brain file of an agent
    Write {
        name: String,
        file: String,
        #[arg(long)]
        content: String,
    },
    /// Activate a pending agent (moves from _pending/ to _active/ and scaffolds)
    Activate { name: String },
    /// List agents
    List {
        #[arg(long)]
        pending: bool,
    },
    /// Show agent status
    Status { name: String },
}

#[derive(clap::Subcommand, Debug)]
enum OrchestratorCmd {
    /// Read orchestrator/tasks.md
    Read,
    /// Write content to an orchestrator file
    Write {
        file: String,
        #[arg(long)]
        content: String,
    },
    /// Show orchestrator status
    Status,
}

fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    // ── Subcommands dispatch (agent, tui, orchestrator) ──
    if let Some(cmd) = cli.command {
        let root = std::path::Path::new(".");
        match cmd {
            Commands::Tui => {
                tui::run_tui(root).unwrap_or_else(|e| {
                    eprintln!("[TreeC] TUI error: {}", e);
                    std::process::exit(1);
                });
                return;
            }
            Commands::Agent { cmd } => {
                handle_agent_cmd(root, cmd);
                return;
            }
            Commands::Orchestrator { cmd } => {
                handle_orchestrator_cmd(root, cmd);
                return;
            }
        }
    }

    let root = std::fs::canonicalize(&cli.path).unwrap_or_else(|_| {
        eprintln!(
            "[TreeC] Error: Cannot resolve path '{}'",
            cli.path.display()
        );
        std::process::exit(1);
    });

    // ── Handle --config ──
    if cli.init_config {
        handle_init_config(&root);
        return;
    }

    // ── Handle --config-neural ──
    if let Some(args) = &cli.config_neural {
        let provider = &args[0];
        // API key is optional for ollama; use "local" as placeholder
        let api_key = args.get(1).map(|s| s.as_str()).unwrap_or("local");
        handle_config_neural(&root, provider, api_key);
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

    // ── Handle --status ──
    if cli.status {
        let config = config::Config::load(&root);
        handle_status(&root, &config);
        return;
    }

    // ── Parse --brain-files ──
    let brain_files: Vec<String> = cli
        .brain_files
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

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

    // ── Dependency Analysis ──
    let deps = analyzer::detect_dependencies(&root);
    if !deps.is_empty() {
        let deps_md = analyzer::format_dependencies_md(&deps, &project_name);
        println!("   📦 {} dependencies detected", deps.len());
        // Write to perception/dependencies.md if brain exists
        let brain_dir = root.join(".brain");
        if brain_dir.exists() {
            let _ = brain::update_dependencies(&root, &deps_md);
        }
    }

    // ── Neural Link ──
    if cli.neural_link {
        handle_neural_link(
            &root,
            &md_content,
            &config,
            &brain_files,
            &start,
            cli.dry_run,
        );
    }

    // ── Update Brain ──
    if cli.update_brain {
        handle_update_brain(
            &root,
            &md_content,
            &config,
            text_files,
            total_folders,
            total_loc,
            &brain_files,
            &start,
            cli.dry_run,
        );
    }

    let elapsed = start.elapsed();
    println!("\n✅ Completed in {:.2?}", elapsed);
}

// ═══════════════════════════════════════════════════════════════════
// Scan Pipeline
// ═══════════════════════════════════════════════════════════════════

/// Run the full scan → analyze → generate pipeline.
/// File analysis is parallelized with rayon for performance on large repos.
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

    // ── Parallel file analysis with progress bar ──
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("   {spinner:.cyan} Analyzing [{bar:30.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("=>-"),
    );

    let detect_language = config.detect_language;
    let count_lines = config.count_lines;
    let pb_clone = pb.clone();

    // Parallel analysis — indicatif's ProgressBar is Arc-backed (thread-safe)
    let raw_metas: Vec<Option<analyzer::FileMeta>> = scan_result
        .files
        .par_iter()
        .map(|entry| {
            let result = analyzer::analyze_file(
                &entry.path,
                &entry.relative_path,
                entry.size_bytes,
                detect_language,
                count_lines,
            );
            pb_clone.inc(1);
            result
        })
        .collect();

    pb.finish_and_clear();

    // Collect results sequentially
    let mut file_metas: Vec<analyzer::FileMeta> = Vec::with_capacity(total_files);
    let mut total_loc: usize = 0;
    let mut binary_count: usize = 0;

    for meta in raw_metas.into_iter().flatten() {
        if meta.is_binary {
            binary_count += 1;
        } else {
            total_loc += meta.line_count;
        }
        file_metas.push(meta);
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

    if config.generate_markdown && std::fs::write(root.join("Tree.md"), &md_content).is_ok() {
        artifacts.push("Tree.md");
    }
    if config.generate_json {
        let json = generator::generate_json(
            project_name,
            &file_metas,
            text_files,
            total_folders,
            total_loc,
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
IncludeHiddenDirs = false  # Set to true to include .github, .vscode, etc.

[Exports]
GenerateMarkdown = true
GenerateJson = true
GenerateTxt = true

[Ignore]
Folders = ["target", "node_modules", ".git", "dist", "build", ".obsidian", ".brain"]
Extensions = [".exe", ".dll", ".so", ".dylib", ".o", ".obj", ".bin", ".img", ".iso", ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".mp3", ".mp4", ".avi", ".mov", ".zip", ".tar", ".gz", ".rar", ".7z", ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".woff", ".woff2", ".ttf", ".eot"]
Files = []

# [NeuralLink]
# Provider = "gemini"          # gemini | openai | claude | ollama
# Model = "gemini-2.0-flash"   # or: gpt-4.1-mini | claude-sonnet-4-20250514 | llama3.2
# ApiKey = ""                  # Prefer: set TREEC_API_KEY env var instead
"#;

    match std::fs::write(&toml_path, default_config) {
        Ok(_) => {
            println!("✅ TreeC.toml created!");
            println!("   Configure AI: treec --config-neural gemini YOUR_KEY");
            println!("   Local AI:     treec --config-neural ollama llama3.2");
            println!("   💡 Tip: Use TREEC_API_KEY env var instead of storing key in file.");
        }
        Err(e) => eprintln!("[TreeC] Error: {}", e),
    }
}

fn handle_config_neural(root: &std::path::Path, provider: &str, api_key: &str) {
    let provider_lower = provider.to_lowercase();

    let (valid_provider, default_model) = match provider_lower.as_str() {
        "gemini" | "google" => ("gemini", "gemini-2.0-flash"),
        "openai" | "gpt" => ("openai", "gpt-4.1-mini"),
        "claude" | "anthropic" => ("claude", "claude-sonnet-4-20250514"),
        "ollama" | "local" => ("ollama", "llama3.2"),
        _ => {
            eprintln!("[TreeC] Error: Unknown provider '{}'", provider);
            eprintln!("   Supported: gemini, openai, claude, ollama");
            std::process::exit(1);
        }
    };

    println!("⚙️  Configuring Neural Link...");
    println!("   Provider : {}", valid_provider);
    println!("   Model    : {}", default_model);

    if valid_provider == "ollama" {
        println!("   Endpoint : http://localhost:11434 (local)");
        println!("   API key  : not required for local deployment");
    }

    // For ollama, use "local" as a placeholder key (ignored at runtime)
    let effective_key = if valid_provider == "ollama" {
        "local"
    } else {
        api_key
    };

    match config::Config::save_neural_config(root, effective_key, valid_provider, default_model) {
        Ok(_) => {
            println!("✅ Neural Link configured in TreeC.toml");
            println!("   You can now run: treec --neural-link");

            // Security: protect TreeC.toml in .gitignore (skip for ollama, no secret stored)
            if valid_provider != "ollama" {
                if !is_in_gitignore(root, "TreeC.toml") {
                    println!(
                        "\n   ⚠️  Security: TreeC.toml not in .gitignore — adding automatically..."
                    );
                    match add_to_gitignore(root, "TreeC.toml") {
                        Ok(_) => println!("      ✅ TreeC.toml added to .gitignore"),
                        Err(e) => eprintln!("      ⚠️  Could not update .gitignore: {}", e),
                    }
                } else {
                    println!("   🔒 Security: TreeC.toml already in .gitignore ✅");
                }
                let masked = &api_key[..api_key.len().min(8)];
                println!("\n   💡 Tip: For better security, use env var:");
                println!("      Windows : set TREEC_API_KEY={}...", masked);
                println!("      Linux   : export TREEC_API_KEY={}...", masked);
            }
        }
        Err(e) => {
            eprintln!("[TreeC] Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_neural_link(
    root: &std::path::Path,
    md_content: &str,
    config: &config::Config,
    brain_files: &[String],
    start: &Instant,
    dry_run: bool,
) {
    println!("\n🧠 Neural Link activated!");
    println!(
        "   Provider: {} | Model: {}",
        config.neural_provider, config.neural_model
    );

    warn_token_estimate(md_content, &config.neural_model);

    if dry_run {
        println!("\n🔍 Dry-run mode — no API call will be made.");
        println!("   The prompt above shows what would be sent to the AI.");
        if !brain_files.is_empty() {
            println!("   Selective files: {:?}", brain_files);
        } else {
            println!("   All brain files would be generated.");
        }
        println!("   Run without --dry-run to execute.");
        return;
    }

    // Ollama doesn't need an API key
    let api_key = if config.neural_provider == "ollama" {
        "local".to_string()
    } else {
        match &config.neural_api_key {
            Some(key) if !key.is_empty() => key.clone(),
            _ => {
                eprintln!("[TreeC] Error: No API key configured.");
                eprintln!("   Run: treec --config-neural <PROVIDER> <API_KEY>");
                eprintln!("   Or:  set TREEC_API_KEY=<YOUR_KEY>");
                std::process::exit(1);
            }
        }
    };

    match neural::execute_neural_link(
        root,
        md_content,
        &api_key,
        &config.neural_model,
        &config.neural_provider,
        brain_files,
    ) {
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

#[allow(clippy::too_many_arguments)]
fn handle_update_brain(
    root: &std::path::Path,
    md_content: &str,
    config: &config::Config,
    text_files: usize,
    total_folders: usize,
    total_loc: usize,
    brain_files: &[String],
    start: &Instant,
    dry_run: bool,
) {
    let brain_dir = root.join(".brain");
    if !brain_dir.exists() {
        eprintln!("[TreeC] Error: .brain/ not found. Run 'treec --neural-link' first.");
        std::process::exit(1);
    }

    println!("\n🔄 Updating brain...");

    // Update perception/tree.md
    if let Err(e) = brain::update_tree(root, md_content) {
        eprintln!("[TreeC] Error updating perception/tree.md: {}", e);
    } else {
        println!("   📝 perception/tree.md updated");
    }

    // Append to memory/long_term.md
    let memory_entry = format!(
        "\n## Entry — {}\n- Project rescanned: {} files, {} folders, {} LOC\n- Brain updated with latest structure\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
        text_files,
        total_folders,
        total_loc
    );
    if let Err(e) = brain::append_memory(root, &memory_entry) {
        eprintln!("[TreeC] Warning: {}", e);
    } else {
        println!("   📝 memory/long_term.md updated");
    }

    // Append to memory/changelog.md
    let changelog_entry = format!(
        "\n## Change — {}\n- File: .brain/perception/tree.md, .brain/memory/long_term.md\n- Description: Brain updated with latest project scan\n- Reason: treec --update-brain\n- Risk: None\n- Status: Complete\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
    );
    if let Err(e) = brain::append_changelog(root, &changelog_entry) {
        eprintln!("[TreeC] Warning: {}", e);
    } else {
        println!("   📝 memory/changelog.md updated");
    }

    if dry_run {
        warn_token_estimate(md_content, &config.neural_model);
        println!("\n🔍 Dry-run mode — skipping AI call.");
        println!("   Local brain files were updated. AI regeneration skipped.");
        return;
    }

    // AI refresh if key available
    let has_key = config.neural_provider == "ollama"
        || config
            .neural_api_key
            .as_deref()
            .map(|k| !k.is_empty())
            .unwrap_or(false);

    if has_key {
        warn_token_estimate(md_content, &config.neural_model);
        let key = if config.neural_provider == "ollama" {
            "local".to_string()
        } else {
            config.neural_api_key.clone().unwrap_or_default()
        };
        println!("   🔗 Sending updated project to AI...");
        match neural::execute_neural_link(
            root,
            md_content,
            &key,
            &config.neural_model,
            &config.neural_provider,
            brain_files,
        ) {
            Ok(_) => println!("   ✅ Brain files regenerated"),
            Err(e) => eprintln!("   ⚠️  AI update failed: {} (local files still updated)", e),
        }
    }

    let elapsed = start.elapsed();
    println!("\n🔄 Brain update completed in {:.2?}", elapsed);
}

fn handle_obsidian(root: &std::path::Path) {
    println!("🔮 Setting up Obsidian vault...");

    let brain_dir = root.join(".brain");
    if !brain_dir.exists() {
        println!("   📁 Creating .brain/ directory...");
        if let Err(e) = brain::init_brain(root) {
            eprintln!("[TreeC] Error: {}", e);
            std::process::exit(1);
        }
    }

    let obsidian_dir = brain_dir.join(".obsidian");
    if let Err(e) = std::fs::create_dir_all(&obsidian_dir) {
        eprintln!("[TreeC] Error creating .obsidian/: {}", e);
        std::process::exit(1);
    }

    let _ = std::fs::write(
        obsidian_dir.join("app.json"),
        r#"{
  "showLineNumber": true,
  "strictLineBreaks": false,
  "livePreview": true,
  "defaultViewMode": "preview",
  "readableLineLength": true
}"#,
    );

    let _ = std::fs::write(
        obsidian_dir.join("graph.json"),
        r#"{
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
}"#,
    );

    let _ = std::fs::write(
        obsidian_dir.join("workspace.json"),
        r#"{
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
}"#,
    );

    println!("✅ Obsidian vault configured in .brain/");
    println!("   📂 Open '{}' as an Obsidian vault", brain_dir.display());
    println!("   🔗 Graph view will show all neural connections");
}

fn handle_clean(root: &std::path::Path) {
    println!("🧹 Cleaning generated files...");
    for file in &["Tree.md", "Structure.json", "Structure.txt"] {
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

fn handle_status(root: &std::path::Path, config: &config::Config) {
    println!("📊 TreeC Status\n");

    // Config
    if root.join("TreeC.toml").exists() {
        println!("   ⚙️  TreeC.toml     ✅ found");
    } else {
        println!("   ⚙️  TreeC.toml     ❌ not found  →  run: treec --config");
    }

    // Tree.md
    let tree_md = root.join("Tree.md");
    if tree_md.exists() {
        let ts = modified_time(&tree_md);
        println!("   📄 Tree.md         ✅ exists  ({})", ts);
    } else {
        println!("   📄 Tree.md         ❌ not found  →  run: treec");
    }

    // .brain/
    let brain_dir = root.join(".brain");
    if brain_dir.exists() {
        // Detect brain version: new hierarchical (cortex/) vs old flat
        let is_new_structure = brain_dir.join("cortex").exists();
        let last_update_file = if is_new_structure {
            brain_dir.join("cortex").join("context.md")
        } else {
            brain_dir.join("context.md")
        };
        let ts = modified_time(&last_update_file);
        let structure_tag = if is_new_structure {
            "hierarchical"
        } else {
            "flat — run 'treec --neural-link' to upgrade"
        };
        println!(
            "   🧠 .brain/          ✅ exists  ({}) (last update: {})",
            structure_tag, ts
        );
    } else {
        println!("   🧠 .brain/          ❌ not found  →  run: treec --neural-link");
    }

    // Neural Link
    println!("\n   🤖 Neural Link:");
    println!("      Provider : {}", config.neural_provider);
    println!("      Model    : {}", config.neural_model);

    let env_key_set = std::env::var("TREEC_API_KEY").is_ok();

    if config.neural_provider == "ollama" {
        println!("      API Key  : N/A (local deployment)");
        println!("      Endpoint : http://localhost:11434");
    } else {
        match &config.neural_api_key {
            Some(_) => {
                let source = if env_key_set {
                    "TREEC_API_KEY env var"
                } else {
                    "TreeC.toml"
                };
                println!("      API Key  : ✅ configured  (source: {})", source);
            }
            None => {
                println!("      API Key  : ❌ not set");
                println!("         → run: treec --config-neural <PROVIDER> <KEY>");
                println!("         → or:  set TREEC_API_KEY=<YOUR_KEY>");
            }
        }
    }

    // Security
    println!("\n   🔒 Security:");
    if is_in_gitignore(root, "TreeC.toml") {
        println!("      TreeC.toml in .gitignore  ✅");
    } else {
        let has_file_key = config.neural_api_key.is_some() && !env_key_set;
        if has_file_key {
            println!("      TreeC.toml in .gitignore  ⚠️  NOT protected — API key at risk!");
            println!("         → run: treec --config-neural <PROVIDER> <KEY>  (auto-fixes)");
        } else {
            println!("      TreeC.toml in .gitignore  — not protected (no key stored)");
        }
    }

    if env_key_set {
        println!("      TREEC_API_KEY env var     ✅ set (preferred)");
    } else {
        println!("      TREEC_API_KEY env var     — not set (optional but recommended)");
    }
}

// ═══════════════════════════════════════════════════════════════════
// Security Helpers
// ═══════════════════════════════════════════════════════════════════

fn is_in_gitignore(root: &std::path::Path, entry: &str) -> bool {
    std::fs::read_to_string(root.join(".gitignore"))
        .map(|c| c.lines().any(|l| l.trim() == entry))
        .unwrap_or(false)
}

fn add_to_gitignore(root: &std::path::Path, entry: &str) -> Result<(), String> {
    let path = root.join(".gitignore");
    let mut content = std::fs::read_to_string(&path).unwrap_or_default();
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(&format!(
        "\n# TreeC — API key config (contains secrets)\n{}\n",
        entry
    ));
    std::fs::write(&path, &content).map_err(|e| format!("Cannot write .gitignore: {}", e))
}

// ═══════════════════════════════════════════════════════════════════
// Token & Cost Estimation
// ═══════════════════════════════════════════════════════════════════

/// Estimate token usage and cost before any AI API call.
/// Aborts (exit 1) if >90% of context window is used to prevent failed/truncated calls.
fn warn_token_estimate(content: &str, model: &str) {
    let estimated_tokens = (content.len() as f64 / 3.5) as usize;
    let context_limit = model_context_window(model);
    let usage_pct = (estimated_tokens * 100) / context_limit.max(1);
    let estimated_kb = content.len() / 1024;
    let cost_usd = estimate_cost_usd(estimated_tokens, model);

    println!(
        "   📏 Context: ~{} tokens ({} KB) | {:.0}% of context window | ~${:.4} USD",
        estimated_tokens, estimated_kb, usage_pct as f64, cost_usd
    );

    if usage_pct >= 90 {
        eprintln!(
            "   ❌ ERROR: ~{} tokens ≥ 90% of {} context window ({} tokens).",
            estimated_tokens, model, context_limit
        );
        eprintln!("      The AI call will likely fail or produce truncated results.");
        eprintln!("      Options:");
        eprintln!("        • Reduce MaxFileSizeKB in TreeC.toml");
        eprintln!("        • Add more folders/extensions to [Ignore]");
        eprintln!("        • Use --brain-files to regenerate only specific sections");
        eprintln!(
            "        • Switch to a model with a larger context window (e.g. gemini-2.0-flash)"
        );
        std::process::exit(1);
    } else if usage_pct >= 70 {
        println!(
            "   ⚠️  Warning: {}% of context window used. Responses may be truncated for large sections.",
            usage_pct
        );
        println!("      Tip: Use --brain-files to regenerate only what you need.");
    }
}

/// Approximate cost in USD for input tokens (prices per 1M tokens, 2026).
fn estimate_cost_usd(tokens: usize, model: &str) -> f64 {
    let price_per_1m: f64 = match model {
        m if m.contains("gemini-2.0-flash") => 0.10,
        m if m.contains("gemini-1.5-flash") => 0.075,
        m if m.contains("gemini-1.5-pro") => 1.25,
        m if m.contains("gpt-4.1-mini") => 0.40,
        m if m.contains("gpt-4.1") => 2.00,
        m if m.contains("gpt-4o-mini") => 0.15,
        m if m.contains("gpt-4o") => 2.50,
        m if m.contains("claude-haiku") => 0.80,
        m if m.contains("claude-sonnet") => 3.00,
        m if m.contains("claude-opus") => 15.00,
        m if m.contains("ollama") || m.contains("llama") || m.contains("mistral") => 0.0,
        _ => 1.00,
    };
    (tokens as f64 / 1_000_000.0) * price_per_1m
}

/// Approximate context window size (tokens) for known models.
fn model_context_window(model: &str) -> usize {
    match model {
        m if m.starts_with("gemini-2.0") => 1_000_000,
        m if m.starts_with("gemini-1.5") => 1_000_000,
        m if m.starts_with("gemini") => 32_768,
        m if m.starts_with("gpt-4.1") => 128_000,
        m if m.starts_with("gpt-4o") => 128_000,
        m if m.starts_with("gpt-4") => 128_000,
        m if m.starts_with("gpt-3.5") => 16_385,
        m if m.starts_with("claude") => 200_000,
        _ => 32_768, // conservative default
    }
}

// ═══════════════════════════════════════════════════════════════════
// Utility
// ═══════════════════════════════════════════════════════════════════

fn modified_time(path: &std::path::Path) -> String {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|t| {
            let dt: chrono::DateTime<chrono::Local> = t.into();
            dt.format("%Y-%m-%d %H:%M").to_string()
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

fn handle_agent_cmd(root: &std::path::Path, cmd: AgentCmd) {
    match cmd {
        AgentCmd::Scaffold { name, role } => {
            agent::cmd_scaffold(root, &name, &role).unwrap_or_else(|e| {
                eprintln!("[TreeC] {}", e);
                std::process::exit(1);
            });
        }
        AgentCmd::Write {
            name,
            file,
            content,
        } => {
            agent::cmd_write(root, &name, &file, &content).unwrap_or_else(|e| {
                eprintln!("[TreeC] {}", e);
                std::process::exit(1);
            });
        }
        AgentCmd::Activate { name } => {
            agent::cmd_activate(root, &name).unwrap_or_else(|e| {
                eprintln!("[TreeC] {}", e);
                std::process::exit(1);
            });
        }
        AgentCmd::List { pending } => {
            agent::cmd_list(root, pending);
        }
        AgentCmd::Status { name } => {
            agent::cmd_status(root, &name);
        }
    }
}

fn handle_orchestrator_cmd(root: &std::path::Path, cmd: OrchestratorCmd) {
    match cmd {
        OrchestratorCmd::Read => {
            let path = root.join(".brain").join("orchestrator").join("tasks.md");
            match std::fs::read_to_string(&path) {
                Ok(content) => println!("{}", content),
                Err(_) => eprintln!(
                    "[TreeC] orchestrator/tasks.md not found. Run 'treec --neural-link' first."
                ),
            }
        }
        OrchestratorCmd::Write { file, content } => {
            let filename = if file.ends_with(".md") {
                file.clone()
            } else {
                format!("{}.md", file)
            };
            brain::write_orchestrator_file(root, &filename, &content).unwrap_or_else(|e| {
                eprintln!("[TreeC] {}", e);
                std::process::exit(1);
            });
            println!("✅ Written orchestrator/{}", filename);
        }
        OrchestratorCmd::Status => {
            let brain_dir = root.join(".brain");
            if !brain_dir.join("orchestrator").exists() {
                eprintln!("❌ Multi-agent brain not initialized. Run 'treec --neural-link'.");
                return;
            }
            let agents = brain::list_agents(root, "_active");
            let pending = brain::list_agents(root, "_pending");
            println!("🎯 Orchestrator Status");
            println!("   Active agents:  {}", agents.len());
            println!("   Pending agents: {}", pending.len());
            // Count tasks
            let tasks_path = brain_dir.join("orchestrator/tasks.md");
            if let Ok(content) = std::fs::read_to_string(&tasks_path) {
                let open = content.lines().filter(|l| l.contains("- [ ]")).count();
                let done = content.lines().filter(|l| l.contains("- [x]")).count();
                println!("   Tasks: {} open, {} done", open, done);
            }
        }
    }
}
