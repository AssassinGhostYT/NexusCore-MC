// English messages for NexusCore-MC
pub struct En;

impl En {
    pub const SERVER_STARTING: &'static str = "Starting NexusCore-MC on {}";
    pub const RAKNET_LISTENING: &'static str = "RakNet listening on {}:{} with GUID {}";
    pub const LISTENING: &'static str = "Listening for connections...";
    pub const CLIENT_CONNECTED: &'static str = "Client connected: {}";
    pub const CLIENT_DISCONNECTED: &'static str = "Client disconnected: {}";
    pub const PROTOCOL_VERSION: &'static str = "Protocol version: {}";
    pub const COMPRESSION_ENABLED: &'static str = "Compression enabled";
    pub const LOGIN_RECEIVED: &'static str = "Login received from {}";
    pub const LOGIN_USERNAME: &'static str = "Username: {}";
    pub const LOGIN_UUID: &'static str = "UUID: {}";
    pub const LOGIN_HAS_KEY: &'static str = "Identity key: {}";
    pub const LOGIN_HAS_CLIENT_DATA: &'static str = "Client data: {}";
    pub const XBOX_AUTH_REQUESTED: &'static str = "Xbox auth requested, starting ECDH handshake";
    pub const ENCRYPTION_ENABLED: &'static str = "Encryption enabled";
    pub const OFFLINE_LOGIN: &'static str = "Offline login, skipping encryption";
    pub const LOGIN_FAILED: &'static str = "Failed to parse login: {}";
    pub const HANDSHAKE_JWT_FAILED: &'static str = "Failed to generate handshake JWT: {:?}";
    pub const PUBLIC_KEY_PARSE_FAILED: &'static str = "Failed to parse client public key: {:?}";
    pub const PLAYER_SPAWNED: &'static str = "Player spawned (runtime_id={})";
    pub const GAMEPLAY_PACKETS_SENT: &'static str = "Gameplay packets sent";
    pub const COMPRESSION_ALGORITHM: &'static str = "Compression algorithm: {}";
    pub const COMPRESSION_THRESHOLD: &'static str = "Compression threshold: {}";
    pub const RESOURCE_PACK_INFO_SENT: &'static str = "ResourcePacksInfo sent";
    pub const RESOURCE_PACK_STACK_SENT: &'static str = "ResourcePackStack sent";
    pub const START_GAME_SENT: &'static str = "StartGame sent";
    pub const CHUNK_RADIUS_RECEIVED: &'static str = "RequestChunkRadius received, radius={}";
    pub const CHUNKS_SENT: &'static str = "Chunks sent: {}x{}";
    pub const BIOME_DEFS_SENT: &'static str = "Biome definitions sent";
    pub const PLAY_STATUS_SPAWN: &'static str = "PlayStatus(PlayerSpawn) sent";
    pub const VIOLATION_DETECTED: &'static str = "CLIENT VIOLATION: PacketID=0x{:x} Severity={} Context={}";
    pub const UNHANDLED_PACKET: &'static str = "Unhandled packet ID: {}";
    pub const GAME_PACKET_RECEIVED: &'static str = "Game packet: ID={} (0x{:02x}), Size={}";
    pub const LANGUAGE_PROMPT: &'static str = "Select language:\n  1) Español\n  2) English\n> ";
    pub const LANGUAGE_SELECTED: &'static str = "Language selected: English";
    pub const LANG_INVALID: &'static str = "Invalid option, using English by default";
    pub const SERVER_READY: &'static str = "=== NexusCore-MC ready to accept connections ===";
}
