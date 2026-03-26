# Changelog

All notable changes to TreeC are documented here.
Format: `## [x.y.z] — Title`

---

## [0.6.0] — Neural Brain Hierárquico, Análise de Dependências e Dry-run

### Breaking Changes
- `.brain/` agora usa estrutura hierárquica com subpastas. Brains gerados com v0.5.0 ou anterior precisam ser regenerados com `treec --neural-link`.

### Added
- **Nova estrutura `.brain/`** — 7 subpastas: `cortex/`, `memory/`, `perception/`, `motor/`, `language/`, `identity/`, `system/` seguindo a especificação do Neural Brain System
- **`cortex/`** — `context`, `architecture`, `decisions`, `roadmap`, `patterns`, `releases` + `knowledge/` (modules, functions, api, database, models, services)
- **`memory/`** — `long_term`, `short_term`, `changelog`, `lessons`
- **`perception/`** — `tree`, `dependencies`, `files_summary`
- **`motor/`** — `tasks`, `backlog`, `bugs`, `issues`
- **`language/`** — `readme`, `documentation`
- **`identity/`** — `project`, `goals`
- **`system/`** — `rules`, `workflow`
- **`--dry-run`** — `treec --neural-link --dry-run` exibe estimativa de tokens + custo sem chamar a API
- **Análise de dependências** — detecta `Cargo.toml`, `package.json`, `requirements.txt`, `go.mod`, `*.csproj` e gera `perception/dependencies.md` automaticamente a cada scan
- **19 campos no `BrainOutput`** — adicionados: `patterns`, `releases`, `documentation`, `backlog`, `bugs`, `project`, `goals`

### Changed
- **Wikilinks com path completo** — IA gera `[[cortex/context]]` em vez de `[[context]]`
- **`--update-brain`** — atualiza `perception/tree.md`, `memory/long_term.md` e `memory/changelog.md` nos novos paths
- **`--status`** — detecta brain flat (v0.5) vs hierárquico (v0.6) e avisa para regenerar
- **`--brain-files`** — lista de chaves disponíveis atualizada com os novos campos

---

## [0.5.0] — Scanner, Paralelismo, Ollama, Custo e CI

### Added
- **`ignore` crate** — scanner reescrito com suporte completo ao `.gitignore` (negação `!`, anchoring, `.git/info/exclude`)
- **Rayon parallelism** — análise de arquivos paralela. Speedup linear com nº de cores
- **Estimativa de custo USD** — custo estimado exibido antes de toda chamada à API
- **Ollama (modelos locais)** — `treec --config-neural ollama llama3.2`. Sem API key, sem nuvem
- **`--brain-files` seletivo** — regenera só os arquivos necessários; reduz custo de API
- **GitHub Actions CI** — `ci.yml`: fmt + clippy + build em ubuntu, windows, macos

### Removed
- Dependências `regex` e `walkdir` substituídas por `ignore` + `rayon`

### Fixed
- CI quality fixes: `cargo fmt` + 5 warnings de clippy corrigidos

---

## [0.4.0] — UX, Diagnóstico e Distribuição

### Added
- **`treec --status`** — diagnóstico completo: config, brain, API key, fonte da key, .gitignore check
- **Estimativa de tokens** — aviso antes de enviar à IA: % uso do context window, abort se >90%
- **Auto-proteção .gitignore** — `--config-neural` adiciona `TreeC.toml` ao `.gitignore` automaticamente
- **Barra de progresso** — `indicatif` spinner + barra durante análise de arquivos
- **GitHub Actions release** — `.github/workflows/release.yml` para binários multiplataforma

---

## [0.3.0] — Robustez e Segurança

### Added
- **TOML parser com `toml` crate** — substituiu parsing frágil por Regex
- **`TREEC_API_KEY` env var** — prioridade sobre `ApiKey` no arquivo
- **Detecção de binários aprimorada** — null bytes + >30% bytes não-imprimíveis
- **`IncludeHiddenDirs`** — nova opção em `[General]` para incluir `.github`, `.vscode` etc.

---

## [0.2.0] — Neural Link

### Added
- **`treec --neural-link`** — cria `.brain/` com 17 arquivos via IA
- **Suporte a Gemini, OpenAI e Claude**
- **`treec --update-brain`** — atualização incremental do brain
- **`treec --config-neural`** — salva provedor + API key no `TreeC.toml`
- **`treec --obsidian`** — configura vault Obsidian dentro de `.brain/`
- **`treec --clean`** — remove `Tree.md`, `Structure.*`, `.brain/`
- **Retry com backoff** — 3 tentativas: 5s → 15s → 30s para erros 429
- **Obsidian wikilinks** — sistema de prompt pede `[[wikilinks]]` entre arquivos

---

## [0.1.0] — Core Scanner

### Added
- **Scanner de diretórios** com filtros por pasta, extensão e arquivo
- **Detecção de arquivos binários** — null byte check nos primeiros 1024 bytes
- **Contagem de LOC** — byte-scan por `0x0A`
- **Detecção de linguagem** — 40+ mapeamentos por extensão
- **Geração de `Tree.md`** — Markdown com árvore ASCII + código fonte
- **Geração de `Structure.json`** e **`Structure.txt`**
- **Respeito ao `.gitignore`**
- **`TreeC.toml`** — configuração por arquivo
- **CLI com clap** — `--help` detalhado
