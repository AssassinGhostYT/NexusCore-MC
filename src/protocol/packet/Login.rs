// Login packet (ID 1) - v1001 format
// Format: [i32 BE protocol_version][varint conn_req_len][conn_req_bytes]
// conn_req: [i32 LE auth_len][auth_json][i32 LE client_data_len][client_data_jwt]
//
// Xbox Live auth JSON structure (AuthenticationType=0, Online):
//   { "AuthenticationType": 0, "Token": "<JWT>", "Certificate": null }
//   The JWT payload contains: { "cpk": "<client_pub_key_b64>", "xid": "...", "xname": "..." }
//   The JWT header (x5u) contains the public key of whoever signed this JWT.
//
// Offline auth JSON structure (AuthenticationType=2):
//   { "AuthenticationType": 2, "Token": "<JWT>", "Certificate": null }
//   Same JWT but not signed by Xbox — payload has username/uuid.
//
// Old chain format (some older clients):
//   { "chain": ["<JWT1>", "<JWT2>", "<JWT3>"] }
//   Last JWT has extraData.displayName, extraData.XUID, identityPublicKey

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::protocol::error::{PacketError, PResult};
use crate::protocol::varint::read_varu32;

pub struct Login {
    pub protocol_version: i32,
    pub username: String,
    pub uuid: String,
    /// The client's ECDH public key (base64, for encryption handshake).
    /// Present for both Xbox (Online) and Offline clients.
    /// - Xbox: comes from "cpk" field in the Token JWT payload
    /// - Chain: comes from "identityPublicKey" in the last chain JWT payload
    pub identity_public_key: String,
    /// Raw AuthenticationType value: 0=Online(Xbox), 1=Guest, 2=Offline
    pub auth_type: u8,
    pub client_data: Option<Vec<u8>>,
}

/// Decode a JWT payload (base64url middle segment) into JSON
fn parse_jwt_payload(jwt: &str) -> Option<serde_json::Value> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() < 2 { return None; }
    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[1]
    ).ok()?;
    serde_json::from_slice(&decoded).ok()
}

/// Extract username, UUID, identity_public_key from JWT payload claims.
/// For Xbox (Token format): cpk = client public key, xname = username, xid = xuid
/// For Chain format: extraData.displayName, extraData.XUID, identityPublicKey
fn extract_from_claims(claims: &serde_json::Value) -> (String, String, String) {
    let mut username = String::new();
    let mut uuid = String::new();
    let mut pub_key = String::new();

    // Chain format: extraData object
    if let Some(extra) = claims.get("extraData") {
        if let Some(name) = extra.get("displayName").and_then(|v| v.as_str()) {
            if !name.is_empty() { username = name.to_string(); }
        }
        if let Some(xuid) = extra.get("XUID").and_then(|v| v.as_str()) {
            if !xuid.is_empty() { uuid = xuid.to_string(); }
        }
        // In chain format, extraData also has "identity" field (player UUID)
        if uuid.is_empty() {
            if let Some(id) = extra.get("identity").and_then(|v| v.as_str()) {
                uuid = id.to_string();
            }
        }
    }

    // Token format (Xbox/Offline): xname, xid, cpk
    if username.is_empty() {
        if let Some(n) = claims.get("xname").and_then(|v| v.as_str()) {
            username = n.to_string();
        }
    }
    if uuid.is_empty() {
        if let Some(id) = claims.get("xid").and_then(|v| v.as_str())
            .or_else(|| claims.get("sub").and_then(|v| v.as_str())) {
            uuid = id.to_string();
        }
    }

    // Public key: "cpk" (Token format) OR "identityPublicKey" (chain format)
    if let Some(k) = claims.get("cpk").and_then(|v| v.as_str()) {
        if !k.is_empty() { pub_key = k.to_string(); }
    }
    if pub_key.is_empty() {
        if let Some(k) = claims.get("identityPublicKey").and_then(|v| v.as_str()) {
            if !k.is_empty() { pub_key = k.to_string(); }
        }
    }

    (username, uuid, pub_key)
}

impl Login {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut buf = &payload[..];

        // 1. Protocol version: i32 big-endian
        let protocol_version = buf.read_i32::<BigEndian>().map_err(|e| {
            PacketError::Io { context: "Login.protocol_version", source: e }
        })?;
        log::debug!("Login: protocol_version={}", protocol_version);

        // 2. connection_request: varint-prefixed byte blob
        let conn_req_len = read_varu32(&mut buf)
            .ok_or_else(|| PacketError::VarintOverflow { kind: "Login.connection_request_length" })? as usize;
        log::debug!("Login: connection_request length={}", conn_req_len);

        if conn_req_len > buf.len() {
            return Err(PacketError::Underflow {
                field: "Login.connection_request",
                need: conn_req_len,
                have: buf.len(),
            });
        }
        let conn_req = &buf[..conn_req_len];
        let mut cr = &conn_req[..];

