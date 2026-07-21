# Player.py — Player interface for Python plugins


class Player:
    """
    Represents a connected player.
    Provided by the Rust engine via PyO3.
    """

    def __init__(self, name, uuid, entity_id):
        self._name = name
        self._uuid = uuid
        self._entity_id = entity_id

    def get_name(self):
        return self._name

    def get_uuid(self):
        return self._uuid

    def get_entity_id(self):
        return self._entity_id

    def send_message(self, message):
        """Send a message to this player"""
        pass  # Implemented by Rust via PyO3

    def teleport(self, x, y, z):
        """Teleport the player"""
        pass  # Implemented by Rust via PyO3

    def get_position(self):
        """Get player position (x, y, z)"""
        return (0, 0, 0)  # Implemented by Rust via PyO3

    def get_gamemode(self):
        """Get player gamemode"""
        return 0  # Implemented by Rust via PyO3

    def set_gamemode(self, gamemode):
        """Set player gamemode"""
        pass  # Implemented by Rust via PyO3

    def kick(self, reason=""):
        """Kick the player"""
        pass  # Implemented by Rust via PyO3

    def ban(self, reason="", duration=None):
        """Ban the player"""
        pass  # Implemented by Rust via PyO3

    # Property aliases
    name = property(get_name)
    uuid = property(get_uuid)
    entity_id = property(get_entity_id)
