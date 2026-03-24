# 🗺️ TreeC — Roadmap

> Registro completo de funcionalidades: concluídas, em andamento, pendentes e sugeridas.
> Atualizado em: 2026-03-24

---

## Legenda

| Ícone | Status |
|---|---|
| ✅ | Concluído |
| 🔄 | Em andamento |
| 🟡 | Pendente (planejado) |
| 💡 | Ideia / Sugestão |
| ❌ | Cancelado / Descartado |

---

## ✅ Concluído

### v0.1.0 — Core Scanner (2026-03-23)

- ✅ **Scanner de diretórios** — `walkdir` com filtros por pasta, extensão e arquivo
- ✅ **Detecção de arquivos binários** — null byte check nos primeiros 1024 bytes
- ✅ **Contagem de LOC** — byte-scan por `0x0A`, sem overhead de UTF-8
- ✅ **Detecção de linguagem** — 40+ mapeamentos por extensão
- ✅ **Geração de `Tree.md`** — Markdown completo com árvore ASCII + código fonte
- ✅ **Geração de `Structure.json`** — Metadados em JSON para consumo por ferramentas
- ✅ **Geração de `Structure.txt`** — Árvore ASCII pura
- ✅ **Respeito ao `.gitignore`** — Glob-to-regex converter
- ✅ **`TreeC.toml`** — Configuração por arquivo com seções `[General]`, `[Exports]`, `[Ignore]`
- ✅ **Exclusões automáticas** — `Tree.md`, `Structure.*`, `treec.exe`, `.git` nunca incluídos
- ✅ **CLI com clap** — Roteador de comandos com `--help` detalhado

### v0.2.0 — Neural Link (2026-03-24)

- ✅ **`treec --neural-link`** — Cria `.brain/` com 17 arquivos via IA
- ✅ **Suporte a Gemini** — `gemini-2.0-flash` como padrão
- ✅ **Suporte a OpenAI** — `gpt-4.1-mini` como padrão
- ✅ **Suporte a Claude** — `claude-sonnet-4-20250514` como padrão
- ✅ **`treec --update-brain`** — Atualização incremental de `tree.md`, `memory.md`, `changelog.md`
- ✅ **`treec --config-neural <PROVIDER> <KEY>`** — Salva provedor + API key no `TreeC.toml`
- ✅ **`treec --neural-link-remove-api`** — Remove seção `[NeuralLink]` do `TreeC.toml`
- ✅ **`treec --config`** — Gera `TreeC.toml` padrão
- ✅ **`treec --obsidian`** — Configura vault Obsidian dentro de `.brain/`
- ✅ **`treec --clean`** — Remove `Tree.md`, `Structure.*`, `.brain/`
- ✅ **Retry com backoff** — 3 tentativas: 5s → 15s → 30s para erros 429
- ✅ **Mascaramento de API key** — Nunca exposta em mensagens de erro
- ✅ **Erros legíveis** — Parsing do body de erro de 400/401/403/404/429/5xx
- ✅ **Obsidian wikilinks** — Sistema de prompt pede `[[wikilinks]]` entre arquivos
- ✅ **`brain/` seed content** — Arquivos inicializados com conteúdo útil antes da IA
- ✅ **`prompt.md`** — Regras embutidas para agentes de IA trabalharem no projeto

### v0.3.0 — Robustez e Segurança (2026-03-24)

- ✅ **TOML parser com `toml` crate** — Substituiu parsing frágil por Regex. Respeita seções corretamente
- ✅ **`TREEC_API_KEY` env var** — Prioridade sobre `ApiKey` no arquivo. Evita exposição de credenciais no git
- ✅ **Detecção de binários aprimorada** — Dupla heurística: null bytes + >30% bytes não-imprimíveis
- ✅ **`IncludeHiddenDirs`** — Nova opção em `[General]` para incluir `.github`, `.vscode` etc.
- ✅ **`context.md` e `memory.md`** — Documentação de segundo cérebro na raiz do projeto

### v0.4.0 — UX, Diagnóstico e Distribuição (2026-03-24)

