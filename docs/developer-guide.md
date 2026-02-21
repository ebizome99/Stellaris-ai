# 开发者指南

## 环境设置

### 系统要求

- Windows 10/11 (MSVC)
- Node.js 20+
- Rust 1.75+
- Visual Studio 2022 (C++ 工具链)

### 安装依赖

```bash
# 克隆仓库
git clone https://github.com/ebizome99/Stellaris-ai.git
cd Stellaris-ai

# 安装Node.js依赖
npm install

# 检查Rust环境
cargo --version
rustc --version
```

### 开发模式

```bash
# 启动开发服务器
npm run tauri:dev
```

### 构建

```bash
# 构建生产版本
npm run tauri:build
```

## 项目结构

```
Stellaris-ai/
├── src/                      # Next.js 前端
│   ├── app/                  # App Router 页面
│   │   ├── layout.tsx        # 根布局
│   │   └── page.tsx          # 首页
│   ├── components/
│   │   ├── features/         # 功能组件
│   │   │   ├── GeneratorPanel.tsx
│   │   │   ├── GalleryPanel.tsx
│   │   │   ├── GPUMonitor.tsx
│   │   │   └── SettingsPanel.tsx
│   │   ├── layout/           # 布局组件
│   │   │   ├── MainLayout.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Header.tsx
│   │   └── ui/               # 通用UI组件
│   ├── hooks/                # React Hooks
│   ├── stores/               # Zustand 状态管理
│   ├── lib/                  # 工具函数
│   └── styles/               # 样式文件
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── main.rs           # 入口
│   │   ├── lib.rs            # 库入口
│   │   ├── core/             # 核心模块
│   │   │   ├── config.rs     # 配置
│   │   │   ├── error.rs      # 错误
│   │   │   └── types.rs      # 类型
│   │   ├── engine/           # 引擎
│   │   │   ├── local.rs      # 本地引擎
│   │   │   ├── cloud.rs      # 云端引擎
│   │   │   └── provider.rs   # 提供者抽象
│   │   ├── scheduler/        # 调度器
│   │   │   ├── gpu.rs        # GPU调度
│   │   │   ├── hybrid.rs     # 混合调度
│   │   │   └── task_queue.rs # 任务队列
│   │   ├── security/         # 安全
│   │   │   ├── encryption.rs # 加密
│   │   │   └── sanitize.rs   # 脱敏
│   │   ├── cloud/            # 云端API
│   │   │   ├── openai.rs
│   │   │   └── stability.rs
│   │   ├── models/           # 模型管理
│   │   ├── commands/         # Tauri命令
│   │   └── utils/            # 工具
│   ├── Cargo.toml            # Rust依赖
│   └── tauri.conf.json       # Tauri配置
├── docs/                     # 文档
├── .github/workflows/        # CI/CD
└── package.json              # Node依赖
```

## 编码规范

### TypeScript

```typescript
// 严格模式
// tsconfig.json
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitAny": true
  }
}

// 使用类型定义
interface GenerationParams {
  prompt: string;
  width: number;
  height: number;
}

// 避免any
function process(params: GenerationParams): Result { ... }
```

### Rust

```rust
// 使用 Clippy 检查
cargo clippy -- -D warnings

// 文档注释
/// 生成图像
/// 
/// # Arguments
/// * `params` - 生成参数
/// 
/// # Returns
/// 生成结果
pub async fn generate(params: GenerationParams) -> Result<GenerationResult> {
    // ...
}

// 错误处理
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("GPU错误: {0}")]
    GPU(String),
}
```

## 添加新功能

### 1. 添加新的云端提供商

```rust
// src-tauri/src/cloud/new_provider.rs
use async_trait::async_trait;
use crate::engine::provider::ModelProvider;

pub struct NewProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

#[async_trait]
impl ModelProvider for NewProvider {
    fn name(&self) -> &str { "NewProvider" }
    
    async fn generate(&self, task_id: TaskId, params: GenerationParams) -> Result<GenerationResult> {
        // 实现生成逻辑
    }
}
```

### 2. 添加新的UI组件

```typescript
// src/components/features/NewFeature.tsx
'use client';

export function NewFeature() {
  return (
    <div className="card p-6">
      {/* 组件内容 */}
    </div>
  );
}
```

### 3. 添加新的Tauri命令

```rust
// src-tauri/src/commands/mod.rs
#[tauri::command]
pub async fn new_command(param: String) -> Result<String> {
    // 实现命令逻辑
    Ok("result".to_string())
}

// 在 main.rs 中注册
.invoke_handler(tauri::generate_handler![
    // ...其他命令
    commands::new_command,
])
```

## 调试

### 前端调试

```bash
# 开发模式自动打开DevTools
npm run tauri:dev
```

### 后端调试

```rust
// 使用日志
tracing::info!("信息日志");
tracing::debug!("调试日志");
tracing::error!("错误日志");

// 设置日志级别
RUST_LOG=debug npm run tauri:dev
```

### GPU调试

```rust
// 检查GPU状态
let status = gpu_scheduler.get_all_gpu_status();
for gpu in status {
    tracing::info!("GPU {}: {} MB / {} MB", 
        gpu.id, gpu.used_memory_mb, gpu.total_memory_mb);
}
```

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let original = "test-key";
        let encrypted = encrypt(original.as_bytes()).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original.as_bytes(), decrypted.as_slice());
    }
}
```

### 运行测试

```bash
# Rust测试
cd src-tauri
cargo test

# 前端测试
npm test
```

## 发布流程

1. 更新版本号
   - `package.json`
   - `src-tauri/Cargo.toml`
   - `src-tauri/tauri.conf.json`

2. 更新CHANGELOG

3. 创建Git标签
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

4. GitHub Actions自动构建发布

## 常见问题

### Q: 编译报错 "linker 'link.exe' not found"
A: 安装 Visual Studio 2022，选择 C++ 工具链

### Q: GPU检测失败
A: 确保安装了NVIDIA驱动，或使用 `cpu-only` feature

### Q: 前端热重载不工作
A: 检查端口3000是否被占用
