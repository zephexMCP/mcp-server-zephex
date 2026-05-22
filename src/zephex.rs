use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "zephex";
const SERVER_PATH: &str = "node_modules/zephex/dist/cli.js";

/// Settings the user provides in their Zed `settings.json` under
/// `context_servers."mcp-server-zephex".settings`.
#[derive(Debug, Deserialize, JsonSchema)]
struct ZephexContextServerSettings {
    /// Your Zephex API key. Get one at https://zephex.dev/dashboard
    zephex_api_key: String,
}

struct ZephexExtension;

impl zed::Extension for ZephexExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // 1. Read the user's API key from their Zed settings.
        let settings = ContextServerSettings::for_project("mcp-server-zephex", project)?;
        let Some(settings_value) = settings.settings else {
            return Err(
                "Missing `zephex_api_key` in settings. Add your API key from \
                 https://zephex.dev/dashboard to your Zed settings.json under \
                 context_servers.\"mcp-server-zephex\".settings.zephex_api_key"
                    .into(),
            );
        };
        let parsed: ZephexContextServerSettings =
            serde_json::from_value(settings_value).map_err(|e| e.to_string())?;

        if parsed.zephex_api_key.trim().is_empty()
            || parsed.zephex_api_key == "YOUR_ZEPHEX_API_KEY"
        {
            return Err(
                "Replace YOUR_ZEPHEX_API_KEY in settings.json with your real API key from \
                 https://zephex.dev/dashboard"
                    .into(),
            );
        }

        // 2. Ensure the latest `zephex` npm package is installed in Zed's managed
        //    node_modules cache. Zed handles caching across launches — these calls
        //    are no-ops when already up to date.
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(latest_version.as_str()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        // 3. Build the absolute path to the installed CLI entry point.
        //    `dist/cli.js` matches the `bin` field in zephex's package.json.
        let server_path = std::env::current_dir()
            .map_err(|e| format!("failed to read current dir: {e}"))?
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        // 4. Spawn it with Zed's bundled Node and inject ZEPHEX_API_KEY.
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path],
            env: vec![("ZEPHEX_API_KEY".to_string(), parsed.zephex_api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ZephexContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ZephexExtension);