- ✅ **`treec --status`** — Diagnóstico completo: config, brain, API key, fonte da key, .gitignore check
- ✅ **Estimativa de tokens** — Aviso antes de enviar à IA: %uso do context window, abort se >90%
- ✅ **Auto-proteção .gitignore** — `--config-neural` adiciona `TreeC.toml` ao .gitignore automaticamente
- ✅ **Barra de progresso** — `indicatif` spinner + barra durante análise de arquivos
- ✅ **GitHub Actions release** — `.github/workflows/release.yml` para binários multiplataforma
- ✅ **Cargo.toml para crates.io** — `license`, `keywords`, `categories`, `readme` adicionados
- ✅ **`roadmap.md`** — Roadmap completo do projeto com 35+ itens

### v0.5.0 — Scanner, Paralelismo, Ollama, Custo e CI (2026-03-24)

- ✅ **`ignore` crate** — Scanner reescrito. gitignore completo com negação (`!`), anchoring, `.git/info/exclude`
- ✅ **Suporte a regras de negação no `.gitignore`** — Coberto pelo `ignore` crate; semântica completa do gitignore
- ✅ **Rayon parallelism** — Análise de arquivos paralela com `rayon::par_iter()`. Speedup linear com nº de cores
- ✅ **Estimativa de custo USD** — Custo estimado em USD exibido antes de toda chamada à API
- ✅ **Ollama (modelos locais)** — `treec --config-neural ollama llama3.2`. Sem API key, sem nuvem. Funciona com llama3.2, mistral, qwen2.5
- ✅ **`--brain-files` seletivo** — `treec --neural-link --brain-files context,tasks`. Regenera só o necessário; reduz custo de API
- ✅ **GitHub Actions CI** — `.github/workflows/ci.yml`: fmt + clippy + build em 3 plataformas (ubuntu, windows, macos)
- ✅ **Removido `regex` e `walkdir`** — Dependências substituídas por `ignore` + `rayon`
- ✅ **CI quality fixes** — `cargo fmt` aplicado; 5 warnings de clippy corrigidos (`dead_code`, `if_same_then_else`, `needless_borrows_for_generic_args`, `collapsible_if`, `too_many_arguments`)

---

## 🔄 Em andamento

*(Nenhuma tarefa em progresso no momento)*

---

## 🟡 Pendente — Próximas versões

### v0.6.0 — Robustez, Análise e Distribuição

---

#### Chunking / sumarização para repositórios grandes
**Descrição:** Para projetos com Tree.md > X tokens, ao invés de enviar tudo, enviar apenas: árvore de diretórios + assinaturas de funções/structs + comentários. O código completo ficaria acessível por demanda.
**Prioridade:** Alta
**Dificuldade:** Alta
**Impacto:** Permite uso em repositórios médios/grandes sem exceder context window

---

#### Análise de dependências
**Descrição:** Detectar e listar dependências do projeto (`Cargo.toml`, `package.json`, `requirements.txt`, `go.mod` etc.) e incluir no `Structure.json` e `Tree.md`.
**Prioridade:** Média
**Dificuldade:** Média
**Impacto:** Enriquece o contexto enviado para a IA; melhora documentação gerada

---

#### Modo diff entre scans
**Descrição:** `treec --diff` — compara o estado atual do projeto com o último `Tree.md` gerado e mostra arquivos adicionados/removidos/modificados.
**Prioridade:** Média
**Dificuldade:** Alta
**Impacto:** Útil para tracking de mudanças sem IA

---

#### Publicar no crates.io
**Descrição:** Preparar `Cargo.toml` com todos os metadados (license, categories, keywords) e publicar a crate em crates.io para `cargo install treec`.
**Prioridade:** Alta
**Dificuldade:** Baixa
**Impacto:** Distribuição padronizada; visibilidade no ecossistema Rust

---

#### Modo `--dry-run`
**Descrição:** `treec --neural-link --dry-run` — simula o que seria enviado à IA (exibe prompt + estimativa de tokens), mas não faz a chamada de API.
**Prioridade:** Média
**Dificuldade:** Baixa
**Impacto:** Permite ao usuário revisar o contexto antes de gastar créditos de API

---

### v0.7.0 — Scanner Avançado e Exports

---

