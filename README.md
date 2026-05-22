# Zephex MCP Server — Zed Extension

[![Zed Extension](https://img.shields.io/badge/Zed-Extension-blueviolet?logo=zed)](https://zed.dev/extensions)
[![npm](https://img.shields.io/npm/v/zephex.svg?label=zephex)](https://www.npmjs.com/package/zephex)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

One-click access to **[Zephex](https://zephex.dev)** — a hosted MCP server with 10 developer tools — directly inside Zed's Agent Panel. No local server to run, no Docker, no port management. Just an API key.

## What you get

- **Project intelligence** — AST-aware code reading, BM25-ranked search, full architecture diagrams
- **Package safety** — live npm / PyPI / Cargo / Maven / NuGet CVE checks and deep migration guidance
- **Security audits** — TLS, CSP, cookies, redirects on any URL with copy-paste fix snippets
- **Persistent reasoning** — long-running thinking sessions with drift detection and cross-session memory
- **Curated knowledge base** — answers across databases, security, frontend, backend, auth, mobile

Files stay on your machine. Only tool calls go out to `https://zephex.dev/mcp`.

## Setup

### 1. Get an API key

Sign up at **[zephex.dev](https://zephex.dev)** (free tier: 300 requests / month). Open the [dashboard](https://zephex.dev/dashboard) and copy your key — it starts with `mcp_`.

### 2. Install this extension

In Zed: `cmd-shift-p` → `zed: extensions` → search **Zephex** → **Install**.

### 3. Add your key to `settings.json`

`cmd-,` (Zed settings) then click the `{}` icon to edit JSON. Paste:

```json
{
  "context_servers": {
    "mcp-server-zephex": {
      "settings": {
        "zephex_api_key": "mcp_your_key_here"
      }
    }
  }
}
```

### 4. Reload

`cmd-shift-p` → `zed: reload context servers`. Open the Agent Panel — Zephex's 10 tools are now available.

## Tools

| Tool | What it does |
|---|---|
| `get_project_context` | Detects framework, package manager, build/test/dev/lint commands, env vars, deps, monorepo layout, entry points |
| `read_code` | AST-extracted symbol or full-file reading with token budgets, outline mode, and pagination |
| `find_code` | BM25-ranked search with AST-aware enclosing-block context, multi-query fan-out, exhaustive mode |
| `explain_architecture` | End-to-end architecture analysis with Mermaid diagrams (sequence, service, C4) and health scoring |
| `scope_task` | Plain-English task → minimal focus file set with risk assessment and reusable utilities |
| `thinking` | Persistent reasoning sessions with drift detection, revision tracking, branching hypotheses |
| `audit_headers` | HTTP / TLS / cookie / redirect security audit with fix snippets for Nginx, Caddy, Vercel, Cloudflare, etc. |
| `check_package` | Live npm / PyPI / Cargo / Maven / NuGet / Pub / Hex / CocoaPods / SPM version + supply-chain signals |
| `audit_package` | Deep package intelligence — breaking changes, CVEs, real migration steps and code diff examples |
| `Zephex_dev_info` | Expert knowledge base across 6 domains: databases, security, frontend, backend, auth, mobile |

## Pricing

| Tier | Requests / month | Price |
|---|---|---|
| Free | 300 | $0 |
| Pro | 3,000 | $7 / mo |
| Max | 10,000 | $19 / mo |

[Manage your plan](https://zephex.dev/dashboard).

## How it works

This extension is a thin Rust → WebAssembly shim that:

1. Reads `zephex_api_key` from your Zed settings
2. Uses Zed's bundled Node.js runtime to install the [`zephex` npm package](https://www.npmjs.com/package/zephex) (the actual MCP stdio server)
3. Spawns the server with your API key injected as `ZEPHEX_API_KEY`

The MCP server handles JSON-RPC, keeps file reads local, and proxies tool calls to `https://zephex.dev/mcp`. No long-lived process, no port allocation, no admin permissions.

## Compatibility

- Zed 0.190+ (extension API 0.7.0)
- macOS, Linux, Windows
- Bundled Node.js runtime — no system Node required

## Troubleshooting

**`Missing zephex_api_key`** — your `settings.json` is missing the key or has it under the wrong path. Confirm the structure: `context_servers > mcp-server-zephex > settings > zephex_api_key`. Run `zed: reload context servers` after edits.

**Tool call returns `401 Unauthorized`** — API key is invalid or expired. Get a fresh one at [zephex.dev/dashboard](https://zephex.dev/dashboard).

**Extension fails to start** — open `cmd-shift-p` → `zed: open log` and search for `zephex` or `extension`. The most common cause is a stale Node install in Zed's cache; reinstalling the extension fixes it.

## Links

- [Documentation](https://zephex.dev/docs)
- [Dashboard](https://zephex.dev/dashboard)
- [npm package — zephex](https://www.npmjs.com/package/zephex)
- [Report an issue](https://github.com/zephexMCP/mcp-server-zephex/issues)

## License

[MIT](LICENSE) © Zephex
