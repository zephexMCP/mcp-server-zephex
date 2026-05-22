To use Zephex with Zed:

1. Sign up at [zephex.dev](https://zephex.dev) (free tier available — 300 requests/month)
2. Open your [dashboard](https://zephex.dev/dashboard) and click **Create API Key**
3. Copy the key (it starts with `mcp_`)
4. Replace `YOUR_ZEPHEX_API_KEY` in your Zed `settings.json` with the key you just copied
5. Restart Zed (or run `zed: reload context servers` from the command palette)

The extension will automatically install the `zephex` npm package on first launch via Zed's managed Node runtime — no separate install step required.
