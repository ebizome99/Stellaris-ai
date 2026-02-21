# Stellaris AI 系统架构文档

## 概述

Stellaris AI 是一个AI图片生成系统，采用Clean Architecture和DDD设计原则，支持本地GPU和云端混合算力调度。

## 架构设计

### 分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                      Presentation Layer                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Next.js   │  │   React     │  │  Fluent Design UI   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                      Application Layer                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Commands  │  │   Events    │  │   Tauri Bridge     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                        Domain Layer                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Models    │  │   Services  │  │   Schedulers       │  │
│  │  (Entities) │  │ (Use Cases) │  │   (Policies)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                     Infrastructure Layer                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ LocalEngine │  │ CloudEngine │  │   GPU Scheduler    │  │
│  │   (Candle)  │  │  (REST API) │  │    (NVML/CUDA)     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 核心模块

#### 1. 核心层 (Core)
- **config**: 应用配置管理
- **error**: 统一错误处理
- **types**: 核心类型定义
- **events**: 事件系统

#### 2. 引擎层 (Engine)
- **ModelProvider Trait**: 统一的模型提供者接口
- **LocalEngine**: 本地GPU推理引擎
- **CloudEngine**: 云端API引擎

#### 3. 调度层 (Scheduler)
- **GPUScheduler**: 多GPU资源调度
- **HybridComputeScheduler**: 混合算力调度
- **TaskQueue**: 优先级任务队列

#### 4. 安全层 (Security)
- **encryption**: AES-256-GCM加密
- **sanitize**: 日志脱敏

## 依赖反转原则

所有核心业务逻辑依赖于抽象接口（Trait），而非具体实现：

```rust
// ModelProvider Trait - 抽象接口
pub trait ModelProvider: Send + Sync {
    async fn generate(&self, task_id: TaskId, params: GenerationParams) -> Result<GenerationResult>;
    async fn cancel(&self, task_id: TaskId) -> Result<()>;
    async fn is_available(&self) -> bool;
}

// 本地引擎实现
impl ModelProvider for LocalEngine { ... }

// 云端引擎实现  
impl ModelProvider for CloudEngine { ... }

// 调度器依赖抽象
pub struct HybridComputeScheduler {
    local_engine: Arc<dyn ModelProvider>,
    cloud_engine: Arc<dyn ModelProvider>,
}
```

## 数据流

```
用户请求
    │
    ▼
┌──────────────┐
│   Frontend   │ React + Next.js
└──────┬───────┘
       │ Tauri IPC
       ▼
┌──────────────┐
│   Commands   │ Rust Tauri Commands
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Scheduler  │ HybridComputeScheduler
└──────┬───────┘
       │
       ├──▶ LocalEngine ──▶ GPU ──▶ Candle
       │
       └──▶ CloudEngine ──▶ HTTP ──▶ OpenAI/Stability
```

## 可扩展性设计

### 插件化模型体系
- 模型通过ModelProvider Trait抽象
- 新模型只需实现Trait即可接入
- 支持动态加载和卸载

### 微服务演进
- 模块化单体设计
- 各模块通过消息队列通信
- 可独立拆分为微服务

### 水平扩展
- 无状态调度器
- 任务队列支持分布式
- 云端API天然支持扩展
