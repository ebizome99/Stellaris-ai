# GPU 调度器文档

## 概述

GPU调度器负责管理多GPU资源，实现负载均衡和任务分配。

## 功能特性

### 1. GPU自动识别
- 使用NVML检测NVIDIA GPU
- 获取GPU名称、显存、计算能力等信息
- 无GPU时提供模拟模式

### 2. 调度策略

#### Round Robin (轮询)
```rust
// 按顺序分配GPU
GPU0 → GPU1 → GPU2 → GPU0 → ...
```

#### Least Load (最小负载)
```rust
// 选择任务最少的GPU
选择 active_tasks 最小的GPU
```

#### VRAM First (显存优先)
```rust
// 选择显存最充足的GPU
选择 free_memory_mb >= required 的GPU中最大的
```

#### Manual (手动绑定)
```rust
// 用户手动指定GPU
使用用户预设的GPU绑定
```

### 3. GPU亲和性
- 任务可以绑定到特定GPU
- 模型可以预加载到指定GPU
- 支持GPU独占模式

### 4. 高优先级任务抢占
```rust
// 优先级队列
Realtime > High > Normal > Low

// 高优先级任务可以：
// 1. 插队到队列前端
// 2. 抢占低优先级任务的GPU资源
```

## 状态监控

### GPU状态结构
```rust
pub struct GPUStatus {
    pub id: GPUId,
    pub name: String,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub free_memory_mb: u64,
    pub memory_utilization: f32,
    pub gpu_utilization: f32,
    pub temperature: Option<u32>,
    pub active_tasks: usize,
    pub queue_length: usize,
    pub available: bool,
}
```

### 监控指标
- 显存使用率: `used_memory_mb / total_memory_mb`
- GPU利用率: NVML获取的GPU使用率
- 温度: GPU核心温度
- 任务数: 活跃任务和队列长度

## 资源分配

### 显存估算
```rust
fn estimate_vram_usage(params: &GenerationParams) -> u64 {
    let base_model_vram = 4_500; // MB, SDXL基础
    let pixels = params.width * params.height;
    let activation_vram = pixels * 3 * 4 / 1024 / 1024;
    let batch_multiplier = params.batch_size;
    
    (base_model_vram + activation_vram) * batch_multiplier
}
```

### 分配流程
```
1. 估算任务显存需求
2. 选择调度策略
3. 查找可用GPU
4. 分配GPU资源
5. 更新GPU状态
6. 执行任务
7. 释放GPU资源
```

## 多卡并行

### 批量并行
```rust
// 多GPU同时生成不同批次
GPU0: batch[0..n]
GPU1: batch[n..2n]
GPU2: batch[2n..3n]
```

### 模型并行 (未来)
```rust
// 大模型分割到多GPU
GPU0: Layer 0-25
GPU1: Layer 26-50
GPU2: Layer 51-75
GPU3: Layer 76-100
```

## 故障恢复

### GPU丢失
1. 检测GPU不可用
2. 将任务转移到其他GPU
3. 无可用GPU时切换到云端

### 显存不足
1. 触发低显存模式
2. 降低分辨率/批量大小
3. 启用CPU Offload
4. 切换到云端

## 配置示例

```toml
[gpu]
enabled = true
low_vram_mode = false
vram_limit_mb = 0  # 0表示自动检测
fp16 = true
auto_mixed_precision = true
cpu_offload = false
default_batch_size = 1
max_resolution = [2048, 2048]
scheduling_strategy = "least_load"  # round_robin, least_load, vram_first, manual
```
