use std::fs;
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════
// Brain Directory Structure
// ═══════════════════════════════════════════════════════════════════

/// Subdirectories to create inside .brain/
const BRAIN_SUBDIRS: &[&str] = &[
    "cortex",
    "cortex/knowledge",
    "memory",
    "perception",
    "motor",
    "language",
    "identity",
    "system",
];

/// Root-level brain files (index + prompt)
const ROOT_FILES: &[&str] = &["index.md", "prompt.md"];

/// cortex/ files (thinking, decisions, architecture)
const CORTEX_FILES: &[&str] = &[
    "cortex/context.md",
    "cortex/architecture.md",
    "cortex/decisions.md",
    "cortex/roadmap.md",
    "cortex/patterns.md",
    "cortex/releases.md",
];

/// cortex/knowledge/ files (project internals)
const KNOWLEDGE_FILES: &[&str] = &[
    "cortex/knowledge/modules.md",
    "cortex/knowledge/functions.md",
    "cortex/knowledge/api.md",
    "cortex/knowledge/database.md",
    "cortex/knowledge/models.md",
    "cortex/knowledge/services.md",
];

/// memory/ files (history, changelog, lessons)
const MEMORY_FILES: &[&str] = &[
    "memory/long_term.md",
    "memory/short_term.md",
    "memory/changelog.md",
    "memory/lessons.md",
];

/// perception/ files (project structure, deps)
const PERCEPTION_FILES: &[&str] = &[
    "perception/tree.md",
    "perception/dependencies.md",
    "perception/files_summary.md",
];

/// motor/ files (tasks, backlog, bugs)
const MOTOR_FILES: &[&str] = &[
    "motor/tasks.md",
    "motor/backlog.md",
    "motor/bugs.md",
    "motor/issues.md",
];

/// language/ files (docs, readme)
const LANGUAGE_FILES: &[&str] = &["language/readme.md", "language/documentation.md"];

/// identity/ files (project identity)
const IDENTITY_FILES: &[&str] = &["identity/project.md", "identity/goals.md"];

/// system/ files (AI rules, workflow)
const SYSTEM_FILES: &[&str] = &["system/rules.md", "system/workflow.md"];

// ═══════════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════════

/// Initialize the .brain/ directory with the hierarchical Neural Brain structure.
///
/// Creates all subdirectories and seeds each file with initial content.
/// Skips files that already exist (safe to re-run).
pub fn init_brain(root: &Path) -> Result<(), String> {
    let brain_dir = root.join(".brain");

    // Create all subdirectories
    for subdir in BRAIN_SUBDIRS {
        let dir = brain_dir.join(subdir);
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create .brain/{}/: {}", subdir, e))?;
    }

    // Seed all file groups
    let all_files = ROOT_FILES
        .iter()
        .chain(CORTEX_FILES.iter())
        .chain(KNOWLEDGE_FILES.iter())
        .chain(MEMORY_FILES.iter())
        .chain(PERCEPTION_FILES.iter())
        .chain(MOTOR_FILES.iter())
        .chain(LANGUAGE_FILES.iter())
        .chain(IDENTITY_FILES.iter())
        .chain(SYSTEM_FILES.iter());

    for file in all_files {
        let path = brain_dir.join(file);
        if !path.exists() {
            let content = seed_content(file);
            fs::write(&path, &content)
                .map_err(|e| format!("Failed to create .brain/{}: {}", file, e))?;
        }
    }

    Ok(())
}

/// Update perception/tree.md with the latest project scan output.
pub fn update_tree(root: &Path, tree_content: &str) -> Result<(), String> {
    let path = root.join(".brain").join("perception").join("tree.md");
    fs::write(&path, tree_content)
        .map_err(|e| format!("Failed to write .brain/perception/tree.md: {}", e))
}

/// Update perception/dependencies.md with detected project dependencies.
pub fn update_dependencies(root: &Path, content: &str) -> Result<(), String> {
    let path = root
        .join(".brain")
        .join("perception")
        .join("dependencies.md");
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write .brain/perception/dependencies.md: {}", e))
}

/// Write AI-generated content to a specific brain file path (relative to .brain/).
pub fn update_brain_file(root: &Path, filename: &str, content: &str) -> Result<(), String> {
    let path = root.join(".brain").join(filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }

    fs::write(&path, content).map_err(|e| format!("Failed to write .brain/{}: {}", filename, e))
}

/// Append a timestamped entry to memory/long_term.md.
pub fn append_memory(root: &Path, entry: &str) -> Result<(), String> {
    let path = root.join(".brain").join("memory").join("long_term.md");
    let mut existing = fs::read_to_string(&path).unwrap_or_default();
    existing.push_str(entry);
    fs::write(&path, &existing)
        .map_err(|e| format!("Failed to update .brain/memory/long_term.md: {}", e))
}

