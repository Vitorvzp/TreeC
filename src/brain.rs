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
// Multi-Agent Structure
// ═══════════════════════════════════════════════════════════════════

/// Multi-agent subdirectories
const MULTI_AGENT_SUBDIRS: &[&str] = &[
    "orchestrator",
    "agents",
    "agents/_pending",
    "agents/_active",
    "shared_memory",
];

/// Default agent roles seeded on first neural-link
const DEFAULT_AGENTS: &[(&str, &str)] = &[
    ("architect", "Architect"),
    ("backend", "Backend Developer"),
    ("tests", "QA / Test Engineer"),
    ("docs", "Documentation Writer"),
    ("frontend", "Frontend Developer"),
];

/// Files inside each agent brain
const AGENT_BRAIN_FILES: &[&str] = &[
    "identity.md",
    "instructions.md",
    "context.md",
    "tasks.md",
    "memory.md",
    "decisions.md",
    "knowledge.md",
];

/// Orchestrator brain files
const ORCHESTRATOR_FILES: &[&str] = &[
    "orchestrator/identity.md",
    "orchestrator/goals.md",
    "orchestrator/global_context.md",
    "orchestrator/roadmap.md",
    "orchestrator/tasks.md",
    "orchestrator/agent_coordination.md",
    "orchestrator/decisions.md",
];

/// Shared memory files
const SHARED_MEMORY_FILES: &[&str] = &[
    "shared_memory/architecture.md",
    "shared_memory/decisions.md",
    "shared_memory/changelog.md",
    "shared_memory/lessons.md",
    "shared_memory/knowledge.md",
];

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
            fs::write(&path, content)
                .map_err(|e| format!("Failed to create .brain/{}: {}", file, e))?;
        }
    }

    // Initialize multi-agent extension
    init_multi_agent_brain(root)?;

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
    fs::write(&path, existing)
        .map_err(|e| format!("Failed to update .brain/memory/changelog.md: {}", e))
}

/// Initialize the multi-agent extension of .brain/.
/// Safe to call on a brain already initialized with init_brain().
/// Creates orchestrator/, agents/*, shared_memory/ if not present.
pub fn init_multi_agent_brain(root: &Path) -> Result<(), String> {
    let brain_dir = root.join(".brain");

    // Create multi-agent subdirs
    for subdir in MULTI_AGENT_SUBDIRS {
        let dir = brain_dir.join(subdir);
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create .brain/{}/: {}", subdir, e))?;
    }

    // Seed orchestrator files
    for file in ORCHESTRATOR_FILES {
        let path = brain_dir.join(file);
        if !path.exists() {
            fs::write(&path, seed_orchestrator(file))
                .map_err(|e| format!("Failed to seed .brain/{}: {}", file, e))?;
        }
    }

    // Seed shared_memory files
    for file in SHARED_MEMORY_FILES {
        let path = brain_dir.join(file);
        if !path.exists() {
            fs::write(&path, seed_shared_memory(file))
                .map_err(|e| format!("Failed to seed .brain/{}: {}", file, e))?;
        }
    }

    // Seed default agents
    for (agent_name, agent_role) in DEFAULT_AGENTS {
        scaffold_agent_dir(root, agent_name, agent_role)?;
    }

    Ok(())
}

/// Create the directory and seed files for a single agent.
/// Safe to call if the agent already exists (skips existing files).
pub fn scaffold_agent_dir(root: &Path, name: &str, role: &str) -> Result<(), String> {
    let agent_dir = root.join(".brain").join("agents").join(name);
    fs::create_dir_all(&agent_dir)
        .map_err(|e| format!("Failed to create agent dir '{}': {}", name, e))?;

    for file in AGENT_BRAIN_FILES {
        let path = agent_dir.join(file);
        if !path.exists() {
            fs::write(&path, seed_agent_file(name, role, file))
                .map_err(|e| format!("Failed to seed agent file {}/{}: {}", name, file, e))?;
        }
    }
    Ok(())
}

