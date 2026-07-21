// PluginDescription — parses plugin.toml manifest
// TOML format instead of YAML

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PluginDescription {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    main: String,               // Main Python class (e.g. "src/Main.py")
    api: String,                // API version (e.g. "1.0.0")
    depend: Vec<String>,
    softdepend: Vec<String>,
    loadbefore: Vec<String>,
    website: Option<String>,
    prefix: Option<String>,
    commands: HashMap<String, CommandDesc>,
    permissions: HashMap<String, PermissionDesc>,
}

#[derive(Debug, Clone)]
pub struct CommandDesc {
    pub description: String,
    pub usage: Option<String>,
    pub permission: Option<String>,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PermissionDesc {
    pub description: String,
    pub default: String,
}

impl PluginDescription {
    pub fn name(&self) -> &str { &self.name }
    pub fn version(&self) -> &str { &self.version }
    pub fn description(&self) -> &str { &self.description }
    pub fn authors(&self) -> &[String] { &self.authors }
    pub fn main(&self) -> &str { &self.main }
    pub fn api(&self) -> &str { &self.api }
    pub fn depend(&self) -> &[String] { &self.depend }
    pub fn softdepend(&self) -> &[String] { &self.softdepend }
    pub fn loadbefore(&self) -> &[String] { &self.loadbefore }
    pub fn website(&self) -> Option<&str> { self.website.as_deref() }
    pub fn prefix(&self) -> Option<&str> { self.prefix.as_deref() }
    pub fn commands(&self) -> &HashMap<String, CommandDesc> { &self.commands }
    pub fn permissions(&self) -> &HashMap<String, PermissionDesc> { &self.permissions }

    pub fn from_toml(toml_str: &str) -> Result<Self, String> {
        let value: toml::Value = toml::from_str(toml_str)
            .map_err(|e| format!("Error parsing plugin.toml: {}", e))?;

        let root = value.get("plugin").or(Some(&value));

        let name = root.and_then(|r| r.get("name"))
            .and_then(|v| v.as_str())
            .ok_or("plugin.toml: 'name' is required")?
            .to_string();

        let version = root.and_then(|r| r.get("version"))
            .and_then(|v| v.as_str())
            .ok_or("plugin.toml: 'version' is required")?
            .to_string();

        let description = root.and_then(|r| r.get("description"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let authors = root.map(|r| Self::parse_string_array(r, "authors")).unwrap_or_default();

        let main = root.and_then(|r| r.get("main"))
            .and_then(|v| v.as_str())
            .ok_or("plugin.toml: 'main' is required (path to main .py file)")?
            .to_string();

        let api = root.and_then(|r| r.get("api"))
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .to_string();

        let depend = root.map(|r| Self::parse_string_array(r, "depend")).unwrap_or_default();
        let softdepend = root.map(|r| Self::parse_string_array(r, "softdepend")).unwrap_or_default();
        let loadbefore = root.map(|r| Self::parse_string_array(r, "loadbefore")).unwrap_or_default();

        let website = root.and_then(|r| r.get("website")).and_then(|v| v.as_str()).map(String::from);
        let prefix = root.and_then(|r| r.get("prefix")).and_then(|v| v.as_str()).map(String::from);

        let mut commands = HashMap::new();
        if let Some(cmds_table) = value.get("commands").and_then(|v| v.as_table()) {
            for (cmd_name, cmd_val) in cmds_table {
                let cmd = CommandDesc {
                    description: cmd_val.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    usage: cmd_val.get("usage").and_then(|v| v.as_str()).map(String::from),
                    permission: cmd_val.get("permission").and_then(|v| v.as_str()).map(String::from),
                    aliases: Self::parse_string_array(cmd_val, "aliases"),
                };
                commands.insert(cmd_name.clone(), cmd);
            }
        }

        let mut permissions = HashMap::new();
        if let Some(perms_table) = value.get("permissions").and_then(|v| v.as_table()) {
            for (perm_name, perm_val) in perms_table {
                let perm = PermissionDesc {
                    description: perm_val.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    default: perm_val.get("default").and_then(|v| v.as_str()).unwrap_or("op").to_string(),
                };
                permissions.insert(perm_name.clone(), perm);
            }
        }

        Ok(Self {
            name, version, description, authors, main, api,
            depend, softdepend, loadbefore, website, prefix,
            commands, permissions,
        })
    }

    fn parse_string_array(value: &toml::Value, key: &str) -> Vec<String> {
        value.get(key)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default()
    }
}
