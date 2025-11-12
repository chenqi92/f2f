# F2F Converter

一体化多格式互转桌面软件 - 高保真、可编排、可批量的跨平台格式转换工具

## 特性

- 🚀 **拖拽即用** - 简单直观的文件拖放操作
- 🎯 **常用预设** - 一键完成常见转换任务
- 🧪 **食谱编排** - 通过可视化 DAG 编排复杂转换流程
- 📊 **透明可解释** - 清晰展示转换路径、质量评分和潜在风险
- 🔧 **插件扩展** - 灵活的插件系统支持自定义转换器
- 🌍 **跨平台** - 支持 Windows、macOS、Linux (x86/ARM)

## 支持的转换

### 文档
- DOC/DOCX ↔ Markdown/HTML/PDF
- PDF (含扫描) → DOCX/Markdown (OCR)
- HTML ↔ Markdown/PDF/DOCX

### 图片
- JPG/PNG/TIFF ↔ WebP/AVIF
- 位图 → SVG (矢量化)

### 音视频
- 容器/编码互转 (通过 FFmpeg)
- 4K → 1080p + 字幕处理
- 字幕抽取/嵌入

### 数据
- CSV/TSV ↔ JSON/YAML/XML
- PDF 表格 → CSV (结构化提取)

## 技术栈

- **内核**: Rust (转换规划、DAG 执行、沙箱、缓存)
- **桌面壳**: Tauri 2.0
- **前端**: React + Vite + TypeScript + Tailwind CSS
- **状态管理**: Zustand + Jotai
- **数据获取**: TanStack Query
- **UI 组件**: shadcn/ui (Radix)

## 项目结构

```
f2f/
├── src/                    # 前端代码
│   ├── components/         # React 组件
│   ├── pages/              # 页面组件
│   ├── lib/                # 工具库
│   ├── hooks/              # 自定义 Hooks
│   ├── types/              # TypeScript 类型定义
│   ├── store/              # 状态管理
│   └── styles/             # 样式文件
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── core/           # 核心模块
│   │   │   ├── registry.rs # 能力注册表
│   │   │   ├── planner.rs  # 转换规划器
│   │   │   ├── pipeline.rs # 流水线执行器
│   │   │   ├── adapter.rs  # 工具适配器
│   │   │   ├── storage.rs  # 存储管理
│   │   │   └── quality.rs  # 质量评估
│   │   ├── types/          # 类型定义
│   │   ├── commands/       # Tauri 命令
│   │   ├── error.rs        # 错误处理
│   │   └── main.rs         # 主入口
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

## 核心架构

### 分层设计

```
┌─────────────────────────────────────────┐
│          UI 层 (React)                  │
│  页面 | 组件 | 预设/食谱编辑器 | 预览   │
└─────────────────────────────────────────┘
                    ↓ IPC
┌─────────────────────────────────────────┐
│        Tauri 命令/事件                  │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│          Rust 内核                      │
├─────────────────────────────────────────┤
│ • 转换规划器 (Planner)                  │
│ • 流水线执行器 (Pipeline)               │
│ • 工具适配器 (Adapter)                  │
│ • 能力注册表 (Registry)                 │
│ • 质量评估 (QA)                         │
│ • 存储管理 (Storage)                    │
└─────────────────────────────────────────┘
```

### 能力注册表

每个转换器声明其能力：
- 输入/输出格式
- 质量评分 (0-1)
- 速度/成本
- 依赖要求
- 保留特性
- 潜在风险

注册表基于这些信息构建**转换图**，用于路径规划和质量评估。

### 转换规划

1. **探测**: 识别文件 MIME 类型和特征
2. **匹配**: 在转换图中查找可达路径
3. **评分**: 综合质量、速度、成本排序
4. **解释**: 展示路径、风险、依赖缺失

### 路由与回退

- **直达优先**: 一步到位的转换
- **质量门槛**: 未达标自动切换更优路径
- **多候选**: 保留备用方案
- **失败回退**: 工具异常时切换等价工具

## 开发指南

### 前置要求

- Node.js 18+
- Rust 1.70+
- pnpm (推荐) 或 npm/yarn

### 安装依赖

```bash
# 安装前端依赖
pnpm install

# Rust 依赖会在构建时自动安装
```

### 开发模式

```bash
# 启动开发服务器 (热重载)
pnpm tauri:dev
```

### 构建

```bash
# 构建前端
pnpm build

# 构建 Tauri 应用
pnpm tauri:build
```

### 代码规范

```bash
# 前端 Lint
pnpm lint

# 前端格式化
pnpm format

# Rust 格式化
cd src-tauri && cargo fmt
```

## IPC 契约

前端通过类型安全的 IPC 调用与 Rust 后端通信：

```typescript
import { detectCapabilities, createJob } from "@/lib/tauri";

// 探测系统能力
const capabilities = await detectCapabilities();

// 创建转换任务
const jobId = await createJob(
  ["/path/to/file.docx"],
  "markdown",
  undefined,
  { quality_priority: "balanced" }
);
```

完整 API 文档见 `src/types/ipc.ts`。

## 质量评估

系统会评估转换结果的质量指标：

- **OCR**: 字符准确率 (CER) ≥ 95%
- **文档结构**: 标题/段落/表格一致度 ≥ 90%
- **表格召回**: ≥ 90%
- **媒体码率**: 偏差 ≤ ±10%

未达标时自动切换更高质量路径或给出改进建议。

## 工具集成

### 必集工具 (内置或首启安装)

- **Pandoc**: 文档格式互转中枢
- **LibreOffice**: Office ↔ PDF/ODF
- **FFmpeg**: 音视频处理
- **ImageMagick/libvips**: 图片处理
- **Ghostscript**: PDF 处理
- **Tesseract OCR**: 文本识别

### 可选增强 (插件)

- **PaddleOCR/EasyOCR**: 高精度 OCR
- **Tabula/Camelot**: PDF 表格提取
- **wkhtmltopdf**: HTML→PDF 高保真
- **本地/云端 LLM**: OCR 纠错、结构修复

## 插件开发

TBD

## 许可证

MIT

## 贡献指南

欢迎贡献！请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 路线图

- [x] M1: 项目初始化和核心架构
- [ ] M2: 快速转换功能闭环
- [ ] M3: 食谱构建器
- [ ] M4: 插件系统和质量评估
- [ ] M5: OCR/LLM 集成
- [ ] M6: 性能优化和打包分发
