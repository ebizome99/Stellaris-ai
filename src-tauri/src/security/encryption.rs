//! 加密模块
//! 
//! AES-256-GCM加密用于API密钥存储

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::core::error::{Error, Result};

/// 加密密钥 (32字节)
static ENCRYPTION_KEY: once_cell::sync::Lazy<[u8; 32]> = once_cell::sync::Lazy::new(|| {
    let key = std::env::var("STELLARIS_KEY")
        .unwrap_or_else(|_| "stellaris-ai-default-key-change-in-production".to_string());
    let mut key_bytes = [0u8; 32];
    let bytes = key.as_bytes();
    let len = bytes.len().min(32);
    key_bytes[..len].copy_from_slice(&bytes[..len]);
    key_bytes
});

/// 加密数据
pub fn encrypt(data: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(&*ENCRYPTION_KEY)
        .map_err(|e| Error::Crypto(format!("初始化加密器失败: {}", e)))?;
    
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| Error::Crypto(format!("加密失败: {}", e)))?;
    
    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);
    
    Ok(result)
}

/// 解密数据
pub fn decrypt(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 12 {
        return Err(Error::Crypto("数据太短".into()));
    }
    
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let cipher = Aes256Gcm::new_from_slice(&*ENCRYPTION_KEY)
        .map_err(|e| Error::Crypto(format!("初始化解密器失败: {}", e)))?;
    
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| Error::Crypto(format!("解密失败: {}", e)))
}

/// 加密字符串为Base64
pub fn encrypt_to_base64(data: &str) -> Result<String> {
    let encrypted = encrypt(data.as_bytes())?;
    Ok(BASE64.encode(&encrypted))
}

/// 从Base64解密字符串
pub fn decrypt_from_base64(data: &str) -> Result<String> {
    let decoded = BASE64.decode(data)
        .map_err(|e| Error::Crypto(format!("Base64解码失败: {}", e)))?;
    let decrypted = decrypt(&decoded)?;
    String::from_utf8(decrypted)
        .map_err(|e| Error::Crypto(format!("UTF-8解码失败: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let original = "my-secret-api-key-12345";
        let encrypted = encrypt(original.as_bytes()).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original.as_bytes(), decrypted.as_slice());
    }
    
    #[test]
    fn test_base64_roundtrip() {
        let original = "sk-test-key-abcdef";
        let encrypted = encrypt_to_base64(original).unwrap();
        let decrypted = decrypt_from_base64(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }
}
