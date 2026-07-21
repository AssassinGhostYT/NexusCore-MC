#![allow(deprecated)]
use p384::{SecretKey, PublicKey};
use p384::ecdsa::{SigningKey, Signature, signature::Signer};
use p384::elliptic_curve::ecdh::diffie_hellman;
use p384::elliptic_curve::pkcs8::{DecodePublicKey, EncodePublicKey};
use sha2::Sha256;
use aes::Aes256;
use ctr::cipher::{KeyIvInit, StreamCipher};
use base64::engine::general_purpose::{URL_SAFE_NO_PAD, STANDARD};
use base64::Engine;

// AES-256-CTR — matches gophertunnel exactly:
// cipher.NewCTR(block, append(keyBytes[:12], 0, 0, 0, 2))
// This is little-endian counter mode starting at 2, 128-bit block
type Aes256Ctr = ctr::Ctr128BE<Aes256>;

pub struct EncryptionState {
    encrypter: Aes256Ctr,
    decrypter: Aes256Ctr,
    send_counter: u64,
    receive_counter: u64,
    key_bytes: [u8; 32],
}

/// Compute packet checksum matching gophertunnel exactly:
/// SHA256(counter_LE_8bytes || body || key_bytes)
/// The `body` passed here is everything after the 0xfe game packet wrapper byte.
fn compute_checksum(body: &[u8], counter: u64, key_bytes: &[u8]) -> [u8; 8] {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(&counter.to_le_bytes());
    hasher.update(body);
    hasher.update(key_bytes);
    let result = hasher.finalize();
    let mut checksum = [0u8; 8];
    checksum.copy_from_slice(&result[..8]);
    checksum
}

impl EncryptionState {
    /// Create a new EncryptionState from the ECDH shared secret and salt.
    ///
    /// Key derivation (matches gophertunnel exactly):
    ///   keyBytes = SHA-256(salt || sharedSecret_zeroPadded_48bytes)
    ///
    /// IV (matches gophertunnel decoder.go / encoder.go exactly):
    ///   iv = keyBytes[0..12] + [0, 0, 0, 2]
    pub fn new(shared_secret: &[u8], salt: &[u8]) -> Self {
        use sha2::Digest;

        // Zero-pad shared secret to 48 bytes (P-384 = 48 byte field element)
        // Gophertunnel: append(bytes.Repeat([]byte{0}, 48-len(x.Bytes())), x.Bytes()...)
        let mut padded_secret = [0u8; 48];
        let offset = 48usize.saturating_sub(shared_secret.len());
        padded_secret[offset..].copy_from_slice(&shared_secret[shared_secret.len().saturating_sub(48)..]);

        // SHA-256(salt || padded_shared_secret) → 32-byte key
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(&padded_secret);
        let key_bytes: [u8; 32] = hasher.finalize().into();

        // IV = key_bytes[0..12] + [0, 0, 0, 2]  (16 bytes total)
        // Matches: cipher.NewCTR(block, append(first12, 0, 0, 0, 2))
        let mut iv = [0u8; 16];
        iv[..12].copy_from_slice(&key_bytes[..12]);
        iv[12] = 0;
        iv[13] = 0;
        iv[14] = 0;
        iv[15] = 2;

        use generic_array::GenericArray;
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
            key_bytes,
        }
    }

    /// Encrypt a game packet body (everything after the 0xfe game packet wrapper byte).
    /// Matching gophertunnel encoder:
    ///   hash = SHA256(counter_LE || body || keyBytes)
    ///   data = append(body, hash[:8])
    ///   stream.XORKeyStream(data, data)
    pub fn encrypt_packet(&mut self, body: &[u8]) -> Vec<u8> {
        let checksum = compute_checksum(body, self.send_counter, &self.key_bytes);
        self.send_counter += 1;

        let mut out = Vec::with_capacity(body.len() + 8);
        out.extend_from_slice(body);
        out.extend_from_slice(&checksum);

        // Encrypt the entire body + checksum
        self.encrypter.apply_keystream(&mut out);
        out
    }

    /// Decrypt an incoming encrypted game packet body (everything after the 0xfe wrapper byte).
    /// stream.XORKeyStream(body, body) — decrypts the full body slice in place.
    /// Then verify checksum on body[..len-8].
    pub fn decrypt_packet(&mut self, body: &mut [u8]) -> Result<Vec<u8>, String> {
        if body.len() < 8 {
            return Err("Payload too short for checksum".to_string());
        }

        // Decrypt the body (excluding 0xfe, which is not part of this slice)
        self.decrypter.apply_keystream(body);

        let split_idx = body.len() - 8;
        let (packet_bytes, checksum_bytes) = body.split_at(split_idx);

        // Verify checksum: SHA256(counter || packet_bytes || key_bytes)
        let computed = compute_checksum(packet_bytes, self.receive_counter, &self.key_bytes);
        self.receive_counter += 1;

        if checksum_bytes != computed {
            return Err(format!(
                "Checksum mismatch: received {:02x?}, computed {:02x?}",
                checksum_bytes, computed
            ));
        }

        Ok(packet_bytes.to_vec())
    }
}

