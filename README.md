<div align="center">

# 🌲 TreeC

**Tree + Content Exporter**

A high-performance CLI tool that maps your repository into a single, structured, AI-ready documentation file.

Uma ferramenta CLI de alta performance que mapeia seu repositório em um único arquivo de documentação estruturado e pronto para IA.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)

</div>

---

## 🇺🇸 English

### What is TreeC?

TreeC scans your entire project and generates structured documentation files that allow someone (or an AI) to understand your codebase **without opening a single file**.

### Generated Artifacts

| File             | Description                                                                        |
| ---------------- | ---------------------------------------------------------------------------------- |
| `Tree.md`        | Full Markdown: summary, ASCII tree, and all file contents with syntax highlighting |
| `Structure.json` | Machine-readable JSON with project stats and file metadata                         |
| `Structure.txt`  | ASCII directory tree only                                                          |

### Installation

```bash
# Clone and install
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo install --path .
```

### Usage

```bash
# Scan current directory
treec

# Scan a specific project
treec C:\path\to\your\project
```

### Configuration

Create a `TreeC.toml` in the project root:

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
Folders = ["target", "node_modules", ".git", "dist", "build"]
Extensions = [".exe", ".dll", ".png", ".jpg", ".zip", ".pdf"]
Files = []
```

### Features

- 🔍 **Binary Detection** — Null-byte check (1KB buffer) skips images, executables, archives
- 🌳 **ASCII Tree** — Folders first, alphabetical, with `├──` / `└──` characters
- 🧠 **40+ Languages** — Syntax highlighting for Rust, Python, JS, TS, Go, C#, and more
- ⚡ **Fast LOC Counting** — Byte-scan approach (no UTF-8 overhead)
- 📋 **GitIgnore Aware** — Respects `.gitignore` patterns automatically
- 📦 **Zero Config** — Works out of the box with sensible defaults

---

## 🇧🇷 Português

### O que é o TreeC?

O TreeC escaneia todo o seu projeto e gera arquivos de documentação estruturados que permitem a qualquer pessoa (ou IA) entender seu código **sem abrir nenhum arquivo**.

### Artefatos Gerados

| Arquivo          | Descrição                                                                                       |
| ---------------- | ----------------------------------------------------------------------------------------------- |
| `Tree.md`        | Markdown completo: resumo, árvore ASCII e conteúdo de todos os arquivos com syntax highlighting |
| `Structure.json` | JSON legível por máquina com estatísticas do projeto e metadados dos arquivos                   |
| `Structure.txt`  | Apenas a árvore de diretórios ASCII                                                             |

### Instalação

```bash
# Clone e instale
git clone https://github.com/Vitorvzp/TreeC.git
cd TreeC
cargo install --path .
```

### Uso

```bash
# Escanear diretório atual
treec

# Escanear um projeto específico
treec C:\caminho\do\seu\projeto
```

### Configuração

Crie um `TreeC.toml` na raiz do projeto:

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
Folders = ["target", "node_modules", ".git", "dist", "build"]
Extensions = [".exe", ".dll", ".png", ".jpg", ".zip", ".pdf"]
Files = []
```

### Funcionalidades

- 🔍 **Detecção de Binários** — Verificação de null-byte (buffer 1KB) pula imagens, executáveis e arquivos compactados
- 🌳 **Árvore ASCII** — Pastas primeiro, ordem alfabética, com caracteres `├──` / `└──`
- 🧠 **40+ Linguagens** — Syntax highlighting para Rust, Python, JS, TS, Go, C# e mais
- ⚡ **Contagem Rápida de LOC** — Abordagem por byte-scan (sem overhead de UTF-8)
- 📋 **Integração com GitIgnore** — Respeita padrões do `.gitignore` automaticamente
- 📦 **Zero Configuração** — Funciona imediatamente com padrões sensatos

---

### Output Example / Exemplo de Saída

```
🌲 TreeC v0.1.0 | Scanning 'my-project'...
   📂 Found 42 files in 8 folders
   📄 40 text files, 2 binary files skipped
   📊 3847 total lines of code

✅ Documentation generated in 0.05s
   Artifacts: Tree.md, Structure.json, Structure.txt
```

---

<div align="center">

**Built with ❤️ and Rust 🦀**

</div>
