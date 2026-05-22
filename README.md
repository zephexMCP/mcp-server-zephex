# Zephex MCP Server — Zed Extension

One-click access to [Zephex](https://zephex.dev) — a hosted, remote MCP server providing 10 developer tools to Zed's Agent Panel with zero local setup.

## Setup

1. Install this extension from the Zed Extensions marketplace
2. Get your API key at [zephex.dev/dashboard](https://zephex.dev/dashboard)
3. Open Zed `settings.json` (`cmd-,` then click "Open Settings File") and add:

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

4. Restart Zed. The Agent Panel will show Zephex's tools.

## Tools provided

| Tool | What it does |
|---|---|
| `get_project_context` | Auto-detects framework, package manager, scripts, env vars, deps |
| `read_code` | AST-extracted symbol or full-file reading with token budgets |
| `find_code` | BM25-ranked search with AST-aware enclosing-block context |
| `explain_architecture` | End-to-end architecture analysis with Mermaid diagrams |
| `scope_task` | Plain-English task → minimal focus file set with risk assessment |
| `thinking` | Persistent reasoning session with drift detection and revision tracking |
| `audit_headers` | HTTP / TLS / cookie / redirect security audit with fix snippets |
| `check_package` | Live npm / PyPI / Cargo / Maven / etc. version + supply-chain check |
| `audit_package` | Deep package intelligence — breaking changes, CVEs, migration steps |
| `Zephex_dev_info` | Expert developer knowledge base across 6 categories |

## Pricing

| Tier | Requests / month | Price |
|---|---|---|
| Free | 300 | $0 |
| Pro | 3,000 | $7 / month |
| Max | 10,000 | $19 / month |

## Links

- [Documentation](https://zephex.dev/docs)
- [Dashboard](https://zephex.dev/dashboard)
- [Source for the underlying server](https://www.npmjs.com/package/zephex)

## License

MIT