/// Append a timestamped entry to memory/changelog.md.
pub fn append_changelog(root: &Path, entry: &str) -> Result<(), String> {
    let path = root.join(".brain").join("memory").join("changelog.md");
    let mut existing = fs::read_to_string(&path).unwrap_or_default();
    existing.push_str(entry);
    fs::write(&path, &existing)
        .map_err(|e| format!("Failed to update .brain/memory/changelog.md: {}", e))
}

// ═══════════════════════════════════════════════════════════════════
// Seed Content
// ═══════════════════════════════════════════════════════════════════

fn seed_content(filename: &str) -> String {
    match filename {
        // ── Root ──
        "index.md" => r#"# 🧠 Neural Brain — Index

> Auto-generated by TreeC Neural Link

## Cortex (Thinking & Architecture)

- [[cortex/context]] — Project overview & tech stack
- [[cortex/architecture]] — System architecture & design
- [[cortex/decisions]] — Technical decisions log
- [[cortex/roadmap]] — Future plans & features
- [[cortex/patterns]] — Design patterns used
- [[cortex/releases]] — Release history

## Cortex / Knowledge Base

- [[cortex/knowledge/modules]] — Module documentation
- [[cortex/knowledge/functions]] — Function reference
- [[cortex/knowledge/api]] — API documentation
- [[cortex/knowledge/database]] — Database schema
- [[cortex/knowledge/models]] — Data models
- [[cortex/knowledge/services]] — Service documentation

## Memory (History & Context)

- [[memory/long_term]] — Long-term project memory
- [[memory/short_term]] — Current session context
- [[memory/changelog]] — Change tracking log
- [[memory/lessons]] — Lessons learned

## Perception (Project Structure)

- [[perception/tree]] — File structure
- [[perception/dependencies]] — Project dependencies
- [[perception/files_summary]] — File summaries

## Motor (Tasks & Actions)

- [[motor/tasks]] — Active task list
- [[motor/backlog]] — Backlog
- [[motor/bugs]] — Known bugs
- [[motor/issues]] — Open issues

## Language (Documentation)

- [[language/readme]] — AI-generated README
- [[language/documentation]] — Full documentation

## Identity

- [[identity/project]] — Project identity & mission
- [[identity/goals]] — Project goals

## System (AI Rules)

- [[system/rules]] — AI agent rules
- [[system/workflow]] — AI workflow
"#
        .to_string(),

        "prompt.md" => AGENT_PROMPT.to_string(),

        // ── Cortex ──
        "cortex/context.md" => {
            "# 📄 Context\n\n> Project overview — populated by Neural Link AI.\n\n"
                .to_string()
        }
        "cortex/architecture.md" => {
            "# 🏗️ Architecture\n\n> System architecture — populated by Neural Link AI.\n\n"
                .to_string()
        }
        "cortex/decisions.md" => {
            "# 🔀 Technical Decisions\n\n> Register all technical decisions here.\n\n".to_string()
        }
        "cortex/roadmap.md" => {
            "# 🗺️ Roadmap\n\n> Future features and improvements.\n\n".to_string()
        }
        "cortex/patterns.md" => {
            "# 🔷 Design Patterns\n\n> Design patterns and architectural patterns used in the project.\n\n"
                .to_string()
        }
        "cortex/releases.md" => {
            "# 🚀 Releases\n\n> Release history and planned releases.\n\n".to_string()
        }

        // ── Cortex / Knowledge ──
        f if f.starts_with("cortex/knowledge/") => {
            let name = f
                .trim_start_matches("cortex/knowledge/")
                .trim_end_matches(".md");
            format!(
                "# {}\n\n> Auto-generated by TreeC Neural Link\n\n",
                capitalize(name)
            )
        }

        // ── Memory ──
        "memory/long_term.md" => format!(
            "# 🧠 Long-term Memory\n\n> Never delete entries — only append.\n\n## Entry — {}\n- Brain initialized by TreeC Neural Link\n",
            chrono::Local::now().format("%Y-%m-%d")
        ),
        "memory/short_term.md" => {
            "# ⚡ Short-term Memory\n\n> Current session context. Reset on each scan.\n\n"
                .to_string()
        }
        "memory/changelog.md" => format!(
            "# 📋 Changelog\n\n## Change — {}\n- File: .brain/*\n- Description: Brain initialized by TreeC Neural Link\n- Reason: First neural link execution\n- Risk: None\n- Status: Complete\n",
            chrono::Local::now().format("%Y-%m-%d")
        ),
        "memory/lessons.md" => {
            "# 💡 Lessons Learned\n\n> Key insights and lessons from the project evolution.\n\n"
                .to_string()
        }

        // ── Perception ──
        "perception/tree.md" => {
            "# 🌳 Project Tree\n\n> File structure — updated by TreeC on each scan.\n\n"
                .to_string()
        }
        "perception/dependencies.md" => {
            "# 📦 Dependencies\n\n> Project dependencies — detected by TreeC.\n\n".to_string()
        }
        "perception/files_summary.md" => {
            "# 📄 Files Summary\n\n> Summary of project files — populated by Neural Link AI.\n\n"
                .to_string()
        }

        // ── Motor ──
        "motor/tasks.md" => "# ✅ Tasks\n\n> Active task list.\n\n".to_string(),
        "motor/backlog.md" => "# 📋 Backlog\n\n> Planned future work.\n\n".to_string(),
        "motor/bugs.md" => "# 🐛 Bugs\n\n> Known bugs and their status.\n\n".to_string(),
        "motor/issues.md" => "# ⚠️ Issues\n\n> Open issues and blockers.\n\n".to_string(),

        // ── Language ──
        "language/readme.md" => {
            "# 📖 README\n\n> AI-generated README — populated by Neural Link AI.\n\n".to_string()
        }
        "language/documentation.md" => {
            "# 📚 Documentation\n\n> Full project documentation — populated by Neural Link AI.\n\n"
                .to_string()
        }

        // ── Identity ──
        "identity/project.md" => {
            "# 🎯 Project Identity\n\n> What this project is, its mission, and its purpose.\n\n"
                .to_string()
        }
        "identity/goals.md" => {
            "# 🏆 Goals\n\n> Project goals, success criteria, and north star.\n\n".to_string()
        }

        // ── System ──
        "system/rules.md" => {
            "# 📏 AI Rules\n\n> Rules the AI agent must follow when working on this project.\n\nSee [[prompt]] for the full agent prompt.\n\n".to_string()
        }
        "system/workflow.md" => {
            "# 🔄 AI Workflow\n\n> Standard workflow for AI-assisted development on this project.\n\n\
            ## Steps\n1. Read [[index]]\n2. Read [[cortex/context]]\n3. Read [[cortex/architecture]]\n\
            4. Read [[motor/tasks]]\n5. Read [[memory/long_term]]\n6. Plan changes\n7. Implement\n\
            8. Update [[memory/changelog]]\n9. Update [[memory/long_term]]\n\n"
                .to_string()
        }

        _ => format!(
            "# {}\n\n> Auto-generated by TreeC Neural Link\n\n",
            title_from_path(filename)
        ),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn title_from_path(filename: &str) -> String {
    filename
        .split('/')
        .next_back()
        .unwrap_or(filename)
        .trim_end_matches(".md")
        .split('_')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ═══════════════════════════════════════════════════════════════════
// Agent Prompt
// ═══════════════════════════════════════════════════════════════════

const AGENT_PROMPT: &str = r#"# TreeC Neural Brain — Agent Prompt

## Identity
You are an AI software engineer integrated into this project through TreeC Neural Link.
You act as a persistent software agent with memory and full project awareness.

---

# Before ANY code modification, you MUST read:

1. [[index]] — Brain overview
2. [[cortex/context]] — Project overview
3. [[cortex/architecture]] — Architecture
4. [[memory/long_term]] — Project history
5. [[cortex/roadmap]] — Planned work
6. [[cortex/decisions]] — Past decisions
7. [[memory/changelog]] — Recent changes
8. [[motor/tasks]] — Active tasks

---

# Memory Rules

## memory/long_term.md
Append entries when:
- Feature added / architecture changed / refactor done
- Bug fixed / dependency added / key insight discovered

Format:
```
## Entry — YYYY-MM-DD
- What changed
- Why it changed
- Impact on system
```

Never delete entries. Only append.

## memory/changelog.md
Every code modification must be logged.

Format:
```
## Change — YYYY-MM-DD
- File: <filename>
- Description: <what changed>
- Reason: <why>
- Risk: <low|medium|high>
- Status: Complete
```

---

# Decisions Log

Register all technical decisions in [[cortex/decisions]].

Format:
```
## Decision — YYYY-MM-DD
- Decision: <what was decided>
- Reason: <why>
- Alternatives considered: <options>
- Impact: <effect on system>
```

---

# Task Management

Use [[motor/tasks]] for the active task list.
Use [[motor/backlog]] for future work.
Use [[motor/bugs]] for known bugs.
Use [[motor/issues]] for blockers.

---

# Behavior Rules

- Prefer refactoring over rewriting
- Prefer simple solutions
- Avoid unnecessary dependencies
- Maintain consistent architecture
- Document everything
- Think before coding
- Use [[cortex/context]], [[cortex/architecture]] before suggesting changes
- Update [[memory/changelog]] and [[memory/long_term]] after every change

---

# Goal

You are the project's second brain.
Your job: understand, maintain memory, document decisions, and safely improve the codebase.
"#;
