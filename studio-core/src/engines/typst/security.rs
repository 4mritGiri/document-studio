// src/engines/typst/security.rs

use crate::domain::SecurityConfig;

use lopdf::encryption::crypt_filters::{Aes256CryptFilter, CryptFilter};
use lopdf::{Document, EncryptionState, EncryptionVersion, Permissions};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn apply_security(pdf_bytes: Vec<u8>, config: &SecurityConfig) -> Result<Vec<u8>, String> {
    let mut doc = Document::load_mem(&pdf_bytes).map_err(|e| format!("Failed to load PDF: {e}"))?;

    let user_pwd = config.user_password.as_deref().unwrap_or("");
    let owner_pwd = config.owner_password.as_deref().unwrap_or(user_pwd);

    // Generate 32-byte key using existing dependencies (uuid + sha2)
    let mut hasher = Sha256::new();
    hasher.update(Uuid::new_v4().as_bytes());
    hasher.update(Uuid::new_v4().as_bytes());
    let key: [u8; 32] = hasher.finalize().into();

    // FIX: Now we can use the imported CryptFilter trait
    let crypt_filter: Arc<dyn CryptFilter> = Arc::new(Aes256CryptFilter);

    let version = EncryptionVersion::V5 {
        encrypt_metadata: true,
        crypt_filters: BTreeMap::from([(b"StdCF".to_vec(), crypt_filter)]),
        file_encryption_key: &key,
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        owner_password: owner_pwd,
        user_password: user_pwd,
        permissions: Permissions::all(),
    };

    let state = EncryptionState::try_from(version)
        .map_err(|e| format!("Failed to create encryption state: {e}"))?;

    doc.encrypt(&state)
        .map_err(|e| format!("Encryption failed: {e}"))?;

    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {e}"))?;

    Ok(output)
}
