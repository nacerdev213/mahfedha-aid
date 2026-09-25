use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use aes_gcm::aead::rand_core::RngCore;
use sha2::{Digest, Sha256};

pub const MAGIC_HEADER: &[u8; 4] = b"SCA1";
pub const NONCE_LEN: usize = 12;

/// Derives a consistent 256-bit key for AES-256-GCM
fn get_app_key() -> Key<Aes256Gcm> {
    const SECRET_SEED: &[u8] = b"SchoolAidApp::Security::EncryptedBackup::v1::2026-dz-algeria-scaid";
    let mut hasher = Sha256::new();
    hasher.update(SECRET_SEED);
    let result = hasher.finalize();
    *Key::<Aes256Gcm>::from_slice(&result)
}

/// Encrypts binary plaintext using AES-256-GCM.
/// Output format: [ MAGIC_HEADER (4 bytes) | Nonce (12 bytes) | Ciphertext + Tag ]
pub fn encrypt_buffer(plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let key = get_app_key();
    let cipher = Aes256Gcm::new(&key);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| format!("فشل تشفير البيانات: {}", e))?;

    let mut output = Vec::with_capacity(MAGIC_HEADER.len() + NONCE_LEN + ciphertext.len());
    output.extend_from_slice(MAGIC_HEADER);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

/// Decrypts AES-256-GCM encrypted binary buffer.
pub fn decrypt_buffer(encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < MAGIC_HEADER.len() + NONCE_LEN + 16 {
        return Err("الملف المشفر تالف أو حجمه غير صالح (أقصر من الحد الأدنى)".to_string());
    }

    if &encrypted_data[..MAGIC_HEADER.len()] != MAGIC_HEADER {
        return Err("صيغة الملف غير صالحة: هذا الملف ليس أرشيفاً مشفراً صادراً عن هذا البرنامج".to_string());
    }

    let nonce_start = MAGIC_HEADER.len();
    let nonce_end = nonce_start + NONCE_LEN;
    let nonce_bytes = &encrypted_data[nonce_start..nonce_end];
    let ciphertext = &encrypted_data[nonce_end..];

    let key = get_app_key();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "فشل فك تشفير النسخة الاحتياطية: المفتاح غير متطابق أو أن الملف تالف ومعدل خارجياً".to_string())?;

    Ok(plaintext)
}

/// Calculates SHA256 hex string of a byte slice
pub fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