/// Write content to a specific file inside an agent's brain.
/// Creates parent dirs if needed. Overwrites existing content.
#[allow(dead_code)]
pub fn write_agent_file(
    root: &Path,
    agent_name: &str,
    filename: &str,
    content: &str,
) -> Result<(), String> {
    let path = root
        .join(".brain")
        .join("agents")
        .join(agent_name)
        .join(filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create dirs: {}", e))?;
    }

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write {}/{}: {}", agent_name, filename, e))
}

/// Write content to a specific orchestrator file.
#[allow(dead_code)]
pub fn write_orchestrator_file(root: &Path, filename: &str, content: &str) -> Result<(), String> {
    let path = root.join(".brain").join("orchestrator").join(filename);
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write orchestrator/{}: {}", filename, e))
}

/// Save a pending agent JSON to agents/_pending/<name>.json
#[allow(dead_code)]
pub fn save_pending_agent(root: &Path, name: &str, json: &str) -> Result<(), String> {
    let path = root
        .join(".brain")
        .join("agents")
        .join("_pending")
        .join(format!("{}.json", name));
    fs::write(&path, json).map_err(|e| format!("Failed to save pending agent '{}': {}", name, e))
}

/// Save a pending agent prompt to agents/_pending/<name>.prompt.md
#[allow(dead_code)]
pub fn save_pending_prompt(root: &Path, name: &str, prompt: &str) -> Result<(), String> {
    let path = root
        .join(".brain")
        .join("agents")
        .join("_pending")
        .join(format!("{}.prompt.md", name));
    fs::write(&path, prompt).map_err(|e| format!("Failed to save pending prompt '{}': {}", name, e))
}

/// Move agent from _pending/ to _active/ (updates status in JSON).
#[allow(dead_code)]
pub fn activate_agent(root: &Path, name: &str) -> Result<(), String> {
    let pending_dir = root.join(".brain").join("agents").join("_pending");
    let active_dir = root.join(".brain").join("agents").join("_active");

    let json_src = pending_dir.join(format!("{}.json", name));
    let prompt_src = pending_dir.join(format!("{}.prompt.md", name));

    if !json_src.exists() {
        return Err(format!("No pending agent named '{}'", name));
    }

    // Update status field in JSON
    let raw =
        fs::read_to_string(&json_src).map_err(|e| format!("Failed to read pending JSON: {}", e))?;
    let updated = raw
        .replace("\"status\": \"pending\"", "\"status\": \"active\"")
        .replace("\"status\":\"pending\"", "\"status\":\"active\"");

    fs::write(active_dir.join(format!("{}.json", name)), &updated)
        .map_err(|e| format!("Failed to write active JSON: {}", e))?;

    if prompt_src.exists() {
        fs::copy(&prompt_src, active_dir.join(format!("{}.prompt.md", name)))
            .map_err(|e| format!("Failed to copy prompt: {}", e))?;
        fs::remove_file(&prompt_src).ok();
    }

    fs::remove_file(&json_src).ok();
    Ok(())
}