/// Parse the client's public key from base64-encoded DER (SubjectPublicKeyInfo).
/// The `cpk` field in the Xbox Token JWT is standard base64 (not URL-safe).
pub fn parse_client_public_key(pub_key_b64: &str) -> Result<PublicKey, String> {
    // Try standard base64 first (how Xbox sends cpk), then URL-safe
    let der_bytes = STANDARD.decode(pub_key_b64)
        .or_else(|_| URL_SAFE_NO_PAD.decode(pub_key_b64))
        .map_err(|e| format!("Failed to decode base64 client public key: {:?}", e))?;

    PublicKey::from_public_key_der(&der_bytes)
        .map_err(|e| format!("Failed to parse public key from DER: {:?}", e))
}

/// Generate the ServerToClientHandshake JWT.
///
/// Structure (matches gophertunnel enableEncryption):
/// Header: { "alg": "ES384", "x5u": "<server_pub_DER_standard_base64>" }
/// Payload: { "salt": "<salt_RawStdEncoding>" }
/// Signed with ES384 (ECDSA P-384)
///
/// Salt encoding: base64.RawStdEncoding = standard alphabet, NO padding (STANDARD_NO_PAD)
pub fn generate_handshake_jwt(secret_key: &SecretKey, salt: &[u8]) -> Result<String, String> {
    let public_key = secret_key.public_key();

    // Serialize server public key to DER, then standard base64 (matches MarshalPublicKey)
    let der_doc = public_key.to_public_key_der()
        .map_err(|e| format!("Failed to serialize public key to DER: {:?}", e))?;
    let x5u_b64 = STANDARD.encode(der_doc.as_bytes());

    // Header
    let header = serde_json::json!({
        "alg": "ES384",
        "x5u": x5u_b64
    });
    let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap());

    // Payload — salt uses RawStdEncoding (standard alphabet, no padding)
    // base64.RawStdEncoding = STANDARD_NO_PAD in base64 crate
    let salt_b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(salt);
    let payload = serde_json::json!({ "salt": salt_b64 });
    let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload).unwrap());

    // Sign with ES384
    let signing_key = SigningKey::from(secret_key);
    let msg = format!("{}.{}", header_b64, payload_b64);
    let signature: Signature = signing_key.sign(msg.as_bytes());
    let sig_b64 = URL_SAFE_NO_PAD.encode(signature.to_bytes().as_slice());

    Ok(format!("{}.{}.{}", header_b64, payload_b64, sig_b64))
}

/// Compute the ECDH shared secret.
/// Returns the raw X coordinate (up to 48 bytes for P-384).
/// The caller is responsible for zero-padding to 48 bytes if needed.
#[allow(deprecated)]
pub fn compute_shared_secret(secret_key: &SecretKey, client_public: &PublicKey) -> Vec<u8> {
    let shared = diffie_hellman(
        secret_key.to_nonzero_scalar(),
        client_public.as_affine(),
    );
    shared.raw_secret_bytes().as_slice().to_vec()
}
