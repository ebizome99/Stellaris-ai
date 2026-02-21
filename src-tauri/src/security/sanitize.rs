//! 日志脱敏模块
//! 
//! 自动脱敏日志中的敏感信息

use regex::Regex;

/// 敏感信息模式
lazy_static::lazy_static! {
    static ref API_KEY_PATTERN: Regex = Regex::new(
        r"(sk-[a-zA-Z0-9]{20,}|api[_-]?key[_-]?[=:]\s*[\w-]+|bearer\s+[a-zA-Z0-9_-]+)"
    ).unwrap();
    
    static ref PASSWORD_PATTERN: Regex = Regex::new(
        r"(password[_-]?[=:]\s*[\w-]+|pwd[_-]?[=:]\s*[\w-]+)"
    ).unwrap();
    
    static ref EMAIL_PATTERN: Regex = Regex::new(
        r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
    ).unwrap();
    
    static ref IP_PATTERN: Regex = Regex::new(
        r"\b(?:\d{1,3}\.){3}\d{1,3}\b"
    ).unwrap();
}

/// 脱敏日志文本
pub fn sanitize_log(text: &str) -> String {
    let mut result = text.to_string();
    
    result = API_KEY_PATTERN.replace_all(&result, "[API_KEY_REDACTED]").to_string();
    result = PASSWORD_PATTERN.replace_all(&result, "[PASSWORD_REDACTED]").to_string();
    
    result = EMAIL_PATTERN.replace_all(&result, |caps: &regex::Captures| {
        let email = &caps[0];
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() == 2 {
            let local = parts[0];
            let domain = parts[1];
            let masked_local = if local.len() > 2 {
                format!("{}***{}", &local[..1], &local[local.len()-1..])
            } else {
                "***".to_string()
            };
            format!("{}@{}", masked_local, domain)
        } else {
            "[EMAIL_REDACTED]".to_string()
        }
    }).to_string();
    
    result = IP_PATTERN.replace_all(&result, "[IP_REDACTED]").to_string();
    
    result
}

/// 脱敏JSON
pub fn sanitize_json(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map {
                let key_lower = k.to_lowercase();
                if key_lower.contains("key") 
                    || key_lower.contains("token") 
                    || key_lower.contains("secret")
                    || key_lower.contains("password")
                    || key_lower.contains("credential")
                {
                    new_map.insert(k.clone(), serde_json::Value::String("[REDACTED]".into()));
                } else {
                    new_map.insert(k.clone(), sanitize_json(v));
                }
            }
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(sanitize_json).collect())
        }
        serde_json::Value::String(s) => {
            serde_json::Value::String(sanitize_log(s))
        }
        _ => value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sanitize_api_key() {
        let log = "Using API key: sk-1234567890abcdefghij for request";
        let sanitized = sanitize_log(log);
        assert!(sanitized.contains("[API_KEY_REDACTED]"));
        assert!(!sanitized.contains("sk-1234567890abcdefghij"));
    }
    
    #[test]
    fn test_sanitize_email() {
        let log = "User test@example.com logged in";
        let sanitized = sanitize_log(log);
        assert!(sanitized.contains("t***t@example.com"));
        assert!(!sanitized.contains("test@example.com"));
    }
    
    #[test]
    fn test_sanitize_ip() {
        let log = "Request from 192.168.1.100";
        let sanitized = sanitize_log(log);
        assert!(sanitized.contains("[IP_REDACTED]"));
        assert!(!sanitized.contains("192.168.1.100"));
    }
}
