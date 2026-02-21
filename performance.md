# 性能优化文档

## 概述

本文档描述Stellaris AI的性能优化策略，重点针对8GB低显存显卡的极限优化。

## 8GB 显存优化

### 1. FP16 半精度

```rust
// 使用FP16减少显存占用
let use_fp16 = config.gpu.fp16;

// 显存节省: 约50%
// 精度损失: 极小
```

### 2. 自动混合精度 (AMP)

```rust
// 关键层使用FP32，其他使用FP16
// 自动选择最优精度
let amp = AutoMixedPrecision::new();

// 显存节省: 约40%
// 精度损失: 无
```

### 3. 模型分块加载

```rust
// 将模型分成多个块，按需加载
struct ModelChunks {
    unet: Vec<ModelChunk>,
    vae: Vec<ModelChunk>,
    text_encoder: Vec<ModelChunk>,
}

// 加载流程
for chunk in &model_chunks.unet {
    if chunk.needed_for_current_step() {
        chunk.load_to_vram();
    } else {
        chunk.offload_to_ram();
    }
}
```

### 4. CPU Offload

```rust
// 模型部分存储在CPU内存
// 推理时按需传输到GPU

struct OffloadStrategy {
    // 始终在GPU的层
    always_on_gpu: Vec<LayerId>,
    // 按需加载的层
    on_demand: Vec<LayerId>,
}

// 显存节省: 约60-70%
// 性能损失: 约30-50%
```

### 5. Low VRAM 模式

```toml
[gpu]
low_vram_mode = true
```

```rust
// 自动触发的优化
impl LowVRAMMode {
    fn optimize(&self, params: &mut GenerationParams) {
        // 1. 限制分辨率
        params.width = params.width.min(1024);
        params.height = params.height.min(1024);
        
        // 2. 限制批量大小
        params.batch_size = 1;
        
        // 3. 启用所有offload
        self.enable_all_offloads();
        
        // 4. 激进内存回收
        self.aggressive_gc();
    }
}
```

### 6. 分辨率自适应

```rust
// 根据显存自动限制分辨率
fn max_resolution_for_vram(vram_mb: u64) -> (u32, u32) {
    match vram_mb {
        0..=4096 => (512, 512),
        4097..=6144 => (768, 768),
        6145..=8192 => (1024, 1024),
        8193..=12288 => (1280, 1280),
        _ => (2048, 2048),
    }
}
```

### 7. Batch Size 自动调节

```rust
fn auto_batch_size(vram_mb: u64, resolution: (u32, u32)) -> u32 {
    let pixels = resolution.0 * resolution.1;
    let estimated_per_image = pixels * 3 * 4 / 1024 / 1024;
    let available = vram_mb - 4500; // 减去模型占用
    
    (available / estimated_per_image).max(1).min(4)
}
```

### 8. 动态显存回收

```rust
// 推理完成后立即释放
impl Model {
    fn after_inference(&mut self) {
        // 1. 清空CUDA缓存
        cuda_clear_cache();
        
        // 2. 卸载不需要的模型
        self.unload_unused_models();
        
        // 3. 压缩内存碎片
        defragment_memory();
    }
}
```

## 通用性能优化

### 1. 异步处理

```rust
// 使用Tokio异步运行时
#[tokio::main]
async fn main() {
    // 并发执行多个任务
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| tokio::spawn(async move {
            process_task(task).await
        }))
        .collect();
    
    let results = futures::future::join_all(handles).await;
}
```

### 2. LRU 缓存

```rust
// 图片缓存
let cache = LRUCache::new(100);

// 命中时直接返回
if let Some(cached) = cache.get(&prompt_hash) {
    return cached;
}

// 未命中则生成并缓存
let result = generate_image(params);
cache.put(prompt_hash, result);
```

### 3. 队列限流

```rust
// 限制并发任务数
let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));

async fn process_with_limit(task: Task) {
    let _permit = semaphore.acquire().await;
    process(task).await
}
```

### 4. 无锁数据结构

```rust
// 使用DashMap替代RwLock<HashMap>
use dashmap::DashMap;

let map: DashMap<String, Value> = DashMap::new();

// 并发安全，无需手动加锁
map.insert(key, value);
let value = map.get(&key);
```

### 5. 内存池

```rust
// 预分配内存池，避免频繁分配
struct MemoryPool {
    buffers: Vec<Buffer>,
    available: Mutex<Vec<usize>>,
}

impl MemoryPool {
    fn acquire(&self) -> Buffer {
        // 从池中获取
    }
    
    fn release(&self, buffer: Buffer) {
        // 归还到池
    }
}
```

## 性能目标

| 指标 | 目标 | 实际 |
|-----|------|------|
| 冷启动 | < 2秒 | - |
| GPU利用率 | ≥ 90% | - |
| 内存峰值 | < 可用显存 | - |
| UI响应 | < 16ms | - |
| 任务切换 | < 100ms | - |

## 性能监控

```rust
// 内置性能监控
struct PerformanceMetrics {
    fps: f32,
    frame_time_ms: f64,
    gpu_utilization: f32,
    memory_used_mb: u64,
    tasks_per_second: f32,
}

// 通过API获取
GET /api/metrics
```

## 基准测试

```bash
# 运行基准测试
cargo bench

# 测试结果示例
generate_1024x1024_fp16
    time:   [2.34 s 2.41 s 2.49 s]
    
generate_1024x1024_fp32
    time:   [4.12 s 4.28 s 4.45 s]
```
