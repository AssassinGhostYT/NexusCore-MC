# CommandSender.py — CommandSender interface for Python plugins


class CommandSender:
    """
    Interface for anything that can send commands.
    Implemented by Player and Console.
    """

    def __init__(self):
        pass

    def send_message(self, message):
        """Send a message to this sender"""
        pass  # Implemented by Rust

    def get_name(self):
        """Get sender name"""
        return ""  # Implemented by Rust

    def has_permission(self, permission):
        """Check if sender has a permission"""
        return False  # Implemented by Rust

    def is_op(self):
        """Check if sender is operator"""
        return False  # Implemented by Rust
