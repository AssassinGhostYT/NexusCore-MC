// CommandSender — interface for anything that can send commands

pub trait CommandSender: Send + Sync {
    fn send_message(&self, message: &str);
    fn name(&self) -> String;
    fn has_permission(&self, permission: &str) -> bool;
    fn is_op(&self) -> bool;
}
