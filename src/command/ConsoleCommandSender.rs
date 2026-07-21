// ConsoleCommandSender — the server console

use super::command_sender::CommandSender;

pub struct ConsoleCommandSender;

impl CommandSender for ConsoleCommandSender {
    fn send_message(&self, message: &str) {
        log::info!("[Console] {}", message);
    }

    fn name(&self) -> String {
        "Console".to_string()
    }

    fn has_permission(&self, _permission: &str) -> bool {
        true // Console has all permissions
    }

    fn is_op(&self) -> bool {
        true
    }
}
