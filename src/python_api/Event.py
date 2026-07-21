# Event.py — Event system for Python plugins


class Event:
    """Base class for all events"""

    def __init__(self):
        pass

    def get_name(self):
        """Get event name"""
        return self.__class__.__name__


class Cancellable:
    """Mixin for events that can be cancelled"""

    def __init__(self):
        self._cancelled = False

    def is_cancelled(self):
        return self._cancelled

    def set_cancelled(self, cancelled):
        self._cancelled = cancelled

    def cancel(self):
        self._cancelled = True


class PlayerJoinEvent(Event):
    """Fired when a player joins the server"""

    def __init__(self, player):
        super().__init__()
        self.player = player


class PlayerQuitEvent(Event):
    """Fired when a player disconnects"""

    def __init__(self, player):
        super().__init__()
        self.player = player


class PlayerMoveEvent(Event, Cancellable):
    """Fired when a player moves"""

    def __init__(self, player, from_pos, to_pos):
        super().__init__()
        Cancellable.__init__(self)
        self.player = player
        self.from_pos = from_pos
        self.to_pos = to_pos


class PlayerChatEvent(Event, Cancellable):
    """Fired when a player sends a chat message"""

    def __init__(self, player, message):
        super().__init__()
        Cancellable.__init__(self)
        self.player = player
        self.message = message


class BlockBreakEvent(Event, Cancellable):
    """Fired when a player breaks a block"""

    def __init__(self, player, position, block_id):
        super().__init__()
        Cancellable.__init__(self)
        self.player = player
        self.position = position
        self.block_id = block_id


class BlockPlaceEvent(Event, Cancellable):
    """Fired when a player places a block"""

    def __init__(self, player, position, block_id):
        super().__init__()
        Cancellable.__init__(self)
        self.player = player
        self.position = position
        self.block_id = block_id
