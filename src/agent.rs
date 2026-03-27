use serde::{Deserialize, Serialize};
use std::path::Path;

/// Metadata of an agent — stored as JSON in _pending/ or _active/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentMeta {
    pub name: String,
    pub role: String,
    pub specialties: Vec<String>,
    pub status: AgentStatus,
    pub created_at: String,
    pub prompt_file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Pending,
    Active,
    Paused,
}

#[allow(dead_code)]
impl AgentMeta {
    pub fn new(name: &str, role: &str, specialties: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            role: role.to_string(),
            specialties,
            status: AgentStatus::Pending,
            created_at: chrono::Local::now().to_rfc3339(),
            prompt_file: format!("{}.prompt.md", name),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

/// Validate agent name: must be non-empty, no path separators, no dots, lowercase-ish
fn validate_agent_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Agent name cannot be empty.".to_string());
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err(format!("Invalid agent name '{}': path separators are not allowed.", name));
    }
    Ok(())
}

/// Handle: treec agent scaffold <name>
/// Creates agent directory + seed files. Does NOT require JSON to exist.
pub fn cmd_scaffold(root: &Path, name: &str, role: &str) -> Result<(), String> {
    validate_agent_name(name)?;
    crate::brain::scaffold_agent_dir(root, name, role)?;
    println!("✅ Agent '{}' scaffolded at .brain/agents/{}/", name, name);
    Ok(())
}

/// Handle: treec agent write <name> <file> --content <text>
/// Writes content to a specific file in the agent's brain.
pub fn cmd_write(root: &Path, name: &str, file: &str, content: &str) -> Result<(), String> {
    validate_agent_name(name)?;
    // Normalize filename: add .md if missing
    let filename = if file.ends_with(".md") {
        file.to_string()
    } else {
        format!("{}.md", file)
    };

    crate::brain::write_agent_file(root, name, &filename, content)?;
    println!("✅ Written .brain/agents/{}/{}", name, filename);
    Ok(())
}

/// Handle: treec agent activate <name>
/// Moves agent from _pending/ to _active/ and scaffolds its brain dir.
pub fn cmd_activate(root: &Path, name: &str) -> Result<(), String> {
    validate_agent_name(name)?;
    // Read pending JSON to get role
    let json_path = root
        .join(".brain")
        .join("agents")
        .join("_pending")
        .join(format!("{}.json", name));

    let role = if json_path.exists() {
        let raw = std::fs::read_to_string(&json_path)
            .map_err(|e| format!("Cannot read pending JSON: {}", e))?;
        let meta: AgentMeta = serde_json::from_str(&raw)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        meta.role
    } else {
        "Custom Agent".to_string()
    };

    // Scaffold the brain dir
    crate::brain::scaffold_agent_dir(root, name, &role)?;
    // Move from _pending to _active
    crate::brain::activate_agent(root, name)?;

    println!("✅ Agent '{}' activated — .brain/agents/{}/", name, name);
    Ok(())
}

/// Handle: treec agent list [--pending]
pub fn cmd_list(root: &Path, pending: bool) {
    let subfolder = if pending { "_pending" } else { "_active" };
    let agents = crate::brain::list_agents(root, subfolder);

    if pending {
        println!("⏳ Pending agents ({}):", agents.len());
    } else {
        // Also list named agent dirs (default agents)
        let named = list_named_agents(root);
        println!("🤖 Active agents ({} scaffolded, {} activated):", named.len(), agents.len());
        for a in &named {
            println!("  • {} (scaffolded)", a);
        }
        if !agents.is_empty() {
            println!("  Activated from pending:");
            for a in &agents {
                println!("  • {} (activated)", a);
            }
        }
        return;
    }

    if agents.is_empty() {
        println!("  (none)");
    } else {
        for a in &agents {
            println!("  • {}", a);
        }
    }
}

/// List all agent directories (excludes _pending, _active)
fn list_named_agents(root: &Path) -> Vec<String> {
    let agents_dir = root.join(".brain").join("agents");
    if !agents_dir.exists() {
        return vec![];
    }
    std::fs::read_dir(&agents_dir)
        .ok()
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    if e.path().is_dir() && !name.starts_with('_') {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Handle: treec agent status <name>
pub fn cmd_status(root: &Path, name: &str) {
    if let Err(e) = validate_agent_name(name) {
        eprintln!("❌ {}", e);
        return;
    }
    let agent_dir = root.join(".brain").join("agents").join(name);
    if !agent_dir.exists() {
        eprintln!("❌ Agent '{}' not found in .brain/agents/", name);
        return;
    }

    println!("🤖 Agent: {}", name);

    // Try to read identity for role
    let identity = agent_dir.join("identity.md");
    if identity.exists() {
        if let Ok(content) = std::fs::read_to_string(&identity) {
            let role_line = content.lines()
                .find(|l| l.starts_with("**Role:**"))
                .unwrap_or("**Role:** Unknown");
            println!("   {}", role_line);
        }
    }

    // Show tasks count
    let tasks = agent_dir.join("tasks.md");
    if tasks.exists() {
        if let Ok(content) = std::fs::read_to_string(&tasks) {
            let pending = content.lines().filter(|l| l.contains("- [ ]")).count();
            let done = content.lines().filter(|l| l.contains("- [x]")).count();
            println!("   Tasks: {} pending, {} done", pending, done);
        }
    }

    println!("   Brain: .brain/agents/{}/", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root() -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_path_buf();
        crate::brain::init_brain(&path).unwrap();
        (dir, path)
    }

    #[test]
    fn test_cmd_scaffold() {
        let (_d, root) = temp_root();
        cmd_scaffold(&root, "security", "Security Engineer").unwrap();
        assert!(root.join(".brain/agents/security/identity.md").exists());
    }

    #[test]
    fn test_cmd_write_adds_md_extension() {
        let (_d, root) = temp_root();
        cmd_scaffold(&root, "backend", "Backend Developer").unwrap();
        cmd_write(&root, "backend", "identity", "# Custom").unwrap();
        let content = std::fs::read_to_string(
            root.join(".brain/agents/backend/identity.md")
        ).unwrap();
        assert_eq!(content, "# Custom");
    }

    #[test]
    fn test_cmd_activate() {
        let (_d, root) = temp_root();
        let meta = AgentMeta::new("myagent", "Backend Developer", vec!["Rust".to_string()]);
        crate::brain::save_pending_agent(&root, "myagent", &meta.to_json()).unwrap();
        crate::brain::save_pending_prompt(&root, "myagent", "You are a Rust expert.").unwrap();
        cmd_activate(&root, "myagent").unwrap();
        assert!(root.join(".brain/agents/myagent/identity.md").exists());
        assert!(root.join(".brain/agents/_active/myagent.json").exists());
    }

    #[test]
    fn test_agent_meta_serialization() {
        let meta = AgentMeta::new("test", "QA Engineer", vec!["Jest".to_string()]);
        let json = meta.to_json();
        let parsed: AgentMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.status, AgentStatus::Pending);
    }
}
