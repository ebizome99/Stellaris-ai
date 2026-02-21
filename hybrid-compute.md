# 混合算力调度文档

## 概述

混合算力调度器负责智能调度本地GPU和云端API，实现最优的资源利用。

## 调度策略

### 1. 执行模式

```rust
pub enum ExecutionMode {
    Local,   // 强制本地
    Cloud,   // 强制云端
    Auto,    // 自动选择
}
```

### 2. 自动调度逻辑

```
                    ┌─────────────┐
                    │ 用户请求    │
                    └──────┬──────┘
                           ↓
                    ┌─────────────┐
                    │ 检查执行模式 │
                    └──────┬──────┘
                           ↓
              ┌────────────┼────────────┐
              ↓            ↓            ↓
          Local         Cloud        Auto
              ↓            ↓            ↓
        ┌─────────┐  ┌─────────┐  ┌────────────┐
        │本地可用?│  │云端可用?│  │ 智能决策   │
        └────┬────┘  └────┬────┘  └─────┬──────┘
             ↓            ↓             ↓
        执行/失败    执行/失败      选择最优
                                       ↓
                              ┌────────┴────────┐
                              ↓                 ↓
                         本地优先          云端备选
```

### 3. 智能决策因素

#### 本地负载阈值
```toml
[scheduler]
cloud_switch_threshold = 0.8  # 本地负载 > 80% 切换云端
```

#### 显存判断
```rust
if free_vram < required_vram {
    // 显存不足，切换云端
    switch_to_cloud();
}
```

#### 成本优化
```rust
// 本地免费，优先本地
// 云端按量计费，作为备用
```

## 降级策略

### 本地 → 云端
```
触发条件:
1. 本地GPU不可用
2. 显存不足
3. 负载过高
4. 本地推理失败

处理流程:
1. 记录失败原因
2. 检查云端可用性
3. 转发任务到云端
4. 统计降级次数
```

### 云端 → 本地
```
触发条件:
1. 云端API不可用
2. API限流
3. 超过配额
4. 网络错误

处理流程:
1. 记录失败原因
2. 检查本地可用性
3. 转发任务到本地
4. 延后重试云端
```

## 云端提供商

### OpenAI (DALL-E 3)
```rust
// 支持的分辨率
1024x1024, 1024x1792, 1792x1024

// 特点
- 高质量图像
- 较高成本
- 有速率限制
```

### Stability AI (SDXL)
```rust
// 支持的分辨率
灵活配置

// 特点
- 接近本地质量
- 中等成本
- 支持更多参数
```

### 私有云
```rust
// 自定义端点
endpoint = "https://your-server.com/api/generate"

// 特点
- 完全可控
- 无速率限制
- 需要自建服务
```

## 配额管理

### 每日限额
```toml
[cloud]
daily_limit_usd = 50.0
monthly_limit_usd = 1000.0
```

### 提供商限额
```toml
[[cloud.providers]]
name = "openai"
daily_limit = 20.0  # 每日20美元
rate_limit = 60     # 每分钟60次
```

### 限额监控
```rust
pub struct SchedulerStats {
    pub local_tasks: u64,
    pub cloud_tasks: u64,
    pub fallback_count: u64,
    pub cost_saved_usd: f64,  // 使用本地节省的成本
}
```

## 负载均衡

### 多账号轮换
```rust
// 配置多个API Key
providers = [
    { name = "openai-1", key = "sk-xxx1" },
    { name = "openai-2", key = "sk-xxx2" },
]

// 自动轮换使用
current_provider = (current_provider + 1) % providers.len()
```

### 优先级排序
```rust
// 按优先级排序提供商
providers.sort_by(|a, b| a.priority.cmp(&b.priority));
```

## 错误处理

### 重试策略
```rust
struct RetryPolicy {
    max_retries: u32,
    backoff_ms: u64,
    backoff_multiplier: f32,
}

// 指数退避
delay = backoff_ms * (backoff_multiplier ^ retry_count)
```

### 错误分类
```rust
enum CloudError {
    RateLimited,      // 等待后重试
    InvalidRequest,   // 不重试
    ServerError,      // 切换提供商
    NetworkError,     // 重试或降级
    QuotaExceeded,    // 切换提供商
}
```

## 成本估算

```rust
pub fn estimate_cost(params: &GenerationParams) -> f64 {
    let base_cost = 0.02;  // 基础成本
    let resolution_factor = (params.width * params.height) as f64 / (1024.0 * 1024.0);
    let batch_factor = params.batch_size as f64;
    
    base_cost * resolution_factor * batch_factor
}

// 示例: 1024x1024, batch=1 → $0.02
// 示例: 1792x1024, batch=4 → $0.14
```
