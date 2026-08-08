# Zephex — hosted MCP (Zed extension)

[![Website](https://img.shields.io/badge/Website-zephex.dev-111111?style=flat-square)](https://zephex.dev)
[![MCP](https://img.shields.io/badge/MCP-zephex.dev%2Fmcp-00c853?style=flat-square)](https://zephex.dev/mcp)
[![npm](https://img.shields.io/npm/v/zephex.svg?label=npm%20zephex&style=flat-square)](https://www.npmjs.com/package/zephex)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)

**Zephex** is a **hosted MCP server** for AI coding agents: one HTTPS endpoint, one API key, ten tools that read *your* repository instead of guessing from training data.

This repository is the **Zed** extension that connects Zed’s Agent Panel to that hosted server.  
Product home: **[zephex.dev](https://zephex.dev)** · Endpoint: **`https://zephex.dev/mcp`**

---

## What Zephex is

| Piece | Role |
|-------|------|
| **Hosted MCP** | Tools over HTTPS at `https://zephex.dev/mcp` |
| **This extension** | One-click wiring inside **Zed** (stdio via the `zephex` npm package + your API key) |
| **Terminal CLI** | Same tools in a real shell (`zephex` / install script) — optional |

Agents call tools on demand (project context, search, read, architecture, tests, packages, URL audit, memory, reasoning, playbooks). Local file access stays with the client process; tool requests go to the hosted API with your key.

Not a local Docker stack you have to babysit. Not a second “mystery” server URL — use **`https://zephex.dev/mcp`**.

---

## Quick start in Zed

### 1. API key

1. Sign up at [zephex.dev](https://zephex.dev)  
2. Create a key at [zephex.dev/dashboard/api-keys](https://zephex.dev/dashboard/api-keys)  
3. Keys look like `mcp_…` (formats evolve — copy from the dashboard)

Free tier available. Current limits and paid plans: always check the [dashboard](https://zephex.dev/dashboard) / pricing on the site.

### 2. Install the extension

In Zed: command palette → **`zed: extensions`** → search **Zephex** → **Install**.

### 3. Settings

Open settings JSON (`cmd-,` / `ctrl-,`, then `{}`) and set:

```json
{
  "context_servers": {
    "mcp-server-zephex": {
      "settings": {
        "zephex_api_key": "YOUR_KEY_FROM_DASHBOARD"
      }
    }
  }
}
```

### 4. Reload

Command palette → **`zed: reload context servers`**.  
Open the Agent Panel — the ten Zephex tools should appear.

---

## The 10 tools (current product)

These are the **only** tool names. Older docs sometimes listed removed names — ignore those.

| Tool | What it does |
|------|----------------|
| **`get_project_context`** | Framework, scripts, auth, env, monorepo topics — start here on a new repo |
| **`find_code`** | Ranked search (snippet / symbol / concept / everywhere) |
| **`read_code`** | AST symbol, file batch, outline, local call-graph modes |
| **`explain_architecture`** | How modules wire; deep mode for richer flows |
| **`check_package`** | Registry safety, CVEs, upgrades (multiple ecosystems; use tasks) |
| **`check_test`** | Test Pulse — run suite, failures, fix prompt, missing tests |
| **`audit_headers`** | Live HTTPS security grade for a URL **you** provide |
| **`project_memory`** | remember / recall facts across sessions |
| **`keep_thinking`** | Structured multi-step reasoning + loop detection |
| **`Zephex_dev_info`** | Expert playbooks (generic patterns — not private repo code) |

**Sensible call order**

```text
get_project_context → find_code → read_code → [edit] → check_test
         ↘ explain_architecture when the change spans modules
```

**Removed / do not use:** `scope_task`, `inspect_url`, `audit_package`, bare `thinking`  
(Package upgrades → `check_package` with the upgrade/security task. Stuck debug → `keep_thinking`.)

Agent routing skill (any skill-aware agent setup):

```bash
npx skills add zephexMCP/agent-skills --skill zephex
```

---

## MCP (hosted) in one glance

| | |
|--|--|
| Endpoint | `https://zephex.dev/mcp` |
| Auth | Bearer API key from the dashboard |
| Project input | Local path (via client), public `github:owner/repo`, or `inline_files` |
| Docs | [zephex.dev/docs](https://zephex.dev/docs) |

Generic HTTP shape (for clients that take a URL + header — not required for this Zed extension):

```json
{
  "url": "https://zephex.dev/mcp",
  "headers": {
    "Authorization": "Bearer YOUR_API_KEY"
  }
}
```

This extension instead spawns the official **`zephex`** npm package over stdio with `ZEPHEX_API_KEY` set (Zed-managed Node).

---

## Optional: terminal CLI (same account)

Same key and tools, different surface — human text, project cwd, `deep --json` for agents.

```bash
# Mac / Linux
curl -fsSL https://zephex.dev/cli/install.sh | bash

# Windows PowerShell
irm https://zephex.dev/install.ps1 | iex

cd /path/to/your-project
zephex login
zephex overview
zephex deep --json
zephex find "auth"
zephex test
```

| CLI (examples) | Maps toward |
|----------------|-------------|
| `overview` / `deep` | Project orientation (`get_project_context` + more) |
| `find` / `read` | `find_code` / `read_code` |
| `architecture` | `explain_architecture` |
| `safe` / `check-package` | `check_package` |
| `test` / `check test …` | `check_test` |
| `check url …` | `audit_headers` |

Full command map: [github.com/zephexMCP/zephex-cli](https://github.com/zephexMCP/zephex-cli) · [docs/cli-commands](https://zephex.dev/docs/cli-commands)

Browser Mode 2 (no local install): [dashboard/terminal](https://zephex.dev/dashboard/terminal) · [zephex-web-terminal](https://github.com/zephexMCP/zephex-web-terminal)

---

## How this extension works

Thin Rust / Wasm extension for Zed that:

1. Reads `zephex_api_key` from Zed settings  
2. Uses Zed’s bundled Node runtime to ensure the [`zephex`](https://www.npmjs.com/package/zephex) package is available  
3. Starts the MCP **stdio** server with your key as `ZEPHEX_API_KEY`  
4. Tool calls are authenticated against the hosted API  

No separate Docker container. No manual port. No system-wide Node required for the extension path.

---

## Compatibility

- **Zed** 0.190+ (extension API as declared in `extension.toml`)  
- macOS, Linux, Windows  
- Zed-bundled Node for the MCP process  

---

## Troubleshooting

| Symptom | What to check |
|---------|----------------|
| Missing API key | `context_servers → mcp-server-zephex → settings → zephex_api_key` then reload context servers |
| `401` / unauthorized | New key from [dashboard](https://zephex.dev/dashboard/api-keys) |
| Tools missing | Reload context servers; reinstall extension if Node cache is stale |
| Logs | Command palette → `zed: open log` — search `zephex` |

---

## Related public pages

| | |
|--|--|
| Product / docs | [zephex.dev](https://zephex.dev) · [docs](https://zephex.dev/docs) |
| MCP product overview | [zephexMCP/zephex-MCPs](https://github.com/zephexMCP/zephex-MCPs) |
| Terminal CLI | [zephexMCP/zephex-cli](https://github.com/zephexMCP/zephex-cli) |
| Web terminal | [zephexMCP/zephex-web-terminal](https://github.com/zephexMCP/zephex-web-terminal) |
| Agent skill | [zephexMCP/agent-skills](https://github.com/zephexMCP/agent-skills) |
| npm | [zephex](https://www.npmjs.com/package/zephex) |
| Issues | [this repo’s issues](https://github.com/zephexMCP/mcp-server-zephex/issues) |

---

## License

[MIT](LICENSE) for this extension repository.  
The hosted Zephex service is a commercial product; see [zephex.dev](https://zephex.dev) for terms and privacy.
