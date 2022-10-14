use wasi_crypto_guest::error::Error as WasiCryptoError;
use wasi_crypto_guest::symmetric::{SymmetricOptions, SymmetricKey, SymmetricState};

fn main() -> Result<(), WasiCryptoError> {

     symmetric_basic()?;
     symmetric_session()?;

    Ok(())
}

fn symmetric_basic() -> Result<(), WasiCryptoError> {
    println!("Basic symmetric crypto");

    const ALGORITHM: &str = "AES-256-GCM";
    let mut options = SymmetricOptions::new();
    let nonce = [0u8; 12];
    options.set("nonce", nonce)?;
    let key = SymmetricKey::generate(ALGORITHM, Some(&options))?;
    let mut state = SymmetricState::new(ALGORITHM, Some(&key), Some(&options))?;
    let ciphertext = state.encrypt(b"test")?;
    let mut state = SymmetricState::new(ALGORITHM, Some(&key), Some(&options))?;
    let decrypted = state.decrypt(&ciphertext)?;

    if !decrypted.eq(b"test") {
        println!("Error decrypting.")
    }

    Ok(())
}

fn symmetric_session() -> Result<(), WasiCryptoError> {
    println!("Session based symmetric crypto");

    const ALGORITHM: &str = "AES-256-GCM";
    let mut options = SymmetricOptions::new();
    let mut nonce = [0u8; 12];
    options.set("nonce", nonce)?;

    let key = SymmetricKey::generate(ALGORITHM, Some(&options))?;
    let mut state = SymmetricState::new(ALGORITHM, Some(&key), Some(&options))?;
    let c1 = state.encrypt(b"test")?;
    let mut state = SymmetricState::new(ALGORITHM, Some(&key), Some(&options))?;
    state.decrypt(&c1)?;

    nonce[0] += 1;
    options.set("nonce", nonce)?;
    let mut state = SymmetricState::new(ALGORITHM, Some(&key), Some(&options))?;
    let c2 = state.encrypt(b"test")?;
    let mut state = SymmetricState::new(ALGORITHM, Some(&key), Some(&options))?;
    state.decrypt(&c2)?;

    if c1 == c2 {
        println!("Error: ciphertext from different nonces should not match");
    }

    Ok(())
}
