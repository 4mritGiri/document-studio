// src/engines/typst/security.rs

use crate::domain::SecurityConfig;

use lopdf::encryption::crypt_filters::{Aes256CryptFilter, CryptFilter};
use lopdf::{Document, EncryptionState, EncryptionVersion, Permissions};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn apply_security(pdf_bytes: Vec<u8>, config: &SecurityConfig) -> Result<Vec<u8>, String> {
    // Fail closed: if neither password is set, the previous code silently
    // produced a PDF that was cryptographically "encrypted" but required no
    // password to open and granted (via the old hardcoded Permissions::all())
    // unrestricted access anyway — i.e. it looked protected and wasn't.
    // Better to reject the request outright than ship a false sense of
    // security to a bank's customer.
    let user_pwd = config.user_password.as_deref().unwrap_or("");
    let owner_pwd_input = config.owner_password.as_deref();
    if user_pwd.is_empty() && owner_pwd_input.map(str::is_empty).unwrap_or(true) {
        return Err(
            "security config requires at least one of user_password or owner_password to be set \
             — encrypting with both empty provides no actual protection"
                .to_string(),
        );
    }
    let owner_pwd = owner_pwd_input.unwrap_or(user_pwd);

    let mut doc = Document::load_mem(&pdf_bytes).map_err(|e| format!("Failed to load PDF: {e}"))?;

    // Random 256-bit key. uuid::Uuid::new_v4() is backed by a real CSPRNG
    // (via getrandom), so hashing two of them is a reasonable way to get
    // 256 bits of randomness without adding a dedicated `rand` dependency.
    let mut hasher = Sha256::new();
    hasher.update(Uuid::new_v4().as_bytes());
    hasher.update(Uuid::new_v4().as_bytes());
    let key: [u8; 32] = hasher.finalize().into();

    let crypt_filter: Arc<dyn CryptFilter> = Arc::new(Aes256CryptFilter);

    let mut permissions = Permissions::empty();
    if config.allow_printing {
        permissions |= Permissions::PRINTABLE;
    }
    if config.allow_copying {
        permissions |= Permissions::COPYABLE;
    }
    if config.allow_modification {
        permissions |= Permissions::MODIFIABLE;
    }
    if config.allow_annotation {
        permissions |= Permissions::ANNOTABLE;
    }

    let version = EncryptionVersion::V5 {
        encrypt_metadata: true,
        crypt_filters: BTreeMap::from([(b"StdCF".to_vec(), crypt_filter)]),
        file_encryption_key: &key,
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        owner_password: owner_pwd,
        user_password: user_pwd,
        permissions,
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