        // 3. Inside connection_request: auth_data (i32 LE length prefix)
        let auth_len = cr.read_i32::<LittleEndian>().map_err(|e| {
            PacketError::Io { context: "Login.auth_data_length", source: e }
        })? as usize;
        log::debug!("Login: auth_data length={}", auth_len);

        if auth_len > cr.len() {
            return Err(PacketError::Underflow {
                field: "Login.auth_data",
                need: auth_len,
                have: cr.len(),
            });
        }
        let auth_json_bytes = &cr[..auth_len];
        cr = &cr[auth_len..];

        // 4. Inside connection_request: client_data (i32 LE length prefix)
        let client_data_len = cr.read_i32::<LittleEndian>().map_err(|e| {
            PacketError::Io { context: "Login.client_data_length", source: e }
        })? as usize;
        log::debug!("Login: client_data length={}", client_data_len);

        if client_data_len > cr.len() {
            return Err(PacketError::Underflow {
                field: "Login.client_data",
                need: client_data_len,
                have: cr.len(),
            });
        }
        let client_data_raw = &cr[..client_data_len];

        // 5. Parse auth_data JSON
        let auth_json: serde_json::Value = serde_json::from_slice(auth_json_bytes).map_err(|e| {
            log::error!("Login auth_data JSON parse failed: {}", e);
            PacketError::Json { context: "Login.auth_data", source: e }
        })?;

        let auth_json_keys: Vec<&str> = auth_json.as_object()
            .map(|o| o.keys().map(|k| k.as_str()).collect())
            .unwrap_or_default();
        log::debug!("Login: auth_json keys: {:?}", auth_json_keys);

        let mut username = String::new();
        let mut uuid = String::new();
        let mut identity_public_key = String::new();
        let mut auth_type: u8 = 2; // default = offline

        // ── Path A: Modern Token format (Xbox Live & Offline) ────────────────
        // { "AuthenticationType": 0|1|2, "Token": "<JWT>", "Certificate": null }
        if let Some(at_val) = auth_json.get("AuthenticationType") {
            auth_type = at_val.as_u64().unwrap_or(2) as u8;
            log::debug!("Login: AuthenticationType={} (0=Online,1=Guest,2=Offline)", auth_type);

            if let Some(token) = auth_json.get("Token").and_then(|v| v.as_str()) {
                if !token.is_empty() {
                    log::debug!("Login: parsing Token JWT (len={})", token.len());
                    if let Some(claims) = parse_jwt_payload(token) {
                        let claims_keys: Vec<&str> = claims.as_object()
                            .map(|o| o.keys().map(|k| k.as_str()).collect())
                            .unwrap_or_default();
                        log::debug!("Login: Token JWT payload keys: {:?}", claims_keys);
                        let (u, x, k) = extract_from_claims(&claims);
                        if !u.is_empty() { username = u; }
                        if !x.is_empty() { uuid = x; }
                        if !k.is_empty() { identity_public_key = k; }
                    } else {
                        log::warn!("Login: failed to decode Token JWT payload");
                    }
                }
            }
        }
        // ── Path B: Old chain format ──────────────────────────────────────────
        // { "chain": ["<JWT1>", "<JWT2>", "<JWT3>"] }
        // The chain JWTs are signed by Xbox — last one has extraData with username/XUID
        // and identityPublicKey = client's public key.
        else if let Some(chain) = auth_json.get("chain").and_then(|v| v.as_array()) {
            log::debug!("Login: chain format with {} JWTs", chain.len());
            for (i, jwt_val) in chain.iter().enumerate() {
                if let Some(jwt_str) = jwt_val.as_str() {
                    if let Some(claims) = parse_jwt_payload(jwt_str) {
                        let (u, x, k) = extract_from_claims(&claims);
                        if !u.is_empty() && username.is_empty() { username = u; uuid = x; }
                        if !k.is_empty() { identity_public_key = k; }
                        log::debug!("Login: chain[{}] username='{}' has_key={}", i, username, !identity_public_key.is_empty());
                    }
                }
            }
            // Chain with 3 JWTs = Xbox Live (signed chain), 1 JWT = offline
            auth_type = if chain.len() >= 3 { 0 } else { 2 };
        }

        if username.is_empty() { username = "Unknown".to_string(); }

        let auth_mode = match auth_type {
            0 => "Xbox Live",
            1 => "Guest",
            _ => "Offline",
        };
        log::info!("[Login] Username: '{}' ({})", username, auth_mode);

        let client_data_opt = if client_data_raw.is_empty() { None } else { Some(client_data_raw.to_vec()) };

        Ok(Self {
            protocol_version,
            username,
            uuid,
            identity_public_key,
            auth_type,
            client_data: client_data_opt,
        })
    }
}
