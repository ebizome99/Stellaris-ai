# 安全文档

## 概述

Stellaris AI 实现企业级安全措施，保护敏感数据和系统稳定性。

## API密钥加密

### AES-256-GCM 加密

所有API密钥使用AES-256-GCM加密存储：

```rust
// 加密
let encrypted = encrypt(api_key.as_bytes())?;

// 解密
let decrypted = decrypt(&encrypted)?;
let api_key = String::from_utf8(decrypted)?;
```

### 密钥派生

```rust
// 从环境变量获取主密钥
let master_key = env::var("STELLARIS_KEY")
    .unwrap_or_else(|_| "default-key-change-in-production".to_string());

// 生产环境必须设置环境变量
```

### 安全存储

```rust
// 加密后存储到配置文件
pub fn set_api_key(&self, provider: &str, api_key: &str) -> Result<()> {
    let encrypted = encrypt(api_key.as_bytes())?;
    
    // 存储加密数据
    self.encrypted_keys.write().push(EncryptedKey {
        provider: provider.to_string(),
        encrypted_data: encrypted,
    });
    
    Ok(())
}
```

## 日志脱敏

### 自动脱敏规则

```rust
pub fn sanitize_log(text: &str) -> String {
    // 1. API密钥脱敏
    // sk-1234567890abcdefghij → [API_KEY_REDACTED]
    
    // 2. 密码脱敏
    // password=admin123 → [PASSWORD_REDACTED]
    
    // 3. 邮箱部分脱敏
    // test@example.com → t***t@example.com
    
    // 4. IP地址脱敏
    // 192.168.1.100 → [IP_REDACTED]
}
```

### JSON脱敏

```rust
pub fn sanitize_json(value: &Value) -> Value {
    // 自动检测并脱敏包含以下关键词的字段:
    // - key, token, secret, password, credential
    
    // 示例:
    // { "api_key": "sk-xxx" } → { "api_key": "[REDACTED]" }
}
```

## 插件沙箱

### 沙箱隔离

```rust
pub struct PluginSandbox {
    // 限制文件系统访问
    allowed_paths: Vec<PathBuf>,
    
    // 限制网络访问
    allowed_hosts: Vec<String>,
    
    // 资源限制
    max_memory_mb: u64,
    max_cpu_percent: u8,
}
```

### 权限控制

```rust
pub enum PluginPermission {
    FileSystem { read: bool, write: bool },
    Network { outbound: bool },
    GPU { access: bool },
    Process { spawn: bool },
}

// 插件必须声明所需权限
#[plugin_meta]
permissions = [PluginPermission::GPU { access: true }]
```

## 进程隔离

### 推理进程隔离

```rust
// 本地推理在独立进程中运行
pub struct IsolatedProcess {
    // 独立进程ID
    pid: u32,
    
    // 进程间通信
    channel: IpcChannel,
    
    // 资源限制
    limits: ProcessLimits,
}
```

### 崩溃恢复

```rust
// 监控推理进程
impl ProcessMonitor {
    fn watch(&self) {
        loop {
            if !self.process.is_alive() {
                // 1. 记录崩溃信息
                self.log_crash();
                
                // 2. 清理资源
                self.cleanup();
                
                // 3. 自动重启
                self.restart();
            }
        }
    }
}
```

## 输入验证

### 参数验证

```rust
pub fn validate_params(params: &GenerationParams) -> Result<()> {
    // 分辨率限制
    ensure!(params.width >= 256 && params.width <= 4096);
    ensure!(params.height >= 256 && params.height <= 4096);
    
    // 步数限制
    ensure!(params.steps >= 1 && params.steps <= 150);
    
    // CFG限制
    ensure!(params.cfg_scale >= 1.0 && params.cfg_scale <= 30.0);
    
    // 批量限制
    ensure!(params.batch_size >= 1 && params.batch_size <= 16);
    
    // 提示词长度
    ensure!(params.prompt.len() <= 10000);
    
    Ok(())
}
```

### SQL注入防护

```rust
// 使用参数化查询
pub fn save_task(&self, task: &TaskInfo) -> Result<()> {
    let query = "INSERT INTO tasks (id, params, status) VALUES (?1, ?2, ?3)";
    self.conn.execute(query, params![task.id, task.params, task.status])?;
    Ok(())
}
```

## 网络安全

### HTTPS强制

```rust
// 所有云端API使用HTTPS
let client = reqwest::Client::builder()
    .https_only(true)
    .build()?;
```

### 证书验证

```rust
// 启用证书验证
let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(false)
    .build()?;
```

### CSP策略

```json
{
  "security": {
    "csp": "default-src 'self'; img-src 'self' data: blob:; connect-src 'self' https: wss:; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'"
  }
}
```

## 安全配置

```toml
[security]
encrypt_api_keys = true      # 加密API密钥
sanitize_logs = true         # 日志脱敏
plugin_sandbox = true        # 插件沙箱
process_isolation = true     # 进程隔离
```

## 安全最佳实践

### 禁止事项
- ❌ 明文存储API密钥
- ❌ 在日志中记录敏感信息
- ❌ 主线程执行阻塞操作
- ❌ 未处理的异常传播
- ❌ 直接执行用户输入

### 必须事项
- ✅ 所有密钥加密存储
- ✅ 日志自动脱敏
- ✅ 输入参数验证
- ✅ 异常统一处理
- ✅ 定期安全审计

## 安全审计日志

```rust
pub struct AuditLog {
    timestamp: DateTime<Utc>,
    event_type: AuditEventType,
    user: Option<String>,
    details: String,
}

pub enum AuditEventType {
    ApiKeyAccessed,
    ConfigChanged,
    PluginLoaded,
    ProcessCrashed,
    SecurityViolation,
}
```

## 更新日志

| 日期 | 版本 | 变更 |
|-----|------|------|
| 2024-01 | 0.1.0 | 初始安全实现 |
