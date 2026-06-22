use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub const ID_LOGIN: u32 = 1;

#[derive(Debug, Clone)]
pub struct Login {
    #[allow(dead_code)]
    pub protocol_version: i32,
    pub username: String,
    pub uuid: String,
    pub xuid: String,
    pub game_version: String,
    pub device_os: i32,
    pub identity_public_key: String,
}

fn decode_jwt_payload(jwt: &str) -> Option<serde_json::Value> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let payload_b64 = parts[1];
    
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    
    let decoded_bytes = URL_SAFE_NO_PAD.decode(payload_b64.as_bytes()).ok()?;
    let json_val: serde_json::Value = serde_json::from_slice(&decoded_bytes).ok()?;
    Some(json_val)
}

impl Login {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        log::info!("Login::read: payload length = {}", payload.len());
        log::info!("Login::read: first 32 bytes = {:02x?}", &payload[..std::cmp::min(payload.len(), 32)]);
        let protocol_version = payload.read_i32::<BigEndian>().ok()?;
        log::info!("Login::read: protocol_version = {}", protocol_version);
        
        let request_len = match crate::protocol::varint::read_varu32(&mut payload) {
            Some(len) => len as usize,
            None => {
                log::warn!("Login::read: Failed to read request_len VarInt");
                return None;
            }
        };
        log::info!("Login::read: request_len = {}", request_len);
        if payload.len() < request_len {
            log::warn!("Login::read: payload too small for request_len {}", request_len);
            return None;
        }
        let mut request_buf = &payload[..request_len];
        
        let chain_len = match request_buf.read_u32::<LittleEndian>() {
            Ok(len) => len as usize,
            Err(_) => {
                log::warn!("Login::read: Failed to read chain_len");
                return None;
            }
        };
        log::info!("Login::read: chain_len = {}", chain_len);
        if request_buf.len() < chain_len {
            log::warn!("Login::read: request_buf too small for chain_len {}", chain_len);
            return None;
        }
        let chain_bytes = &request_buf[..chain_len];
        request_buf = &request_buf[chain_len..];
        
        let chain_str = match std::str::from_utf8(chain_bytes) {
            Ok(s) => s.to_string(),
            Err(e) => {
                log::warn!("Login::read: chain_bytes is not valid UTF-8: {:?}", e);
                return None;
            }
        };
        log::info!("Login::read: chain_str (len={}) = '{}'", chain_str.len(), chain_str);
        
        let chain_json: serde_json::Value = match serde_json::from_str(&chain_str) {
            Ok(json) => json,
            Err(e) => {
                log::warn!("Login::read: Failed to parse chain_str as JSON: {:?}", e);
                return None;
            }
        };
        
        let chain_arr = if let Some(arr) = chain_json.get("chain").and_then(|v| v.as_array()) {
            arr.clone()
        } else if let Some(token) = chain_json.get("Token").and_then(|v| v.as_str()) {
            vec![serde_json::Value::String(token.to_string())]
        } else {
            log::warn!("Login::read: Missing or invalid 'chain' or 'Token' in JSON");
            return None;
        };
        log::info!("Login::read: Found {} chain/token elements", chain_arr.len());
        
        let mut username = String::new();
        let mut uuid = String::new();
        let mut xuid = String::new();
        let mut identity_public_key = String::new();
        
        for (i, jwt_val) in chain_arr.iter().enumerate() {
            let jwt_str = match jwt_val.as_str() {
                Some(s) => s,
                None => {
                    log::warn!("Login::read: chain[{}] is not a string", i);
                    continue;
                }
            };
            if let Some(payload_json) = decode_jwt_payload(jwt_str) {
                if let Some(pub_key) = payload_json.get("identityPublicKey").or_else(|| payload_json.get("cpk")).and_then(|v| v.as_str()) {
                    identity_public_key = pub_key.to_string();
                }
                if let Some(extra_data) = payload_json.get("extraData") {
                    if let Some(display_name) = extra_data.get("displayName").and_then(|v| v.as_str()) {
                        username = display_name.to_string();
                    }
                    if let Some(identity) = extra_data.get("identity").and_then(|v| v.as_str()) {
                        uuid = identity.to_string();
                    }
                    if let Some(xuid_val) = extra_data.get("XUID").and_then(|v| v.as_str()) {
                        xuid = xuid_val.to_string();
                    }
                } else {
                    if let Some(xname) = payload_json.get("xname").and_then(|v| v.as_str()) {
                        username = xname.to_string();
                    }
                    if let Some(sub) = payload_json.get("sub").and_then(|v| v.as_str()) {
                        uuid = sub.to_string();
                    }
                    if let Some(xid_val) = payload_json.get("xid").and_then(|v| v.as_str()) {
                        xuid = xid_val.to_string();
                    }
                }
            } else {
                log::warn!("Login::read: Failed to decode JWT payload for chain[{}]", i);
            }
        }
        log::info!("Login::read: Parsed credentials: username='{}', uuid='{}', xuid='{}', identity_public_key='{}'", username, uuid, xuid, identity_public_key);
        
        let client_data_len = match request_buf.read_u32::<LittleEndian>() {
            Ok(len) => len as usize,
            Err(_) => {
                log::warn!("Login::read: Failed to read client_data_len");
                return None;
            }
        };
        log::info!("Login::read: client_data_len = {}", client_data_len);
        if request_buf.len() < client_data_len {
            log::warn!("Login::read: request_buf too small for client_data_len {}", client_data_len);
            return None;
        }
        let client_data_bytes = &request_buf[..client_data_len];
        let client_data_str = match std::str::from_utf8(client_data_bytes) {
            Ok(s) => s.to_string(),
            Err(e) => {
                log::warn!("Login::read: client_data_bytes is not valid UTF-8: {:?}", e);
                return None;
            }
        };
        
        let mut game_version = String::new();
        let mut device_os = 0;
        
        if let Some(client_data_json) = decode_jwt_payload(&client_data_str) {
            if let Some(gv) = client_data_json.get("GameVersion").and_then(|v| v.as_str()) {
                game_version = gv.to_string();
            }
            if let Some(dos) = client_data_json.get("DeviceOS").and_then(|v| v.as_i64()) {
                device_os = dos as i32;
            }
        } else {
            log::warn!("Login::read: Failed to decode client_data_str JWT payload");
        }
        
        Some(Login {
            protocol_version,
            username,
            uuid,
            xuid,
            game_version,
            device_os,
            identity_public_key,
        })
    }
}
