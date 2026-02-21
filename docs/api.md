# API 文档

## 概述

Stellaris AI 通过 Tauri IPC 提供前后端通信接口。

## 生成接口

### generate_image

生成AI图像

**参数:**
```typescript
{
  prompt: string;           // 提示词
  negative_prompt?: string; // 负面提示词
  width?: number;           // 宽度 (默认 1024)
  height?: number;          // 高度 (默认 1024)
  steps?: number;           // 步数 (默认 20)
  cfg_scale?: number;       // CFG Scale (默认 7.0)
  seed?: number;            // 种子 (可选)
  batch_size?: number;      // 批量大小 (默认 1)
  model?: string;           // 模型名称 (默认 "sdxl")
}
```

**返回:**
```typescript
{
  task_id: string;
  images: string[];         // Base64编码的图像
  seeds: number[];
  inference_time_ms: number;
  execution_mode: "local" | "cloud" | "auto";
  gpu_id?: number;
  metadata: object;
}
```

**示例:**
```typescript
import { invoke } from '@tauri-apps/api';

const result = await invoke('generate_image', {
  prompt: '一个美丽的风景画',
  width: 1024,
  height: 1024,
  steps: 20,
});
```

## GPU接口

### get_gpu_status

获取所有GPU状态

**返回:**
```typescript
[{
  id: number;
  name: string;
  total_memory_mb: number;
  used_memory_mb: number;
  free_memory_mb: number;
  memory_utilization: number;
  gpu_utilization: number;
  temperature?: number;
  active_tasks: number;
  queue_length: number;
  available: boolean;
}]
```

**示例:**
```typescript
const gpuStatus = await invoke('get_gpu_status');
console.log(gpuStatus[0].name); // "NVIDIA GeForce RTX 4090"
```

## 任务接口

### get_task_status

获取任务状态

**参数:**
- `task_id: string` - 任务ID

**返回:**
```typescript
{
  id: string;
  params: GenerationParams;
  status: "pending" | "queued" | "running" | "completed" | "failed" | "cancelled";
  created_at: string;
  started_at?: string;
  completed_at?: string;
  result?: GenerationResult;
  error?: string;
  progress: number;         // 0-100
  current_step: string;
}
```

### cancel_task

取消任务

**参数:**
- `task_id: string` - 任务ID

**返回:** `void`

**示例:**
```typescript
await invoke('cancel_task', { task_id: 'xxx' });
```

## 模型接口

### get_models

获取可用模型列表

**返回:**
```typescript
[{
  id: string;
  name: string;
  model_type: "sd1x" | "sd2x" | "sdxl" | "sdxl_turbo" | "sd3" | "custom";
  path: string;
  size_mb: number;
  loaded: boolean;
  loaded_on_gpu?: number;
  vram_usage_mb?: number;
  supported_resolutions: [number, number][];
  metadata: object;
}]
```

### load_model

加载模型

**参数:**
- `model_id: string` - 模型ID

**返回:** `void`

### unload_model

卸载模型

**参数:**
- `model_id: string` - 模型ID

**返回:** `void`

## 配置接口

### get_config

获取应用配置

**返回:** `AppConfig`

### update_config

更新应用配置

**参数:**
- `config: AppConfig` - 新配置

**返回:** `void`

## 事件

### 任务进度事件

```typescript
import { listen } from '@tauri-apps/api/event';

await listen('task:progress', (event) => {
  const { task_id, progress, step, total_steps, description } = event.payload;
  console.log(`任务 ${task_id}: ${progress}%`);
});
```

### GPU状态变更事件

```typescript
await listen('gpu:status-changed', (event) => {
  const { gpu_id, change_type } = event.payload;
  console.log(`GPU ${gpu_id} 状态变更: ${change_type}`);
});
```

### 低显存警告事件

```typescript
await listen('system:low-vram-warning', (event) => {
  const { used_mb, total_mb, utilization, suggestions } = event.payload;
  console.warn(`显存不足: ${used_mb}/${total_mb}MB`);
});
```

## 错误处理

所有API调用可能返回错误:

```typescript
try {
  const result = await invoke('generate_image', params);
} catch (error) {
  console.error('生成失败:', error);
  // 错误类型: "GPU错误" | "模型错误" | "推理错误" | "云端API错误" 等
}
```

## 类型定义

```typescript
// 完整类型定义见 src/lib/types.ts
export interface GenerationParams {
  prompt: string;
  negative_prompt?: string;
  width: number;
  height: number;
  steps: number;
  cfg_scale: number;
  seed?: number;
  batch_size: number;
  sampler: string;
  model: string;
  execution_mode: ExecutionMode;
  priority: Priority;
}

export type ExecutionMode = 'local' | 'cloud' | 'auto';
export type Priority = 'low' | 'normal' | 'high' | 'realtime';
export type TaskStatus = 'pending' | 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';
```
