// CommandMap — central registry for all commands
// Inspired by PMMP's SimpleCommandMap.php

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use super::command_base::Command;
use super::command_sender::CommandSender;

pub struct CommandMap {
    known_commands: RwLock<HashMap<String, Arc<Command>>>,
    fallback_prefix: String,
}

impl CommandMap {
    pub fn new(fallback_prefix: &str) -> Self {
        Self {
            known_commands: RwLock::new(HashMap::new()),
            fallback_prefix: fallback_prefix.to_string(),
        }
    }

    /// Register a command with a fallback prefix
    pub fn register(&self, prefix: &str, mut command: Command) -> bool {
        let label = format!("{}:{}", prefix, command.name());
        command.set_label(label.clone());
        command.set_registered(true);

        let mut known = self.known_commands.write().unwrap();
        known.insert(label, Arc::new(command.clone()));
        known.insert(command.name().to_string(), Arc::new(command));
        true
    }

    /// Register multiple commands under the same prefix
    pub fn register_all(&self, prefix: &str, commands: Vec<Command>) {
        for cmd in commands {
            self.register(prefix, cmd);
        }
    }

    /// Get a command by name
    pub fn get_command(&self, name: &str) -> Option<Arc<Command>> {
        let known = self.known_commands.read().unwrap();
        known.get(name).cloned()
    }

    /// Dispatch a command string
    pub fn dispatch(&self, sender: &dyn CommandSender, command_line: &str) -> bool {
        let parts: Vec<&str> = command_line.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        let cmd_name = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        match self.get_command(cmd_name) {
            Some(cmd) => {
                if !cmd.test_permission(sender) {
                    sender.send_message("You don't have permission to run this command.");
                    return false;
                }
                cmd.execute(sender, cmd_name, &args)
            }
            None => {
                sender.send_message(&format!("Unknown command: {}", cmd_name));
                false
            }
        }
    }

    /// Get all registered commands
    pub fn get_commands(&self) -> Vec<Arc<Command>> {
        let known = self.known_commands.read().unwrap();
        known.values().cloned().collect()
    }

    /// Unregister a command
    pub fn unregister(&self, name: &str) -> bool {
        let mut known = self.known_commands.write().unwrap();
        known.remove(name).is_some()
    }

    /// Clear all commands
    pub fn clear(&self) {
        let mut known = self.known_commands.write().unwrap();
        known.clear();
    }

    pub fn fallback_prefix(&self) -> &str {
        &self.fallback_prefix
    }
}
