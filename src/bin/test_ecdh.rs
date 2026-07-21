use std::fs;
use p384::SecretKey;
use p384::elliptic_curve::pkcs8::DecodePrivateKey;
use p384::elliptic_curve::ecdh::diffie_hellman;

fn main() {
    // 1. Load keys
    let server_pem = fs::read_to_string("/root/verify_jwt/server.pem").unwrap();
    let client_pem = fs::read_to_string("/root/verify_jwt/client.pem").unwrap();

    let server_secret = SecretKey::from_pkcs8_pem(&server_pem).unwrap();
    let client_secret = SecretKey::from_pkcs8_pem(&client_pem).unwrap();
    
    let client_pub = client_secret.public_key();

    // 2. Compute shared secret using p384 ECDH
    let shared = diffie_hellman(
        server_secret.to_nonzero_scalar(),
        client_pub.as_affine(),
    );
    let rust_shared_bytes = shared.raw_secret_bytes();
    
    // 3. Read Go shared secret
    let go_shared_bytes = fs::read("/root/verify_jwt/shared_secret.bin").unwrap();

    println!("Rust computed len: {}", rust_shared_bytes.len());
    println!("Go computed len:   {}", go_shared_bytes.len());

    println!("Rust: {:x}", rust_shared_bytes);
    println!("Go:   {:02x?}", go_shared_bytes);

    if &rust_shared_bytes[..] == go_shared_bytes.as_slice() {
        println!("ECDH shared secret matches PERFECTLY!");
    } else {
        println!("ECDH shared secret MISMATCH!");
    }
}
