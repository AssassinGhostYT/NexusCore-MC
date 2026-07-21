use p384::SecretKey;
use p384::ecdsa::{SigningKey, Signature};
use p384::ecdsa::signature::Signer;
use p384::pkcs8::{EncodePublicKey, EncodePrivateKey};
use base64::Engine;
use base64::engine::general_purpose::{URL_SAFE_NO_PAD, STANDARD};
use std::fs;

fn main() {
    let secret_key = SecretKey::random(&mut rand::rngs::OsRng);
    
    // Save private key in PEM format so Go can load the same key to verify if it wants
    let pkcs8_pem = secret_key.to_pkcs8_pem(p384::elliptic_curve::pkcs8::LineEnding::LF).unwrap();
    fs::write("/root/verify_jwt/server.pem", pkcs8_pem.as_str()).unwrap();

    let public_key = secret_key.public_key();
    let der_doc = public_key.to_public_key_der().unwrap();
    let x5u_b64 = STANDARD.encode(der_doc.as_bytes());

    let header = serde_json::json!({
        "alg": "ES384",
        "x5u": x5u_b64
    });
    let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap().as_bytes());

    let salt = b"1234567890123456";
    let salt_b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(salt);
    let payload = serde_json::json!({ "salt": salt_b64 });
    let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload).unwrap().as_bytes());

    let msg = format!("{}.{}", header_b64, payload_b64);
    let signing_key = SigningKey::from(&secret_key);
    let signature: Signature = signing_key.sign(msg.as_bytes());
    let sig_b64 = URL_SAFE_NO_PAD.encode(signature.to_bytes().as_ref());

    let jwt = format!("{}.{}.{}", header_b64, payload_b64, sig_b64);
    fs::write("/root/verify_jwt/token.txt", &jwt).unwrap();
    println!("Generated JWS token saved to token.txt");
}
