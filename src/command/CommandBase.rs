// Command — base class for all commands
// Inspired by PMMP's Command.php

use super::command_sender::CommandSender;

#[derive(Clone)]
pub struct Command {
    name: String,
    description: String,
    usage: String,
    aliases: Vec<String>,
    permissions: Vec<String>,
    label: String,
    registered: bool,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            usage: String::new(),
            aliases: Vec::new(),
            permissions: Vec::new(),
            label: name.to_string(),
            registered: false,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_usage(mut self, usage: &str) -> Self {
        self.usage = usage.to_string();
        self
    }

    pub fn with_aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn with_permission(mut self, perm: &str) -> Self {
        self.permissions.push(perm.to_string());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn usage(&self) -> &str {
        &self.usage
    }

    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    pub fn permissions(&self) -> &[String] {
        &self.permissions
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn is_registered(&self) -> bool {
        self.registered
    }

    pub fn set_registered(&mut self, registered: bool) {
        self.registered = registered;
    }

    /// Test if the target has permission for this command
    pub fn test_permission(&self, target: &dyn CommandSender) -> bool {
        if self.permissions.is_empty() {
            return true;
        }
        self.permissions.iter().any(|p| target.has_permission(p))
    }

    /// Execute the command — override in subclasses
    pub fn execute(&self, _sender: &dyn CommandSender, _label: &str, _args: &[String]) -> bool {
        false
    }
}

/// Command owned by a plugin — delegates execution to a callback
pub struct PluginCommand {
    base: Command,
    owner_name: String,
}

impl PluginCommand {
    pub fn new(name: &str, owner_name: &str) -> Self {
        Self {
            base: Command::new(name),
            owner_name: owner_name.to_string(),
        }
    }

    pub fn base(&self) -> &Command {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Command {
        &mut self.base
    }

    pub fn owner_name(&self) -> &str {
        &self.owner_name
    }
}
