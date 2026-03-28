<!-- 🇧🇷 Português -->

# Contribuindo com o TreeC

Obrigado pelo seu interesse em contribuir com o TreeC. Este guia cobre tudo que você precisa para começar — desde configurar o ambiente de desenvolvimento até adicionar novos agentes ou comandos CLI.

---

## Índice

- [Ambiente de Desenvolvimento](#ambiente-de-desenvolvimento)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Executando Testes](#executando-testes)
- [Qualidade de Código](#qualidade-de-código)
- [Convenção de Commits](#convenção-de-commits)
- [Adicionando um Novo Comando CLI](#adicionando-um-novo-comando-cli)
- [Adicionando um Novo Agente](#adicionando-um-novo-agente)
- [Adicionando um Novo Provedor de IA](#adicionando-um-novo-provedor-de-ia)
- [Diretrizes para Pull Requests](#diretrizes-para-pull-requests)

---

## Ambiente de Desenvolvimento

### Pré-requisitos

- [Rust](https://rustup.rs/) (stable, edition 2021) — `rustup update stable`
- Git

### Configuração

```bash
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo build
```

Execute a CLI a partir do código-fonte durante o desenvolvimento:

```bash
cargo run -- --help
cargo run -- --status
cargo run -- tui
cargo run -- agent list
```

---

## Estrutura do Projeto

```
src/
├── main.rs          # Entry point — parsing da CLI (clap 4.4 derive) e dispatch
├── scanner.rs       # Walker do filesystem com semântica gitignore (ignore 0.4)
├── analyzer.rs      # Detecção de linguagem, contagem de linhas, detecção de binários
├── generator.rs     # Gera Tree.md, Structure.json, Structure.txt
├── brain.rs         # Gerenciamento do .brain/ — init, scaffold, write, activate
├── agent.rs         # Handlers do comando treec agent *
├── neural.rs        # Integração com IA via ureq (Gemini/OpenAI/Claude/Ollama)
├── config.rs        # Parsing do TreeC.toml + env TREEC_API_KEY + keyring
└── tui/
    ├── mod.rs       # Entry point da TUI e loop principal
    ├── app.rs       # Estado da aplicação e tratamento de eventos
    ├── screens.rs   # Todos os renderers de tela (Dashboard, Agents, Tasks, etc.)
    └── wizard.rs    # Wizard de criação de agente em 5 etapas
```

---

## Executando Testes

```bash
# Executar todos os testes
cargo test

# Executar testes de um módulo específico
cargo test --lib scanner
cargo test --lib agent

# Executar com saída (útil para debug)
cargo test -- --nocapture
```

Os testes ficam em blocos `#[cfg(test)]` dentro de cada módulo. Testes de integração que tocam o filesystem usam o crate `tempfile` para criar diretórios temporários isolados — nunca escreva no filesystem real em testes.

Exemplo de estrutura de teste:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_scaffold_creates_files() {
        let dir = TempDir::new().unwrap();
        cmd_scaffold(dir.path(), "my-agent", "Test Role").unwrap();
        assert!(dir.path().join(".brain/agents/my-agent/identity.md").exists());
    }
}
```

---

## Qualidade de Código

Todo código deve passar por estas verificações antes do commit:

```bash
# Formatar o código
cargo fmt

# Lint — tratar todos os warnings como erros
cargo clippy -- -D warnings

# Executar testes
cargo test
```

Checklist pré-commit:

- [ ] `cargo fmt` aplicado
- [ ] `cargo clippy -- -D warnings` passa sem nenhum warning
- [ ] `cargo test` passa
- [ ] Nenhum `unwrap()` em `Result`/`Option` em caminhos de produção — use `?` ou retorne `Result<T, String>`
- [ ] Nenhum `panic!` em caminhos de produção

---

## Convenção de Commits

O TreeC usa [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <descrição curta>

[corpo opcional]

[rodapé opcional]
```

### Tipos

| Tipo | Quando usar |
|---|---|
| `feat` | Nova feature ou capacidade |
| `fix` | Correção de bug |
| `refactor` | Alteração de código que não é feature nem fix |
| `style` | Mudanças de formatação (cargo fmt, espaços) |
| `test` | Adição ou atualização de testes |
| `docs` | Somente alterações de documentação |
| `chore` | Bump de dependências, config de CI, build scripts |
| `perf` | Melhorias de performance |

### Escopos

Use o nome do módulo como escopo quando aplicável: `scanner`, `neural`, `agent`, `brain`, `tui`, `config`, `generator`.

### Exemplos

```
feat(agent): add treec agent pause command
fix(neural): handle 429 rate-limit with exponential backoff
docs: update README with multi-agent platform section
test(brain): add coverage for scaffold_agent_dir
chore: bump ratatui to 0.30
style: apply cargo fmt across tui module
refactor(config): extract keyring logic into config::keyring submodule
```

---

## Adicionando um Novo Comando CLI

### 1. Defina o argumento em `src/main.rs`

Para uma flag simples:

```rust
/// Descrição da nova flag
#[arg(long = "my-flag")]
my_flag: bool,
```

Para um subcomando, adicione uma variante na `enum` relevante:

```rust
// Em AgentCmd:
/// Descrição curta exibida no --help
MyCommand {
    name: String,
    #[arg(long)]
    option: String,
},
```

### 2. Implemente o handler

Adicione a lógica de negócio no módulo relevante (`agent.rs`, `brain.rs`, etc.):

```rust
pub fn cmd_my_command(root: &Path, name: &str, option: &str) -> Result<(), String> {
    // implementação
    println!("Done.");
    Ok(())
}
```

### 3. Faça o dispatch em `main()`

Conecte a nova variante no bloco `match` dentro de `main()`:

```rust
AgentCmd::MyCommand { name, option } => {
    if let Err(e) = agent::cmd_my_command(root, &name, &option) {
        eprintln!("[TreeC] Error: {}", e);
        std::process::exit(1);
    }
}
```

### 4. Atualize a string long_about e o README

Adicione o novo comando na string `long_about` no atributo `#[command(...)]` e na referência CLI do `README.md`.

---

## Adicionando um Novo Agente

Agentes são cidadãos de primeira classe no TreeC. Você pode adicionar um via CLI ou editando os arquivos do brain diretamente.

### Via CLI (recomendado)

```bash
# Crie o diretório do agente com arquivos iniciais
treec agent scaffold my-agent --role "Descrição do que este agente faz"

# Os seguintes arquivos são criados:
# .brain/agents/my-agent/identity.md
# .brain/agents/my-agent/instructions.md
# .brain/agents/my-agent/knowledge.md
# .brain/agents/my-agent/tasks.md
# .brain/agents/my-agent/memory.md

# Popule os arquivos conforme necessário
treec agent write my-agent instructions --content "## Como agir\n- Foque em X\n- Evite Y"
treec agent write my-agent knowledge --content "## Conhecimento de domínio\n..."

# Ative o agente
treec agent activate my-agent
```

### Via código

Se quiser que um agente seja sempre criado quando `treec --neural-link` roda, adicione-o à lista de agentes padrão em `src/brain.rs`. Procure pela função que cria o conjunto padrão de agentes e adicione uma entrada:

```rust
scaffold_agent_dir(root, "my-agent", "My Agent Role")?;
```

### Convenções dos arquivos do agente

| Arquivo | Conteúdo |
|---|---|
| `identity.md` | Quem é o agente, responsabilidades, limites de domínio |
| `instructions.md` | Como agir, comportamento passo a passo, restrições |
| `knowledge.md` | Conhecimento de domínio, padrões, referências |
| `tasks.md` | Tarefas delegadas pelo orquestrador (formato checklist) |
| `memory.md` | Histórico acumulado, decisões, contexto |

Todas as escritas em `.brain/` devem passar por `treec agent write` ou `treec orchestrator write` — nunca edite arquivos diretamente em produção.

---

## Adicionando um Novo Provedor de IA

Os provedores são implementados em `src/neural.rs`.

### 1. Adicione a variante do provedor

Na lógica de parsing do provedor, adicione o reconhecimento do novo nome:

```rust
"myprovider" | "alias" => Provider::MyProvider,
```

### 2. Implemente a requisição

Adicione um match arm que constrói a requisição HTTP para o novo provedor:

```rust
Provider::MyProvider => {
    let url = format!("https://api.myprovider.com/v1/...");
    ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(/* request body */)?
}
```

### 3. Faça o parsing da resposta

Adicione o parsing da resposta no formato JSON do provedor.

### 4. Atualize a documentação

Adicione o provedor na tabela do `README.md` na seção **Neural Link**.

---

## Diretrizes para Pull Requests

1. Faça um fork do repositório e crie um branch a partir de `main`:
   ```bash
   git checkout -b feat/minha-feature
   ```

2. Faça suas alterações seguindo as convenções deste guia.

3. Garanta que todas as verificações passam:
   ```bash
   cargo fmt && cargo clippy -- -D warnings && cargo test
   ```

4. Faça o commit com uma mensagem no padrão de conventional commits.

5. Abra um pull request contra `main` com:
   - Um título claro seguindo a convenção de commits
   - Uma descrição do que mudou e por quê
   - Referência a issues relacionadas

6. Um mantenedor vai revisar e dar feedback.

---

## Dúvidas

Abra uma issue no [GitHub](https://github.com/Vitorvzp/TreeC/issues) ou consulte o diretório `.brain/` do repositório para notas detalhadas sobre arquitetura.

---

<!-- 🇺🇸 English -->

# Contributing to TreeC

Thank you for your interest in contributing to TreeC. This guide covers everything you need to get started — from setting up a development environment to adding new agents or CLI commands.

---

## Table of Contents

- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Running Tests](#running-tests)
- [Code Quality](#code-quality)
- [Commit Convention](#commit-convention)
- [Adding a New CLI Command](#adding-a-new-cli-command)
- [Adding a New Agent](#adding-a-new-agent)
- [Adding a New AI Provider](#adding-a-new-ai-provider)
- [Pull Request Guidelines](#pull-request-guidelines)

---

## Development Environment

### Prerequisites

- [Rust](https://rustup.rs/) (stable, edition 2021) — `rustup update stable`
- Git

### Setup

```bash
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo build
```

Run the CLI from source during development:

```bash
cargo run -- --help
cargo run -- --status
cargo run -- tui
cargo run -- agent list
```

---

## Project Structure

```
src/
├── main.rs          # Entry point — CLI parsing (clap 4.4 derive) and dispatch
├── scanner.rs       # Filesystem walker with gitignore semantics (ignore 0.4)
├── analyzer.rs      # Language detection, line counting, binary detection
├── generator.rs     # Generates Tree.md, Structure.json, Structure.txt
├── brain.rs         # .brain/ management — init, scaffold, write, activate
├── agent.rs         # treec agent * command handlers
├── neural.rs        # AI integration via ureq (Gemini/OpenAI/Claude/Ollama)
├── config.rs        # TreeC.toml parsing + TREEC_API_KEY env + keyring
└── tui/
    ├── mod.rs       # TUI entry point and run loop
    ├── app.rs       # Application state and event handling
    ├── screens.rs   # All screen renderers (Dashboard, Agents, Tasks, etc.)
    └── wizard.rs    # 5-step agent creation wizard
```

---

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib scanner
cargo test --lib agent

# Run with output (useful for debugging)
cargo test -- --nocapture
```

Tests live in `#[cfg(test)]` blocks inside each module. Integration tests that touch the filesystem use the `tempfile` crate to create isolated temporary directories — never write to the real filesystem in tests.

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_scaffold_creates_files() {
        let dir = TempDir::new().unwrap();
        cmd_scaffold(dir.path(), "my-agent", "Test Role").unwrap();
        assert!(dir.path().join(".brain/agents/my-agent/identity.md").exists());
    }
}
```

---

## Code Quality

All code must pass these checks before committing:

```bash
# Format code
cargo fmt

# Lint — treat all warnings as errors
cargo clippy -- -D warnings

# Run tests
cargo test
```

A pre-commit checklist:

- [ ] `cargo fmt` applied
- [ ] `cargo clippy -- -D warnings` passes with zero warnings
- [ ] `cargo test` passes
- [ ] No `unwrap()` calls on `Result`/`Option` in production paths — use `?` or return `Result<T, String>`
- [ ] No `panic!` in production paths

---

## Commit Convention

TreeC uses [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

### Types

| Type | When to use |
|---|---|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `refactor` | Code change that is neither a feature nor a fix |
| `style` | Formatting changes (cargo fmt, whitespace) |
| `test` | Adding or updating tests |
| `docs` | Documentation changes only |
| `chore` | Dependency bumps, CI config, build scripts |
| `perf` | Performance improvements |

### Scopes

Use the module name as scope when applicable: `scanner`, `neural`, `agent`, `brain`, `tui`, `config`, `generator`.

### Examples

```
feat(agent): add treec agent pause command
fix(neural): handle 429 rate-limit with exponential backoff
docs: update README with multi-agent platform section
test(brain): add coverage for scaffold_agent_dir
chore: bump ratatui to 0.30
style: apply cargo fmt across tui module
refactor(config): extract keyring logic into config::keyring submodule
```

---

## Adding a New CLI Command

### 1. Define the argument in `src/main.rs`

For a simple flag:

```rust
/// Description of the new flag
#[arg(long = "my-flag")]
my_flag: bool,
```

For a subcommand, add a variant to the relevant `enum`:

```rust
// In AgentCmd:
/// Short description shown in --help
MyCommand {
    name: String,
    #[arg(long)]
    option: String,
},
```

### 2. Implement the handler

Add the business logic in the relevant module (`agent.rs`, `brain.rs`, etc.):

```rust
pub fn cmd_my_command(root: &Path, name: &str, option: &str) -> Result<(), String> {
    // implementation
    println!("Done.");
    Ok(())
}
```

### 3. Dispatch in `main()`

Wire the new variant in the `match` block inside `main()`:

```rust
AgentCmd::MyCommand { name, option } => {
    if let Err(e) = agent::cmd_my_command(root, &name, &option) {
        eprintln!("[TreeC] Error: {}", e);
        std::process::exit(1);
    }
}
```

### 4. Update the long_about string and README

Add the new command to the `long_about` string in the `#[command(...)]` attribute and to the CLI reference in `README.md`.

---

## Adding a New Agent

Agents are first-class citizens in TreeC. You can add one via CLI or by editing the brain files directly.

### Via CLI (recommended)

```bash
# Scaffold the agent directory with seed files
treec agent scaffold my-agent --role "Description of what this agent does"

# The following files are created:
# .brain/agents/my-agent/identity.md
# .brain/agents/my-agent/instructions.md
# .brain/agents/my-agent/knowledge.md
# .brain/agents/my-agent/tasks.md
# .brain/agents/my-agent/memory.md

# Populate files as needed
treec agent write my-agent instructions --content "## How to act\n- Focus on X\n- Avoid Y"
treec agent write my-agent knowledge --content "## Domain knowledge\n..."

# Activate the agent
treec agent activate my-agent
```

### Via code

If you want an agent that is always scaffolded when `treec --neural-link` runs, add it to the default agent list in `src/brain.rs`. Look for the function that scaffolds the default set of agents and add an entry:

```rust
scaffold_agent_dir(root, "my-agent", "My Agent Role")?;
```

### Agent file conventions

| File | Content |
|---|---|
| `identity.md` | Who the agent is, responsibilities, domain boundaries |
| `instructions.md` | How to act, step-by-step behavior, constraints |
| `knowledge.md` | Domain knowledge, patterns, references |
| `tasks.md` | Tasks delegated by the orchestrator (checklist format) |
| `memory.md` | Accumulated history, decisions, context |

All writes to `.brain/` must go through `treec agent write` or `treec orchestrator write` — never edit files directly in production.

---

## Adding a New AI Provider

Providers are implemented in `src/neural.rs`.

### 1. Add the provider variant

In the provider parsing logic, add recognition of the new provider name:

```rust
"myprovider" | "alias" => Provider::MyProvider,
```

### 2. Implement the request

Add a match arm that builds the HTTP request for the new provider:

```rust
Provider::MyProvider => {
    let url = format!("https://api.myprovider.com/v1/...");
    ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(/* request body */)?
}
```

### 3. Parse the response

Add response parsing for the provider's JSON format.

### 4. Update documentation

Add the provider to the table in `README.md` under the **Neural Link** section.

---

## Pull Request Guidelines

1. Fork the repository and create a branch from `main`:
   ```bash
   git checkout -b feat/my-feature
   ```

2. Make your changes following the conventions in this guide.

3. Ensure all checks pass:
   ```bash
   cargo fmt && cargo clippy -- -D warnings && cargo test
   ```

4. Commit with a conventional commit message.

5. Open a pull request against `main` with:
   - A clear title following the commit convention
   - A description of what changed and why
   - Reference to any related issues

6. A maintainer will review and provide feedback.

---

## Questions

Open an issue on [GitHub](https://github.com/Vitorvzp/TreeC/issues) or check the `.brain/` directory in the repository for in-depth architecture notes.
