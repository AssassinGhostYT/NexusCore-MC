use p384::{SecretKey, PublicKey};
use p384::ecdsa::{SigningKey, Signature, signature::Signer};
use p384::elliptic_curve::ecdh::diffie_hellman;
use p384::elliptic_curve::pkcs8::{DecodePublicKey, EncodePublicKey};
use sha2::Sha256;
use aes::Aes256;
use ctr::cipher::{KeyIvInit, StreamCipher};
use base64::engine::general_purpose::{URL_SAFE_NO_PAD, STANDARD};
use base64::Engine;

type Aes256Ctr = ctr::Ctr128BE<Aes256>;

pub struct EncryptionState {
    encrypter: Aes256Ctr,
    decrypter: Aes256Ctr,
    send_counter: u64,
    receive_counter: u64,
    secret_key: [u8; 32],
}

fn compute_checksum(packet: &[u8], counter: u64, secret_key: &[u8]) -> [u8; 8] {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(&counter.to_le_bytes());
    hasher.update(packet);
    hasher.update(secret_key);
    let result = hasher.finalize();
    let mut checksum = [0u8; 8];
    checksum.copy_from_slice(&result[..8]);
    checksum
}

impl EncryptionState {
    pub fn new(shared_secret: &[u8], salt: &[u8]) -> Self {
        // Derive key bytes: SHA-256(salt || shared_secret)
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(shared_secret);
        let key_bytes: [u8; 32] = hasher.finalize().into();

        // GCM nonce is the first 12 bytes of key_bytes
        // Counter starts at 2
        let mut iv = [0u8; 16];
        iv[..12].copy_from_slice(&key_bytes[..12]);
        iv[12..16].copy_from_slice(&[0, 0, 0, 2]);

        use p384::elliptic_curve::generic_array::GenericArray;
        let encrypter = Aes256Ctr::new(
            GenericArray::from_slice(&key_bytes),
            GenericArray::from_slice(&iv),
        );
        let decrypter = Aes256Ctr::new(
            GenericArray::from_slice(&key_bytes),
            GenericArray::from_slice(&iv),
        );

        EncryptionState {
            encrypter,
            decrypter,
            send_counter: 0,
            receive_counter: 0,
            secret_key: key_bytes,
        }
    }

    pub fn encrypt_packet(&mut self, payload: &[u8]) -> Vec<u8> {
        let checksum = compute_checksum(payload, self.send_counter, &self.secret_key);
        self.send_counter += 1;

        // Construct encrypted payload: payload + checksum (8 bytes)
        let mut encrypted = Vec::with_capacity(payload.len() + 8);
        encrypted.extend_from_slice(payload);
        encrypted.extend_from_slice(&checksum);

        self.encrypter.apply_keystream(&mut encrypted);
        encrypted
    }

    pub fn decrypt_packet(&mut self, payload: &mut [u8]) -> Result<Vec<u8>, String> {
        if payload.len() < 8 {
            return Err("Payload too short for checksum".to_string());
        }

        // Decrypt in-place
        self.decrypter.apply_keystream(payload);

        let split_idx = payload.len() - 8;
        let (packet_bytes, checksum_bytes) = payload.split_at(split_idx);

        let computed_checksum = compute_checksum(packet_bytes, self.receive_counter, &self.secret_key);
        self.receive_counter += 1;

        if checksum_bytes != computed_checksum {
            return Err(format!(
                "Checksum mismatch: received {:02x?}, computed {:02x?}",
                checksum_bytes, computed_checksum
            ));
        }

        Ok(packet_bytes.to_vec())
    }
}

pub fn parse_client_public_key(pub_key_b64: &str) -> Result<PublicKey, String> {
    let der_bytes = STANDARD.decode(pub_key_b64)
        .or_else(|_| {
            use base64::engine::general_purpose::URL_SAFE_NO_PAD;
            URL_SAFE_NO_PAD.decode(pub_key_b64)
        })
        .map_err(|e| format!("Failed to decode base64 client public key: {:?}", e))?;
        
    PublicKey::from_public_key_der(&der_bytes)
        .map_err(|e| format!("Failed to parse public key from DER: {:?}", e))
}

pub fn generate_handshake_jwt(secret_key: &SecretKey, salt: &[u8]) -> Result<String, String> {
    let public_key = secret_key.public_key();
    
    let der_doc = public_key.to_public_key_der()
        .map_err(|e| format!("Failed to serialize public key to DER: {:?}", e))?;
    let der_bytes = der_doc.as_bytes();
    
    let x5u_b64 = STANDARD.encode(der_bytes);
    
    // Construct Header
    let header = serde_json::json!({
        "alg": "ES384",
        "x5u": x5u_b64
    });
    let header_str = serde_json::to_string(&header)
        .map_err(|e| format!("Failed to serialize JWT header: {:?}", e))?;
    let header_b64 = URL_SAFE_NO_PAD.encode(header_str.as_bytes());
    
    // Construct Payload
    let salt_b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(salt);
    let payload = serde_json::json!({
        "salt": salt_b64
    });
    let payload_str = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize JWT payload: {:?}", e))?;
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload_str.as_bytes());
    
    // Sign
    let msg = format!("{}.{}", header_b64, payload_b64);
    
    let signing_key = SigningKey::from(secret_key);
    let signature: Signature = signing_key.sign(msg.as_bytes());
    let sig_bytes = signature.to_bytes(); // 96 bytes raw signature
    let sig_b64 = URL_SAFE_NO_PAD.encode(&sig_bytes);
    
    Ok(format!("{}.{}.{}", header_b64, payload_b64, sig_b64))
}

pub fn compute_shared_secret(secret_key: &SecretKey, client_public: &PublicKey) -> Vec<u8> {
    let shared_secret = diffie_hellman(
        secret_key.to_nonzero_scalar(),
        client_public.as_affine(),
    );
    shared_secret.raw_secret_bytes().as_slice().to_vec()
}