#### Contagem de funções/structs/classes
**Descrição:** No `analyzer.rs`, usar regex simples para contar entidades por arquivo: `fn `, `struct `, `impl `, `class `, `def `, `function `. Exibir no resumo e no `Structure.json`.
**Prioridade:** Baixa
**Dificuldade:** Média
**Impacto:** Métricas de complexidade do projeto mais ricas

---

#### Export para HTML
**Descrição:** Geração de `Structure.html` com árvore interativa (expansível/colapsável) e syntax highlighting.
**Prioridade:** Baixa
**Dificuldade:** Alta
**Impacto:** Documentação web navegável sem necessidade de Obsidian

---

#### Export comprimido
**Descrição:** Opção `--compress` para gerar `Tree.md.gz` ou `.zip`. Útil para repositórios grandes ao compartilhar contexto.
**Prioridade:** Baixa
**Dificuldade:** Baixa
**Impacto:** Facilita envio de contexto por e-mail ou chat

---

#### Perfis de configuração
**Descrição:** Suporte a múltiplos perfis no `TreeC.toml` — ex: `[Profile.ci]`, `[Profile.local]`. Ativar com `treec --profile ci`.
**Prioridade:** Baixa
**Dificuldade:** Média
**Impacto:** Permite configurações diferentes para CI/CD e uso local

---

#### `.treecignore`
**Descrição:** Suporte a um arquivo `.treecignore` separado (além do `TreeC.toml`), similar ao `.prettierignore`. Facilita exclusões sem modificar a config principal.
**Prioridade:** Baixa
**Dificuldade:** Baixa
**Impacto:** Melhor ergonomia para projetos com muitas exclusões customizadas

---

### v0.8.0+ — Integrações e UX

---

#### Modo `--watch`
**Descrição:** `treec --watch` — monitora o diretório e regenera automaticamente `Tree.md` (e opcionalmente atualiza o brain) quando arquivos mudam.
**Prioridade:** Média
**Dificuldade:** Alta
**Impacto:** Living documentation — sempre sincronizado com o código

---

#### Integração com CI/CD
**Descrição:** Documentar e criar exemplos de uso em GitHub Actions, GitLab CI, e scripts de deploy. `treec --update-brain` como step de CD.
**Prioridade:** Média
**Dificuldade:** Baixa
**Impacto:** Documentação contínua automática integrada ao fluxo de desenvolvimento

---

#### Cores configuráveis
**Descrição:** Adicionar suporte a `NO_COLOR` e opção `Colors = false` no `TreeC.toml` para desabilitar output colorido (útil em CI e terminais sem suporte).
**Prioridade:** Baixa
**Dificuldade:** Baixa
**Impacto:** Melhor compatibilidade com automações e pipes

---

#### Suporte a múltiplos diretórios
**Descrição:** `treec src/ lib/ tests/` — escanear múltiplos diretórios em uma só execução e gerar documentação unificada.
**Prioridade:** Baixa
**Dificuldade:** Média
**Impacto:** Útil para monorepos com estruturas não-convencionais

---

#### Instalador Windows (MSI / winget)
**Descrição:** Criar pacote para winget ou scoop para facilitar instalação no Windows sem precisar do Rust toolchain.
**Prioridade:** Baixa
**Dificuldade:** Alta
**Impacto:** Maior adoção por usuários não-Rust no Windows

---

#### Auto-update
**Descrição:** `treec --update` — verifica se há nova versão no GitHub Releases e oferece atualização automática.
**Prioridade:** Baixa
**Dificuldade:** Alta
**Impacto:** Melhor DX para usuários do binário

---

### v1.0.0 — Estável e Polido

---

#### Detecção de encoding não-UTF-8
**Descrição:** Para arquivos que falham em `read_to_string`, tentar detectar encoding (Latin-1, UTF-16) e converter antes de incluir no `Tree.md`.
**Prioridade:** Baixa
**Dificuldade:** Alta
**Impacto:** Suporte a projetos legados com encoding não-padrão

---

