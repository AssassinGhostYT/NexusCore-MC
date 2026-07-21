// Mensajes en español para NexusCore-MC
pub struct Es;

impl Es {
    pub const SERVER_STARTING: &'static str = "Iniciando NexusCore-MC en {}";
    pub const RAKNET_LISTENING: &'static str = "RakNet escuchando en {}:{} con GUID {}";
    pub const LISTENING: &'static str = "Escuchando conexiones...";
    pub const CLIENT_CONNECTED: &'static str = "Cliente conectado: {}";
    pub const CLIENT_DISCONNECTED: &'static str = "Cliente desconectado: {}";
    pub const PROTOCOL_VERSION: &'static str = "Version de protocolo: {}";
    pub const COMPRESSION_ENABLED: &'static str = "Compresion habilitada";
    pub const LOGIN_RECEIVED: &'static str = "Login recibido de {}";
    pub const LOGIN_USERNAME: &'static str = "Usuario: {}";
    pub const LOGIN_UUID: &'static str = "UUID: {}";
    pub const LOGIN_HAS_KEY: &'static str = "Clave de identidad: {}";
    pub const LOGIN_HAS_CLIENT_DATA: &'static str = "Datos de cliente: {}";
    pub const XBOX_AUTH_REQUESTED: &'static str = "Auth Xbox solicitada, iniciando handshake ECDH";
    pub const ENCRYPTION_ENABLED: &'static str = "Encriptacion habilitada";
    pub const OFFLINE_LOGIN: &'static str = "Login offline, saltando encriptacion";
    pub const LOGIN_FAILED: &'static str = "Error al parsear login: {}";
    pub const HANDSHAKE_JWT_FAILED: &'static str = "Error al generar handshake JWT: {:?}";
    pub const PUBLIC_KEY_PARSE_FAILED: &'static str = "Error al parsear clave publica del cliente: {:?}";
    pub const PLAYER_SPAWNED: &'static str = "Jugador spawneado (runtime_id={})";
    pub const GAMEPLAY_PACKETS_SENT: &'static str = "Packets de gameplay enviados";
    pub const COMPRESSION_ALGORITHM: &'static str = "Algoritmo de compresion: {}";
    pub const COMPRESSION_THRESHOLD: &'static str = "Umbral de compresion: {}";
    pub const RESOURCE_PACK_INFO_SENT: &'static str = "ResourcePacksInfo enviado";
    pub const RESOURCE_PACK_STACK_SENT: &'static str = "ResourcePackStack enviado";
    pub const START_GAME_SENT: &'static str = "StartGame enviado";
    pub const CHUNK_RADIUS_RECEIVED: &'static str = "RequestChunkRadius recibido, radio={}";
    pub const CHUNKS_SENT: &'static str = "Chunks enviados: {}x{}";
    pub const BIOME_DEFS_SENT: &'static str = "Definiciones de biomas enviadas";
    pub const PLAY_STATUS_SPAWN: &'static str = "PlayStatus(PlayerSpawn) enviado";
    pub const VIOLATION_DETECTED: &'static str = "VIOLACION DEL CLIENTE: PacketID=0x{:x} Severidad={} Contexto={}";
    pub const UNHANDLED_PACKET: &'static str = "Paquete no manejado ID: {}";
    pub const GAME_PACKET_RECEIVED: &'static str = "Paquete de juego: ID={} (0x{:02x}), Tam={}";
    pub const LANGUAGE_PROMPT: &'static str = "Selecciona idioma / Select language:\n  1) Español\n  2) English\n> ";
    pub const LANGUAGE_SELECTED: &'static str = "Idioma seleccionado: Español";
    pub const LANG_INVALID: &'static str = "Opcion invalida, usando Español por defecto";
    pub const SERVER_READY: &'static str = "=== NexusCore-MC listo para recibir conexiones ===";
}
