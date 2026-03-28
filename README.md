<!-- 🇧🇷 Português -->

<div align="center">

# TreeC

**Tree + Content Exporter & AI Neural Brain**

Uma ferramenta CLI de alta performance que mapeia seu repositório, gera documentação estruturada e executa uma plataforma multi-agente de IA para o seu projeto.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0-blue?style=for-the-badge)](Cargo.toml)
[![AI](https://img.shields.io/badge/AI_Powered-Gemini%20|%20OpenAI%20|%20Claude%20|%20Ollama-blue?style=for-the-badge)](https://ai.google.dev/)

</div>

---

## O que é o TreeC?

O TreeC escaneia seu projeto inteiro e gera arquivos de documentação estruturada, permitindo que qualquer pessoa (ou uma IA) entenda sua base de código **sem abrir um único arquivo**.

Com o recurso **Neural Link**, o TreeC se conecta a um provedor de IA (Gemini, OpenAI, Claude ou Ollama) e gera automaticamente uma base de conhecimento `.brain/` — um segundo cérebro para o seu projeto, pronto para visualizar no [Obsidian](https://obsidian.md).

Com a **Plataforma Multi-Agente** (v1.0.0), você pode criar agentes de IA especializados dentro de `.brain/agents/`, delegar tarefas através de um orquestrador e gerenciar tudo por uma interface de terminal construída com [ratatui](https://ratatui.rs).

---

## Instalação

```bash
# A partir do código-fonte (requer Rust toolchain)
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo install --path .

# Ou instale direto do crates.io
cargo install treec-rust
```

Após a instalação, o binário `treec` estará disponível no seu PATH.

---

## Início Rápido

```bash
# 1. Entre no diretório do seu projeto
cd meu-projeto

# 2. Crie o arquivo de configuração padrão
treec --config

# 3. Scan básico — gera Tree.md, Structure.json, Structure.txt
treec

# 4. Configure um provedor de IA
treec --config-neural gemini SUA_CHAVE_GEMINI
# treec --config-neural openai SUA_CHAVE_OPENAI
# treec --config-neural claude SUA_CHAVE_CLAUDE
# treec --config-neural ollama   (sem chave para Ollama local)

# 5. Gere o brain de IA (diretório .brain/)
treec --neural-link

# 6. Configure um vault do Obsidian apontando para .brain/
treec --obsidian

# 7. Atualize o brain após fazer alterações no projeto
treec --update-brain
```

---

## Referência CLI

### Comandos Principais

| Comando | Descrição |
|---|---|
| `treec [path]` | Escaneia o projeto e gera Tree.md, Structure.json, Structure.txt |
| `treec --neural-link` | Cria o segundo cérebro de IA (`.brain/`) |
| `treec --neural-link --dry-run` | Visualiza o prompt sem chamar a IA |
| `treec --update-brain` | Atualiza o `.brain/` existente com o scan mais recente |
| `treec --config-neural <PROVIDER> [KEY]` | Configura provedor de IA e chave de API |
| `treec --neural-link-remove-api` | Remove a configuração de API armazenada |
| `treec --config` | Cria o arquivo padrão `TreeC.toml` |
| `treec --obsidian` | Configura vault do Obsidian para `.brain/` |
| `treec --status` | Exibe o status atual do TreeC (config, brain, API key) |
| `treec --clean` | Remove todos os arquivos gerados (Tree.md, Structure.*, .brain/) |
| `treec --brain-files <keys>` | Regenera apenas arquivos específicos do brain (separados por vírgula) |
| `treec tui` | Abre o dashboard interativo TUI |
| `treec --help` | Exibe a ajuda |

### Comandos de Agente

Gerencie agentes de IA especializados que vivem em `.brain/agents/<name>/`.

| Comando | Descrição |
|---|---|
| `treec agent scaffold <name> --role "<role>"` | Cria um novo agente com arquivos iniciais |
| `treec agent write <name> <file> --content "<text>"` | Escreve conteúdo em um arquivo do agente |
| `treec agent activate <name>` | Ativa um agente pendente |
| `treec agent list` | Lista todos os agentes |
| `treec agent list --pending` | Lista apenas agentes pendentes |
| `treec agent status <name>` | Exibe status e lista de arquivos de um agente |

### Comandos do Orquestrador

Gerencie o estado compartilhado do orquestrador em `.brain/orchestrator/`.

| Comando | Descrição |
|---|---|
| `treec orchestrator read` | Lê `orchestrator/tasks.md` |
| `treec orchestrator write <file> --content "<text>"` | Escreve conteúdo em um arquivo do orquestrador |
| `treec orchestrator status` | Exibe o status do orquestrador |

---

## Artefatos Gerados

| Arquivo | Descrição |
|---|---|
| `Tree.md` | Markdown completo: sumário, árvore ASCII e conteúdo de todos os arquivos com syntax highlighting |
| `Structure.json` | JSON legível por máquina com estatísticas e metadados do projeto |
| `Structure.txt` | Apenas a árvore ASCII de diretórios |
| `.brain/` | Base de conhecimento gerada por IA (compatível com Obsidian) |

### Estrutura do Brain

O diretório `.brain/` é um vault compatível com Obsidian que usa `[[wikilinks]]`:

```
.brain/
├── index.md              <- Hub de navegação com neural links
├── context.md            <- Visão geral do projeto, tech stack, riscos
├── architecture.md       <- Arquitetura do sistema + diagramas Mermaid
├── memory.md             <- Memória de longo prazo (somente append)
├── changelog.md          <- Rastreamento de alterações
├── roadmap.md            <- Melhorias sugeridas
├── decisions.md          <- Decisões técnicas (estilo ADR)
├── prompt.md             <- Regras de comportamento para agentes de IA
├── tree.md               <- Snapshot mais recente da estrutura de arquivos
├── readme.md             <- README gerado por IA
├── tasks.md              <- Tarefas pendentes, TODOs, débito técnico
├── agents/               <- Plataforma multi-agente (v1.0.0)
│   ├── <agent-name>/
│   │   ├── identity.md
│   │   ├── instructions.md
│   │   ├── knowledge.md
│   │   ├── tasks.md
│   │   └── memory.md
└── knowledge/
    ├── modules.md        <- Documentação de módulos
    ├── functions.md      <- Assinaturas e docs de funções
    ├── api.md            <- Endpoints e rotas de API
    ├── database.md       <- Schema e queries do banco de dados
    ├── models.md         <- Modelos de dados e tipos
    └── services.md       <- Serviços e integrações
```

---

## Plataforma Multi-Agente

O TreeC v1.0.0 inclui uma plataforma multi-agente construída sobre a base de conhecimento `.brain/`. Cada agente é uma persona baseada em Markdown com sua própria identidade, instruções, conhecimento, tarefas e memória.

### Como Funciona

1. **Crie** um agente com nome e papel:

   ```bash
   treec agent scaffold backend --role "Especialista em backend Rust"
   ```

   Isso cria `.brain/agents/backend/` com cinco arquivos iniciais: `identity.md`, `instructions.md`, `knowledge.md`, `tasks.md` e `memory.md`.

2. **Ative** o agente quando estiver pronto para trabalhar:

   ```bash
   treec agent activate backend
   ```

3. **Escreva** em qualquer arquivo do agente para atualizar contexto, registrar decisões ou atribuir tarefas:

   ```bash
   treec agent write backend tasks --content "## Alta Prioridade\n- [ ] Refatorar tratamento de erros em neural.rs"
   treec agent write backend memory --content "## 2026-03-28\nDecidido usar ureq em vez de reqwest para HTTP síncrono."
   ```

4. **Verifique o status** de todos os agentes de uma vez:

   ```bash
   treec agent list
   treec agent status backend
   ```

### Agentes Padrão

Quando você executa `treec --neural-link`, um conjunto de agentes especializados padrão pode ser criado:

| Agente | Domínio |
|---|---|
| `architect` | Decisões de arquitetura, ADRs, design de sistema |
| `rust-backend` | `src/*.rs`, Cargo.toml, performance, CLI API |
| `frontend` | `src/tui/`, UX, ratatui, crossterm |
| `backend` | `neural.rs`, `config.rs`, `scanner.rs`, provedores de IA |
| `docs` | README.md, documentação pública, exemplos de uso |
| `tests` | Testes de integração, cobertura, CI |

### Orquestrador

O orquestrador coordena o trabalho entre agentes via arquivos compartilhados:

```bash
# Leia a lista de tarefas atual delegada a todos os agentes
treec orchestrator read

# Delegue novas tarefas do orquestrador
treec orchestrator write tasks --content "## Sprint 2026-03-28\n- [ ] docs: atualizar README\n- [ ] tests: adicionar cobertura"

# Verifique o estado geral do orquestrador
treec orchestrator status
```

---

## Neural Link (Provedores de IA)

O TreeC suporta quatro provedores de IA para os recursos Neural Link e geração de brain.

| Provedor | Alias | Modelo Padrão | Autenticação |
|---|---|---|---|
| `gemini` | `google` | `gemini-2.0-flash` | API Key |
| `openai` | `gpt` | `gpt-4o` | Bearer Token |
| `claude` | `anthropic` | `claude-3-5-sonnet` | x-api-key Header |
| `ollama` | — | `llama3` | Nenhuma (local) |

### Configurar um Provedor

```bash
# Gemini (Google)
treec --config-neural gemini SUA_CHAVE_GEMINI

# OpenAI
treec --config-neural openai SUA_CHAVE_OPENAI

# Anthropic Claude
treec --config-neural claude SUA_CHAVE_CLAUDE

# Ollama (local, sem chave)
treec --config-neural ollama
```

A chave de API é armazenada com segurança no keyring do sistema. Ela nunca é gravada em arquivos de texto simples e nunca é exposta em mensagens de erro.

### Regeneração Seletiva do Brain

Regenere apenas arquivos específicos do brain em vez do brain completo:

```bash
# Regenera apenas context e architecture
treec --update-brain --brain-files context,architecture

# Chaves disponíveis:
# context, architecture, decisions, roadmap, patterns, releases,
# modules, functions, api, database, models, services,
# readme, documentation, tasks, backlog, bugs, project, goals
```

---

## Dashboard TUI

O TreeC inclui uma interface de terminal interativa construída com [ratatui](https://ratatui.rs):

```bash
treec tui
```

### Telas

| Tela | Conteúdo |
|---|---|
| Dashboard | Visão geral do projeto e resumo do brain |
| Agents | Lista de agentes com indicadores de status |
| Tasks | Lista de tarefas de `orchestrator/tasks.md` |
| BrainViewer | Navegue e leia arquivos `.brain/` |
| SharedMemory | Conteúdo de `shared_memory/` |
| Changelog | `shared_memory/changelog.md` |
| CreateAgent | Wizard de 5 etapas para criar um novo agente |

### Navegação

| Tecla | Ação |
|---|---|
| `Tab` / Setas | Navegar entre telas e itens |
| `Enter` | Selecionar / abrir |
| `q` | Sair |
| `?` | Exibir overlay de ajuda |

---

## Configuração

Crie um `TreeC.toml` na raiz do seu projeto (ou execute `treec --config`):

```toml
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
Folders = ["target", "node_modules", ".git", "dist", "build", ".brain"]
Extensions = [".exe", ".dll", ".png", ".jpg", ".zip", ".pdf"]
Files = []

[NeuralLink]
Provider = "gemini"
Model = "gemini-2.0-flash"
ApiKey = "SUA_CHAVE_API"
```

---

## Funcionalidades

- **Detecção de Binários** — Verificação de null-byte (buffer de 1 KB) pula imagens, executáveis e arquivos compactados
- **Árvore ASCII** — Pastas primeiro, ordem alfabética, com caracteres `├──` / `└──`
- **40+ Linguagens** — Syntax highlighting para Rust, Python, JS, TS, Go, C# e mais
- **Contagem de LOC Rápida** — Abordagem por byte-scan (sem overhead de UTF-8)
- **Respeita GitIgnore** — Segue padrões `.gitignore` automaticamente
- **Zero Config** — Funciona direto com padrões sensatos
- **Neural Link** — Geração de segundo cérebro com IA
- **Multi-Provedor** — Suporta Gemini, OpenAI, Claude e Ollama
- **Plataforma Multi-Agente** — Crie, ative e coordene agentes de IA especializados
- **Dashboard TUI** — Interface de terminal interativa com ratatui e wizard de agentes
- **Integração com Obsidian** — Visualização em grafo com conexões neurais
- **Auto Retry** — Backoff exponencial para APIs com rate limiting
- **Seguro** — Chaves de API no keyring do sistema, nunca expostas em logs

---

## Exemplo de Saída

```
TreeC v1.0.0 | Scanning 'meu-projeto'...
   Found 42 files in 8 folders
   40 text files, 2 binary files skipped
   3847 total lines of code
   Artifacts: Tree.md, Structure.json, Structure.txt

Neural Link activated!
   Provider: gemini | Model: gemini-2.0-flash
   Initializing .brain/ structure...
   Sending project to AI (gemini / gemini-2.0-flash)...
   Writing brain files...
   12 brain files populated by AI
   Neural Link complete! .brain/ is ready.

Neural Link completed in 12.34s
   Brain: .brain/ directory ready
   Tip: Run 'treec --obsidian' to set up the vault
   Tip: Run 'treec tui' to open the dashboard
```

---

<div align="center">

**Construído com Rust — Movido por IA**

</div>

---

<!-- 🇺🇸 English -->

<div align="center">

# TreeC

**Tree + Content Exporter & AI Neural Brain**

A high-performance CLI tool that maps your repository, generates structured documentation, and runs a multi-agent AI platform for your project.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0-blue?style=for-the-badge)](Cargo.toml)
[![AI](https://img.shields.io/badge/AI_Powered-Gemini%20|%20OpenAI%20|%20Claude%20|%20Ollama-blue?style=for-the-badge)](https://ai.google.dev/)

</div>

---

## What is TreeC?

TreeC scans your entire project and generates structured documentation files, allowing anyone (or an AI) to understand your codebase **without opening a single file**.

With the **Neural Link** feature, TreeC connects to an AI provider (Gemini, OpenAI, Claude, or Ollama) and automatically generates a `.brain/` knowledge base — a second brain for your project, ready to view in [Obsidian](https://obsidian.md).

With the **Multi-Agent Platform** (v1.0.0), you can scaffold specialized AI agents inside `.brain/agents/`, delegate tasks through an orchestrator, and manage everything through a terminal UI built with [ratatui](https://ratatui.rs).

---

## Installation

```bash
# From source (requires Rust toolchain)
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo install --path .

# Or install directly from crates.io
cargo install treec-rust
```

After installation, the `treec` binary will be available in your PATH.

---

## Quick Start

```bash
# 1. Enter your project directory
cd my-project

# 2. Create a default config file
treec --config

# 3. Basic scan — generates Tree.md, Structure.json, Structure.txt
treec

# 4. Configure an AI provider
treec --config-neural gemini YOUR_GEMINI_API_KEY
# treec --config-neural openai YOUR_OPENAI_KEY
# treec --config-neural claude YOUR_CLAUDE_KEY
# treec --config-neural ollama   (no key needed for local Ollama)

# 5. Generate the AI brain (.brain/ directory)
treec --neural-link

# 6. Set up an Obsidian vault pointing at .brain/
treec --obsidian

# 7. Update the brain after making changes to your project
treec --update-brain
```

---

## CLI Reference

### Core Commands

| Command | Description |
|---|---|
| `treec [path]` | Scan project and generate Tree.md, Structure.json, Structure.txt |
| `treec --neural-link` | Create AI second brain (`.brain/`) |
| `treec --neural-link --dry-run` | Preview the prompt without calling the AI |
| `treec --update-brain` | Update existing `.brain/` with latest project scan |
| `treec --config-neural <PROVIDER> [KEY]` | Configure AI provider and API key |
| `treec --neural-link-remove-api` | Remove stored API configuration |
| `treec --config` | Create default `TreeC.toml` |
| `treec --obsidian` | Set up Obsidian vault for `.brain/` |
| `treec --status` | Show current TreeC status (config, brain, API key) |
| `treec --clean` | Remove all generated files (Tree.md, Structure.*, .brain/) |
| `treec --brain-files <keys>` | Regenerate specific brain files only (comma-separated) |
| `treec tui` | Open the interactive TUI dashboard |
| `treec --help` | Show help |

### Agent Commands

Manage specialized AI agents that live in `.brain/agents/<name>/`.

| Command | Description |
|---|---|
| `treec agent scaffold <name> --role "<role>"` | Create a new agent with seed files |
| `treec agent write <name> <file> --content "<text>"` | Write content to an agent's brain file |
| `treec agent activate <name>` | Activate a pending agent |
| `treec agent list` | List all agents |
| `treec agent list --pending` | List only pending agents |
| `treec agent status <name>` | Show status and file list for an agent |

### Orchestrator Commands

Manage shared orchestrator state in `.brain/orchestrator/`.

| Command | Description |
|---|---|
| `treec orchestrator read` | Read `orchestrator/tasks.md` |
| `treec orchestrator write <file> --content "<text>"` | Write content to an orchestrator file |
| `treec orchestrator status` | Show orchestrator status |

---

## Generated Artifacts

| File | Description |
|---|---|
| `Tree.md` | Full Markdown: summary, ASCII tree, and all file contents with syntax highlighting |
| `Structure.json` | Machine-readable JSON with project stats and file metadata |
| `Structure.txt` | ASCII directory tree only |
| `.brain/` | AI-generated knowledge base (Obsidian-compatible) |

### Brain Structure

The `.brain/` directory is an Obsidian-compatible knowledge vault that uses `[[wikilinks]]`:

```
.brain/
├── index.md              <- Navigation hub with neural links
├── context.md            <- Project overview, tech stack, risks
├── architecture.md       <- System architecture + Mermaid diagrams
├── memory.md             <- Long-term memory (append-only)
├── changelog.md          <- Change tracking
├── roadmap.md            <- Suggested improvements
├── decisions.md          <- Technical decisions (ADR-style)
├── prompt.md             <- AI agent behavior rules
├── tree.md               <- Latest file structure snapshot
├── readme.md             <- AI-generated README
├── tasks.md              <- Pending tasks, TODOs, technical debt
├── agents/               <- Multi-agent platform (v1.0.0)
│   ├── <agent-name>/
│   │   ├── identity.md
│   │   ├── instructions.md
│   │   ├── knowledge.md
│   │   ├── tasks.md
│   │   └── memory.md
└── knowledge/
    ├── modules.md        <- Module documentation
    ├── functions.md      <- Function signatures & docs
    ├── api.md            <- API endpoints & routes
    ├── database.md       <- Database schema & queries
    ├── models.md         <- Data models & types
    └── services.md       <- Services & integrations
```

---

## Multi-Agent Platform

TreeC v1.0.0 ships a multi-agent platform built on top of the `.brain/` knowledge base. Each agent is a Markdown-based persona with its own identity, instructions, knowledge, tasks, and memory.

### How It Works

1. **Scaffold** an agent with a name and role:

   ```bash
   treec agent scaffold backend --role "Rust backend specialist"
   ```

   This creates `.brain/agents/backend/` with five seed files: `identity.md`, `instructions.md`, `knowledge.md`, `tasks.md`, and `memory.md`.

2. **Activate** the agent when it is ready to work:

   ```bash
   treec agent activate backend
   ```

3. **Write** to any of the agent's files to update context, log decisions, or assign tasks:

   ```bash
   treec agent write backend tasks --content "## High Priority\n- [ ] Refactor neural.rs error handling"
   treec agent write backend memory --content "## 2026-03-28\nDecided to use ureq over reqwest for sync HTTP."
   ```

4. **Check status** of all agents at once:

   ```bash
   treec agent list
   treec agent status backend
   ```

### Built-in Agents

When you run `treec --neural-link`, a set of default specialized agents can be scaffolded:

| Agent | Domain |
|---|---|
| `architect` | Architecture decisions, ADRs, system design |
| `rust-backend` | `src/*.rs`, Cargo.toml, performance, CLI API |
| `frontend` | `src/tui/`, UX, ratatui, crossterm |
| `backend` | `neural.rs`, `config.rs`, `scanner.rs`, AI providers |
| `docs` | README.md, public documentation, usage examples |
| `tests` | Integration tests, coverage, CI |

### Orchestrator

The orchestrator coordinates work across agents via shared files:

```bash
# Read the current task list delegated to all agents
treec orchestrator read

# Delegate new tasks from the orchestrator
treec orchestrator write tasks --content "## Sprint 2026-03-28\n- [ ] docs: update README\n- [ ] tests: add coverage"

# Check overall orchestrator state
treec orchestrator status
```

---

## Neural Link (AI Providers)

TreeC supports four AI providers for the Neural Link and brain generation features.

| Provider | Alias | Default Model | Auth |
|---|---|---|---|
| `gemini` | `google` | `gemini-2.0-flash` | API Key |
| `openai` | `gpt` | `gpt-4o` | Bearer Token |
| `claude` | `anthropic` | `claude-3-5-sonnet` | x-api-key Header |
| `ollama` | — | `llama3` | None (local) |

### Configure a Provider

```bash
# Gemini (Google)
treec --config-neural gemini YOUR_GEMINI_API_KEY

# OpenAI
treec --config-neural openai YOUR_OPENAI_API_KEY

# Anthropic Claude
treec --config-neural claude YOUR_CLAUDE_API_KEY

# Ollama (local, no key needed)
treec --config-neural ollama
```

The API key is stored securely in the system keyring. It is never written to plain-text files and never exposed in error messages.

### Selective Brain Regeneration

Regenerate only specific brain files instead of the full brain:

```bash
# Regenerate only context and architecture
treec --update-brain --brain-files context,architecture

# Available keys:
# context, architecture, decisions, roadmap, patterns, releases,
# modules, functions, api, database, models, services,
# readme, documentation, tasks, backlog, bugs, project, goals
```

---

## TUI Dashboard

TreeC ships an interactive terminal UI built with [ratatui](https://ratatui.rs):

```bash
treec tui
```

### Screens

| Screen | Content |
|---|---|
| Dashboard | Project overview and brain summary |
| Agents | List of agents with status indicators |
| Tasks | Task list from `orchestrator/tasks.md` |
| BrainViewer | Browse and read `.brain/` files |
| SharedMemory | Contents of `shared_memory/` |
| Changelog | `shared_memory/changelog.md` |
| CreateAgent | 5-step wizard to create a new agent |

### Navigation

| Key | Action |
|---|---|
| `Tab` / Arrow keys | Navigate between screens and items |
| `Enter` | Select / open |
| `q` | Quit |
| `?` | Show help overlay |

---

## Configuration

Create a `TreeC.toml` in your project root (or run `treec --config`):

```toml
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
Folders = ["target", "node_modules", ".git", "dist", "build", ".brain"]
Extensions = [".exe", ".dll", ".png", ".jpg", ".zip", ".pdf"]
Files = []

[NeuralLink]
Provider = "gemini"
Model = "gemini-2.0-flash"
ApiKey = "YOUR_API_KEY"
```

---

## Features

- **Binary Detection** — Null-byte check (1 KB buffer) skips images, executables, archives
- **ASCII Tree** — Folders first, alphabetical, with `├──` / `└──` characters
- **40+ Languages** — Syntax highlighting for Rust, Python, JS, TS, Go, C# and more
- **Fast LOC Counting** — Byte-scan approach (no UTF-8 overhead)
- **GitIgnore Aware** — Respects `.gitignore` patterns automatically
- **Zero Config** — Works out of the box with sensible defaults
- **Neural Link** — AI-powered second brain generation
- **Multi-Provider** — Supports Gemini, OpenAI, Claude, and Ollama
- **Multi-Agent Platform** — Scaffold, activate, and coordinate specialized AI agents
- **TUI Dashboard** — ratatui-based interactive terminal interface with agent wizard
- **Obsidian Integration** — Graph view with neural connections
- **Auto Retry** — Exponential backoff for rate-limited APIs
- **Secure** — API keys stored in system keyring, never exposed in logs

---

## Output Example

```
TreeC v1.0.0 | Scanning 'my-project'...
   Found 42 files in 8 folders
   40 text files, 2 binary files skipped
   3847 total lines of code
   Artifacts: Tree.md, Structure.json, Structure.txt

Neural Link activated!
   Provider: gemini | Model: gemini-2.0-flash
   Initializing .brain/ structure...
   Sending project to AI (gemini / gemini-2.0-flash)...
   Writing brain files...
   12 brain files populated by AI
   Neural Link complete! .brain/ is ready.

Neural Link completed in 12.34s
   Brain: .brain/ directory ready
   Tip: Run 'treec --obsidian' to set up the vault
   Tip: Run 'treec tui' to open the dashboard
```

---

<div align="center">

**Built with Rust — Powered by AI**

</div>
