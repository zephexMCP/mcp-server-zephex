To use **Zephex** in Zed:

1. Create an account at [zephex.dev](https://zephex.dev) (free tier available).
2. Open [API keys](https://zephex.dev/dashboard/api-keys) and create a key.
3. Install this extension: command palette → `zed: extensions` → search **Zephex** → Install.
4. In Zed settings JSON, set:

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

5. Run `zed: reload context servers`. Open the Agent Panel — ten hosted tools should be available.

The extension installs the official `zephex` npm package via Zed’s managed Node runtime (stdio MCP). Tool calls go to `https://zephex.dev/mcp` with your key.

Optional terminal CLI (same account):  
`curl -fsSL https://zephex.dev/cli/install.sh | bash` then `cd your-project && zephex login`.

Docs: https://zephex.dev/docs