/// List all agents in a given state folder ("_pending" or "_active" or an agent name).
#[allow(dead_code)]
pub fn list_agents(root: &Path, subfolder: &str) -> Vec<String> {
    let dir = root.join(".brain").join("agents").join(subfolder);
    if !dir.exists() {
        return vec![];
    }
    fs::read_dir(&dir)
        .ok()
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    if name.ends_with(".json") {
                        Some(name.trim_end_matches(".json").to_string())
                    } else if e.path().is_dir() {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
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

fn seed_orchestrator(filename: &str) -> String {
    match filename {
        "orchestrator/identity.md" => "# 🎯 Orchestrator — Identity\n\nVocê é o Orquestrador do sistema multi-agente TreeC.\nSua função: ler o estado do projeto, priorizar tarefas e delegar para os agentes especializados.\n\nLeia sempre:\n- [[orchestrator/global_context]]\n- [[orchestrator/roadmap]]\n- [[orchestrator/tasks]]\n- [[orchestrator/agent_coordination]]\n".to_string(),
        "orchestrator/goals.md" => "# 🏆 Goals\n\n> Objetivos globais do projeto — atualizados pelo usuário ou pela skill.\n\n".to_string(),
        "orchestrator/global_context.md" => "# 🌍 Global Context\n\n> Estado atual do projeto. Atualizado pelo Orchestrator após cada ciclo.\n\n".to_string(),
        "orchestrator/roadmap.md" => "# 🗺️ Roadmap\n\n> Planejamento futuro gerenciado pelo Orchestrator.\n\n".to_string(),
        "orchestrator/tasks.md" => "# ✅ Task Queue\n\n> Fila de tarefas global. O Orchestrator lê, prioriza e delega.\n\n## Formato\n```\n- [ ] [AGENT:backend] Descrição da tarefa | Prioridade: Alta\n```\n\n## Tasks\n\n".to_string(),
        "orchestrator/agent_coordination.md" => "# 🤝 Agent Coordination\n\n> Status de cada agente e regras de delegação.\n\n## Agentes Ativos\n\n| Agente | Status | Última Tarefa |\n|---|---|---|\n\n".to_string(),
        "orchestrator/decisions.md" => "# 🔀 High-Level Decisions\n\n> Decisões de alto nível tomadas pelo Orchestrator.\n\n".to_string(),
        _ => format!("# {}\n\n> Orquestrador — auto-gerado pelo TreeC\n\n", filename),
    }
}

fn seed_shared_memory(filename: &str) -> String {
    match filename {
        "shared_memory/architecture.md" => "# 🏗️ Shared Architecture\n\n> Arquitetura consolidada — todos os agentes leem e respeitam este documento.\n\n".to_string(),
        "shared_memory/decisions.md" => "# 🔀 Architecture Decision Records (ADRs)\n\n> Decisões técnicas consolidadas de todos os agentes.\n\n## Formato\n```\n## ADR-001 — Título\n- Decisão: ...\n- Razão: ...\n- Impacto: ...\n```\n\n".to_string(),
        "shared_memory/changelog.md" => format!("# 📋 Global Changelog\n\n## {}\n- Sistema multi-agente inicializado pelo TreeC\n", chrono::Local::now().format("%Y-%m-%d")),
        "shared_memory/lessons.md" => "# 💡 Shared Lessons Learned\n\n> Lições aprendidas por qualquer agente — compartilhadas com todos.\n\n".to_string(),
        "shared_memory/knowledge.md" => "# 📚 Shared Knowledge Base\n\n> Base de conhecimento técnico compartilhada por todos os agentes.\n\n".to_string(),
        _ => format!("# {}\n\n> Memória compartilhada — auto-gerada pelo TreeC\n\n", filename),
    }
}

fn seed_agent_file(agent_name: &str, role: &str, filename: &str) -> String {
    match filename {
        "identity.md" => format!(
            "# 🤖 {name} — Identity\n\n**Role:** {role}\n\n> Identidade gerada como seed pelo TreeC.\n> A skill deve reescrever este arquivo com `treec agent write {name} identity --content \"...\"`\n\nAntes de agir, leia:\n- [[agents/{name}/instructions]]\n- [[agents/{name}/tasks]]\n- [[agents/{name}/memory]]\n- [[shared_memory/architecture]]\n",
            name = agent_name, role = role
        ),
        "instructions.md" => format!(
            "# 📏 {name} — Instructions\n\n> Regras de operação deste agente.\n> A skill deve preencher via `treec agent write {name} instructions --content \"...\"`\n\n",
            name = agent_name
        ),
        "context.md" => format!("# 📄 {name} — Current Context\n\n> Contexto da tarefa em execução. Atualizado pelo agente a cada ciclo.\n\n", name = agent_name),
        "tasks.md" => format!("# ✅ {name} — Task Queue\n\n> Tarefas delegadas pelo Orchestrator.\n\n## Formato\n```\n- [ ] Descrição da tarefa | Prioridade: Alta\n```\n\n## Pendentes\n\n", name = agent_name),
        "memory.md" => format!("# 🧠 {name} — Memory\n\n> Memória de curto e longo prazo deste agente.\n\n## Entrada — {date}\n- Agente inicializado pelo TreeC\n", name = agent_name, date = chrono::Local::now().format("%Y-%m-%d")),
        "decisions.md" => format!("# 🔀 {name} — Decisions\n\n> Decisões técnicas tomadas por este agente.\n\n", name = agent_name),
        "knowledge.md" => format!("# 📚 {name} — Domain Knowledge\n\n> Conhecimento específico do domínio deste agente.\n> A skill deve preencher via `treec agent write {name} knowledge --content \"...\"`\n\n", name = agent_name),
        _ => format!("# {} — {}\n\n> Auto-gerado pelo TreeC\n\n", agent_name, filename),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_path_buf();
        (dir, path)
    }

    #[test]
    fn test_init_multi_agent_creates_orchestrator() {
        let (_dir, root) = temp_dir();
        init_brain(&root).unwrap();
        assert!(root.join(".brain/orchestrator/tasks.md").exists());
        assert!(root.join(".brain/orchestrator/identity.md").exists());
        assert!(root.join(".brain/shared_memory/changelog.md").exists());
    }

    #[test]
    fn test_default_agents_created() {
        let (_dir, root) = temp_dir();
        init_brain(&root).unwrap();
        for (name, _) in DEFAULT_AGENTS {
            assert!(
                root.join(format!(".brain/agents/{}/identity.md", name))
                    .exists(),
                "Missing agent: {}",
                name
            );
        }
    }

    #[test]
    fn test_scaffold_agent_dir() {
        let (_dir, root) = temp_dir();
        fs::create_dir_all(root.join(".brain/agents")).unwrap();
        scaffold_agent_dir(&root, "security", "Security Engineer").unwrap();
        assert!(root.join(".brain/agents/security/identity.md").exists());
        assert!(root.join(".brain/agents/security/tasks.md").exists());
    }

    #[test]
    fn test_write_agent_file() {
        let (_dir, root) = temp_dir();
        init_brain(&root).unwrap();
        write_agent_file(&root, "backend", "identity.md", "# Custom Identity").unwrap();
        let content = fs::read_to_string(root.join(".brain/agents/backend/identity.md")).unwrap();
        assert_eq!(content, "# Custom Identity");
    }

    #[test]
    fn test_save_and_activate_agent() {
        let (_dir, root) = temp_dir();
        init_brain(&root).unwrap();
        // Use the same format serde_json::to_string_pretty produces
        let json = "{\n  \"name\": \"myagent\",\n  \"status\": \"pending\"\n}";
        save_pending_agent(&root, "myagent", json).unwrap();
        save_pending_prompt(&root, "myagent", "You are a specialist.").unwrap();
        assert!(root.join(".brain/agents/_pending/myagent.json").exists());
        activate_agent(&root, "myagent").unwrap();
        assert!(root.join(".brain/agents/_active/myagent.json").exists());
        assert!(!root.join(".brain/agents/_pending/myagent.json").exists());
        // Verify status was actually updated
        let content = fs::read_to_string(root.join(".brain/agents/_active/myagent.json")).unwrap();
        assert!(
            content.contains("\"active\""),
            "Status should be 'active' in the JSON"
        );
    }

    #[test]
    fn test_list_agents() {
        let (_dir, root) = temp_dir();
        init_brain(&root).unwrap();
        let active = list_agents(&root, "_active");
        // No agents in _active yet (default agents go directly to named dirs)
        assert!(active.is_empty());
    }
}
