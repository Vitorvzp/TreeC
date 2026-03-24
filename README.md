<div align="center">

# 🌲 TreeC

**Tree + Content Exporter & AI Neural Brain**

A high-performance CLI tool that maps your repository and builds an AI-powered second brain for your project.

Uma ferramenta CLI de alta performance que mapeia seu repositório e constrói um segundo cérebro com IA para o seu projeto.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)
[![AI](https://img.shields.io/badge/AI_Powered-Gemini%20|%20OpenAI%20|%20Claude-blue?style=for-the-badge)](https://ai.google.dev/)

</div>

---

## 🇺🇸 English

### What is TreeC?

TreeC scans your entire project and generates structured documentation files, allowing anyone (or an AI) to understand your codebase **without opening a single file**.

With the **Neural Link** feature, TreeC connects to an AI provider (Gemini, OpenAI, or Claude) and automatically generates a `.brain/` knowledge base — a second brain for your project, ready to view in [Obsidian](https://obsidian.md).

### Generated Artifacts

| File             | Description                                                                        |
| ---------------- | ---------------------------------------------------------------------------------- |
| `Tree.md`        | Full Markdown: summary, ASCII tree, and all file contents with syntax highlighting |
| `Structure.json` | Machine-readable JSON with project stats and file metadata                         |
| `Structure.txt`  | ASCII directory tree only                                                          |
| `.brain/`        | 🧠 AI-generated knowledge base (Obsidian-compatible)                              |

### Installation

```bash
# From source
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo install --path .

# Or from a release binary
# Download treec.exe from GitHub Releases and add to your PATH
```

### CLI Commands

| Command                                   | Description                                  |
| ----------------------------------------- | -------------------------------------------- |
| `treec`                                   | Scan project and generate Tree.md            |
| `treec --neural-link`                     | 🧠 Create AI Second Brain (`.brain/`)       |
| `treec --update-brain`                    | 🔄 Update existing brain with latest changes |
| `treec --config-neural <PROVIDER> <KEY>`  | ⚙️ Configure AI provider and API key         |
| `treec --neural-link-remove-api`          | 🔑 Remove API configuration                 |
| `treec --config`                          | 📄 Create default `TreeC.toml`               |
| `treec --obsidian`                        | 🔮 Setup Obsidian vault for `.brain/`        |
| `treec --clean`                           | 🧹 Remove all generated files                |
| `treec --help`                            | ❓ Show help                                 |

### Quick Start

```bash
# 1. Enter your project
cd my-project

# 2. Create config file
treec --config

# 3. Basic scan (generates Tree.md, Structure.json, Structure.txt)
treec

# 4. Configure AI (choose your provider)
treec --config-neural gemini YOUR_GEMINI_KEY
# treec --config-neural openai YOUR_OPENAI_KEY
# treec --config-neural claude YOUR_CLAUDE_KEY

# 5. Generate AI brain
treec --neural-link

# 6. Setup Obsidian vault
treec --obsidian

# 7. After making changes to your project
treec --update-brain
```

### AI Providers

| Provider  | Alias       | Default Model             | Auth Method       |
| --------- | ----------- | ------------------------- | ----------------- |
| `gemini`  | `google`    | `gemini-2.0-flash`        | API Key (URL)     |
| `openai`  | `gpt`       | `gpt-4.1-mini`            | Bearer Token      |
| `claude`  | `anthropic` | `claude-sonnet-4-20250514`| x-api-key Header  |

### Brain Structure

The `.brain/` directory is an Obsidian-compatible knowledge vault with `[[wikilinks]]`:

```
.brain/
├── index.md              ← Navigation hub with neural links
├── context.md            ← Project overview, tech stack, risks
├── architecture.md       ← System architecture + Mermaid diagrams
├── memory.md             ← Long-term memory (append-only)
├── changelog.md          ← Change tracking
├── roadmap.md            ← Suggested improvements
├── decisions.md          ← Technical decisions (ADR-style)
├── prompt.md             ← AI agent behavior rules
├── tree.md               ← Latest file structure
├── readme.md             ← AI-generated README
├── tasks.md              ← Pending tasks, TODOs, debt
└── knowledge/
    ├── modules.md        ← Module documentation
    ├── functions.md      ← Function signatures & docs
    ├── api.md            ← API endpoints & routes
    ├── database.md       ← Database schema & queries
    ├── models.md         ← Data models & types
    └── services.md       ← Services & integrations
```

### Configuration

Create a `TreeC.toml` in the project root (or run `treec --config`):

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

### Features

- 🔍 **Binary Detection** — Null-byte check (1KB buffer) skips images, executables, archives
- 🌳 **ASCII Tree** — Folders first, alphabetical, with `├──` / `└──` characters
- 🧠 **40+ Languages** — Syntax highlighting for Rust, Python, JS, TS, Go, C# and more
- ⚡ **Fast LOC Counting** — Byte-scan approach (no UTF-8 overhead)
- 📋 **GitIgnore Aware** — Respects `.gitignore` patterns automatically
- 📦 **Zero Config** — Works out of the box with sensible defaults
- 🤖 **Neural Link** — AI-powered second brain generation
- 🔄 **Multi-Provider** — Supports Gemini, OpenAI, and Claude
- 🔮 **Obsidian Integration** — Graph view with neural connections
- 🔁 **Auto Retry** — Exponential backoff for rate-limited APIs
- 🔒 **Secure** — API keys never exposed in error messages

---

## 🇧🇷 Português

### O que é o TreeC?

O TreeC escaneia todo o seu projeto e gera arquivos de documentação estruturados que permitem a qualquer pessoa (ou IA) entender seu código **sem abrir nenhum arquivo**.

Com a funcionalidade **Neural Link**, o TreeC se conecta a um provedor de IA (Gemini, OpenAI ou Claude) e gera automaticamente um `.brain/` — um segundo cérebro para o seu projeto, pronto para visualização no [Obsidian](https://obsidian.md).

### Artefatos Gerados

| Arquivo          | Descrição                                                                                       |
| ---------------- | ----------------------------------------------------------------------------------------------- |
| `Tree.md`        | Markdown completo: resumo, árvore ASCII e conteúdo de todos os arquivos com syntax highlighting |
| `Structure.json` | JSON legível por máquina com estatísticas do projeto e metadados dos arquivos                   |
| `Structure.txt`  | Apenas a árvore de diretórios ASCII                                                             |
| `.brain/`        | 🧠 Base de conhecimento gerada por IA (compatível com Obsidian)                                |

### Instalação

```bash
# Pelo código fonte
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo install --path .

# Ou pelo binário de release
# Baixe treec.exe do GitHub Releases e adicione ao PATH
```

### Comandos CLI

| Comando                                   | Descrição                                        |
| ----------------------------------------- | ------------------------------------------------ |
| `treec`                                   | Escaneia e gera Tree.md                          |
| `treec --neural-link`                     | 🧠 Cria o Segundo Cérebro com IA (`.brain/`)    |
| `treec --update-brain`                    | 🔄 Atualiza brain com as últimas mudanças        |
| `treec --config-neural <PROVIDER> <KEY>`  | ⚙️ Configura provedor e chave da API             |
| `treec --neural-link-remove-api`          | 🔑 Remove configuração da API                   |
| `treec --config`                          | 📄 Cria `TreeC.toml` padrão                     |
| `treec --obsidian`                        | 🔮 Configura vault do Obsidian no `.brain/`      |
| `treec --clean`                           | 🧹 Remove todos os arquivos gerados              |
| `treec --help`                            | ❓ Mostra ajuda                                  |

### Início Rápido

```bash
# 1. Entre no seu projeto
cd meu-projeto

# 2. Crie config
treec --config

# 3. Scan básico (gera Tree.md, Structure.json, Structure.txt)
treec

# 4. Configure a IA (escolha seu provedor)
treec --config-neural gemini SUA_CHAVE_GEMINI
# treec --config-neural openai SUA_CHAVE_OPENAI
# treec --config-neural claude SUA_CHAVE_CLAUDE

# 5. Gere o cérebro com IA
treec --neural-link

# 6. Configure o vault Obsidian
treec --obsidian

# 7. Após fazer mudanças no projeto
treec --update-brain
```

### Provedores de IA

| Provedor  | Alias       | Modelo Padrão             | Método de Auth    |
| --------- | ----------- | ------------------------- | ----------------- |
| `gemini`  | `google`    | `gemini-2.0-flash`        | API Key (URL)     |
| `openai`  | `gpt`       | `gpt-4.1-mini`            | Bearer Token      |
| `claude`  | `anthropic` | `claude-sonnet-4-20250514`| Header x-api-key  |

### Estrutura do Brain

O diretório `.brain/` é um vault compatível com Obsidian que usa `[[wikilinks]]`:

```
.brain/
├── index.md              ← Hub de navegação com links neurais
├── context.md            ← Visão geral, tech stack, riscos
├── architecture.md       ← Arquitetura + diagramas Mermaid
├── memory.md             ← Memória de longo prazo (append-only)
├── changelog.md          ← Rastreamento de mudanças
├── roadmap.md            ← Sugestões de melhorias
├── decisions.md          ← Decisões técnicas (estilo ADR)
├── prompt.md             ← Regras de comportamento do agente
├── tree.md               ← Estrutura mais recente
├── readme.md             ← README gerado pela IA
├── tasks.md              ← Tarefas pendentes, TODOs, dívidas
└── knowledge/
    ├── modules.md        ← Documentação de módulos
    ├── functions.md      ← Assinaturas e documentação de funções
    ├── api.md            ← Endpoints e rotas da API
    ├── database.md       ← Schema e queries do banco de dados
    ├── models.md         ← Modelos de dados e tipos
    └── services.md       ← Serviços e integrações
```

### Funcionalidades

- 🔍 **Detecção de Binários** — Verificação de null-byte pula imagens, executáveis e arquivos compactados
- 🌳 **Árvore ASCII** — Pastas primeiro, ordem alfabética, com caracteres `├──` / `└──`
- 🧠 **40+ Linguagens** — Syntax highlighting para Rust, Python, JS, TS, Go, C# e mais
- ⚡ **Contagem Rápida de LOC** — Abordagem por byte-scan (sem overhead de UTF-8)
- 📋 **Integração com GitIgnore** — Respeita padrões do `.gitignore` automaticamente
- 📦 **Zero Configuração** — Funciona imediatamente com padrões sensatos
- 🤖 **Neural Link** — Geração de segundo cérebro com IA
- 🔄 **Multi-Provedor** — Suporta Gemini, OpenAI e Claude
- 🔮 **Integração Obsidian** — Visualização em grafo com conexões neurais
- 🔁 **Retry Automático** — Backoff exponencial para APIs com rate limit
- 🔒 **Seguro** — Chaves de API nunca expostas em mensagens de erro

---

### Output Example / Exemplo de Saída

```
🌲 TreeC v0.2.0 | Scanning 'my-project'...
   📂 Found 42 files in 8 folders
   📄 40 text files, 2 binary files skipped
   📊 3847 total lines of code
   📦 Artifacts: Tree.md, Structure.json, Structure.txt

🧠 Neural Link activated!
   Provider: gemini | Model: gemini-2.0-flash
   🧠 Initializing .brain/ structure...
   🔗 Sending project to AI (gemini / gemini-2.0-flash)...
   ⏳ This may take a moment...
   📝 Writing brain files...
   📄 12 brain files populated by AI
   ✅ Neural Link complete! .brain/ is ready.

🧠 Neural Link completed in 12.34s
   Brain: .brain/ directory ready
   💡 Tip: Run 'treec --obsidian' to setup the vault
```

---

<div align="center">

**Built with ❤️ and Rust 🦀 — Powered by AI 🧠**

</div>
