# PluginBase.py — Base class for all NexusCore plugins
# Every plugin must extend this class


class PluginBase:
    """
    Base class for all NexusCore plugins.
    Override on_load, on_enable, on_disable lifecycle hooks.
    """

    def __init__(self):
        self._plugin_manager = None
        self._server = None
        self._logger = None
        self._data_folder = ""

    def set_context(self, plugin_manager, server, logger, data_folder):
        """Called by the engine when loading the plugin (internal)"""
        self._plugin_manager = plugin_manager
        self._server = server
        self._logger = logger
        self._data_folder = data_folder

    # --- Lifecycle hooks (override these) ---

    def on_load(self):
        """Called when plugin is loaded (before enable)"""
        pass

    def on_enable(self):
        """Called when plugin is enabled"""
        pass

    def on_disable(self):
        """Called when plugin is disabled"""
        pass

    # --- Accessors ---

    def get_server(self):
        """Get the Server instance"""
        return self._server

    def get_logger(self):
        """Get the plugin logger"""
        return self._logger

    def get_data_folder(self):
        """Get the plugin's data folder path"""
        return self._data_folder

    def get_plugin_manager(self):
        """Get the PluginManager"""
        return self._plugin_manager

    def get_name(self):
        """Get plugin name"""
        return self.__class__.__name__

    # --- Event registration ---

    def register_event(self, event_name, handler, priority="NORMAL"):
        """Register an event listener"""
        if self._plugin_manager:
            self._plugin_manager.register_event(event_name, handler, priority, self)

    def register_events(self, listener):
        """Register all handler methods in a listener class"""
        import inspect
        for name, method in inspect.getmembers(listener, inspect.ismethod):
            if name.startswith("on_") and len(name) > 3:
                event_name = name[3:] + "Event"
                self.register_event(event_name, method)

    # --- Command registration ---

    def register_command(self, name, executor, description="", usage="", permission=""):
        """Register a command"""
        if self._plugin_manager:
            self._plugin_manager.register_command(name, executor, description, usage, permission, self)
