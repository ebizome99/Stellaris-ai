# Stellaris AI

AI图片生成系统，支持本地GPU和云端混合算力调度。

## 功能特性

### 双引擎架构
- **本地引擎**: 支持多GPU并行推理，8GB低显存优化
- **云端引擎**: 支持OpenAI、Stability AI等多个云端API
- **混合调度**: 智能调度本地和云端资源

### 多GPU调度
- 自动识别多GPU设备
- 支持多种调度策略 (轮询、最小负载、显存优先)
- GPU状态实时监控
- 高优先级任务抢占


## 技术栈

### 前端
- Tauri v2
- Next.js 14
- React 18
- TypeScript (strict)
- Fluent Design System

### 后端
- Rust (stable)
- Tokio 异步运行时
- Axum Web框架
- Candle ML框架

## 开发

### 环境要求
- Node.js 20+
- Rust 1.75+
- Windows 10/11 (MSVC)

### 安装依赖
```bash
npm install
```

### 开发模式
```bash
npm run tauri:dev
```

### 构建
```bash
npm run tauri:build
```

## 项目结构

```
Stellaris-ai/
├── src/                    # Next.js 前端
│   ├── app/                # App Router
│   ├── components/         # React 组件
│   │   ├── features/       # 功能组件
│   │   ├── layout/         # 布局组件
│   │   └── ui/             # UI组件
│   ├── hooks/              # React Hooks
│   ├── stores/             # Zustand 状态
│   └── styles/             # 样式文件
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── core/           # 核心模块
│   │   ├── engine/         # 推理引擎
│   │   ├── scheduler/      # 调度器
│   │   ├── cloud/          # 云端API
│   │   ├── security/       # 安全模块
│   │   └── commands/       # Tauri命令
│   └── Cargo.toml
├── docs/                   # 文档
└── .github/workflows/      # CI/CD
```

## 许可证

MIT License