#### Modo interativo (TUI)
**Descrição:** `treec --tui` — interface de terminal interativa (usando `ratatui`) com preview em tempo real da árvore, seleção de arquivos a incluir/excluir, e configuração visual.
**Prioridade:** Baixa
**Dificuldade:** Muito Alta
**Impacto:** Onboarding mais fácil para novos usuários

---

## 💡 Ideias Futuras (sem versão definida)

### Integrações com Ferramentas Externas

- **Plugin para VS Code** — Integração nativa como extensão, com sidebar mostrando o `.brain/` atual
- **Plugin para Neovim/Zed** — Acesso ao contexto do projeto diretamente no editor
- **Integração com GitHub Copilot Workspace** — Exportar `.brain/` como contexto base para o Copilot
- **Integração com Cursor** — `.brain/` como base de conhecimento para o AI chat do Cursor
- **Webhook** — `treec --webhook <URL>` para notificar sistemas externos após gerar o brain
- **Export para Notion** — Enviar `.brain/` para páginas do Notion via API

### Análise Avançada de Código

- **Análise de complexidade ciclomática** — Identificar funções complexas automaticamente
- **Detecção de código duplicado** — Apontar copy-paste entre módulos
- **Análise de cobertura de testes** — Integrar com `cargo test --coverage` e incluir no brain
- **Detecção de TODO/FIXME/HACK** — Extrair e centralizar em `tasks.md` automaticamente
- **Análise de imports** — Mapear grafo de dependências entre módulos internos
- **Security scan básico** — Detectar padrões comuns de vulnerabilidade (hardcoded secrets, SQL concat)

### Colaboração

- **Compartilhamento de brain** — `treec --share` para gerar link público temporário do `.brain/`
- **Sync entre máquinas** — Integração com git para versionar o `.brain/` de forma colaborativa
- **Comentários no brain** — Sistema de anotações humanas que sobrevivem ao `--update-brain`

---

## Prioridade Consolidada

| Prioridade | Item                                              | Versão alvo | Status |
| ---------- | ------------------------------------------------- | ----------- | ------ |
| ✅ Feito   | Estimativa de tokens antes do envio               | v0.4.0      | ✅     |
| ✅ Feito   | Aviso + auto-gitignore para API key               | v0.4.0      | ✅     |
| ✅ Feito   | GitHub Releases automatizadas (CI binários)       | v0.4.0      | ✅     |
| ✅ Feito   | Comando `treec --status`                          | v0.4.0      | ✅     |
| ✅ Feito   | Barra de progresso (`indicatif`)                  | v0.4.0      | ✅     |
| ✅ Feito   | Estimativa de custo USD                           | v0.5.0      | ✅     |
| ✅ Feito   | Suporte a modelos locais (Ollama)                 | v0.5.0      | ✅     |
| ✅ Feito   | Regeneração seletiva (`--brain-files`)            | v0.5.0      | ✅     |
| ✅ Feito   | Análise paralela com `rayon`                      | v0.5.0      | ✅     |
| ✅ Feito   | Suporte completo ao `.gitignore` (`ignore` crate) | v0.5.0      | ✅     |
| ✅ Feito   | GitHub Actions CI (fmt + clippy + build)          | v0.5.0      | ✅     |
| ✅ Feito   | CI quality fixes (fmt + 5 clippy warnings)        | v0.5.0      | ✅     |
| 🔴 Alta    | Publicar no crates.io                             | v0.6.0      | 🟡     |
| 🔴 Alta    | Chunking para repositórios grandes                | v0.6.0      | 🟡     |
| 🟠 Média   | Análise de dependências (Cargo/npm/pip)           | v0.6.0      | 🟡     |
| 🟠 Média   | Modo diff entre scans                             | v0.6.0      | 🟡     |
| 🟠 Média   | Modo `--dry-run`                                  | v0.6.0      | 🟡     |
| 🟡 Baixa   | Export para HTML                                  | v0.7.0      | 🟡     |
| 🟡 Baixa   | Modo `--watch`                                    | v0.8.0      | 🟡     |
| 🟡 Baixa   | TUI interativa                                    | v1.0.0      | 🟡     |

---

*Última atualização: 2026-03-24 (v0.5.0 concluído + CI fixes) | Claude Code*
*Ver também: [[context]] | [[memory]] | [[changelog]]*
