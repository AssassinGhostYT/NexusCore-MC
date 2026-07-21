use p384::SecretKey;
use p384::ecdsa::{SigningKey, VerifyingKey, Signature};
use p384::ecdsa::signature::{Signer, Verifier};
use p384::pkcs8::EncodePublicKey;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

fn main() {
    let secret_key = SecretKey::random(&mut rand::rngs::OsRng);
    let salt = b"1234567890123678"; // 16 bytes

    let jwt = crate_handshake_jwt(&secret_key, salt);
    println!("JWT: {}", jwt);

    let parts: Vec<&str> = jwt.split('.').collect();
    assert_eq!(parts.len(), 3);

    let header_b64 = parts[0];
    let payload_b64 = parts[1];
    let sig_b64 = parts[2];

    let header_str = String::from_utf8(URL_SAFE_NO_PAD.decode(header_b64).unwrap()).unwrap();
    let payload_str = String::from_utf8(URL_SAFE_NO_PAD.decode(payload_b64).unwrap()).unwrap();
    println!("Header: {}", header_str);
    println!("Payload: {}", payload_str);

    let sig_bytes = URL_SAFE_NO_PAD.decode(sig_b64).unwrap();
    println!("Signature length: {}", sig_bytes.len());

    let msg = format!("{}.{}", header_b64, payload_b64);
    
    let signing_key = SigningKey::from(secret_key);
    let verifying_key = VerifyingKey::from(&signing_key);
    let signature = Signature::from_slice(&sig_bytes).unwrap();
    
    match verifying_key.verify(msg.as_bytes(), &signature) {
        Ok(_) => println!("Signature VERIFIED successfully!"),
        Err(e) => println!("Signature VERIFICATION FAILED: {:?}", e),
    }
}

fn crate_handshake_jwt(secret_key: &SecretKey, salt: &[u8]) -> String {
    use base64::engine::general_purpose::STANDARD;
    let public_key = secret_key.public_key();
    let der_doc = public_key.to_public_key_der().unwrap();
    let x5u_b64 = STANDARD.encode(der_doc.as_bytes());

    let header = serde_json::json!({
        "alg": "ES384",
        "x5u": x5u_b64
    });
    let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap());

    let salt_b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(salt);
    let payload = serde_json::json!({ "salt": salt_b64 });
    let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload).unwrap());

    let signing_key = SigningKey::from(secret_key);
    let msg = format!("{}.{}", header_b64, payload_b64);
    let signature: Signature = signing_key.sign(msg.as_bytes());
    let sig_b64 = URL_SAFE_NO_PAD.encode(signature.to_bytes().as_ref());

    format!("{}.{}.{}", header_b64, payload_b64, sig_b64)
}
