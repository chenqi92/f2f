# F2F Converter - 项目文件结构

## 📁 根目录

```
f2f/
├── 📄 README.md                    # 项目概览和使用指南
├── 📄 PROJECT_SUMMARY.md           # 详细项目总结
├── 📄 QUICKSTART.md                # 快速开始指南
├── 📄 FILE_STRUCTURE.md            # 本文件
├── 📄 package.json                 # 前端依赖配置
├── 📄 pnpm-lock.yaml               # pnpm 锁文件
├── 📄 tsconfig.json                # TypeScript 配置
├── 📄 vite.config.ts               # Vite 构建配置
├── 📄 tailwind.config.js           # Tailwind CSS 配置
├── 📄 postcss.config.js            # PostCSS 配置
├── 📄 .eslintrc.cjs                # ESLint 配置
├── 📄 .prettierrc.json             # Prettier 配置
├── 📄 .prettierignore              # Prettier 忽略文件
├── 📄 .editorconfig                # 编辑器配置
├── 📄 .gitignore                   # Git 忽略文件
└── 📄 index.html                   # HTML 入口
```

## 📁 文档目录 (docs/)

```
docs/
├── 📄 ARCHITECTURE.md              # 架构设计文档（详细）
└── 📄 DEVELOPMENT.md               # 开发指南（详细）
```

## 📁 前端源码 (src/)

```
src/
├── 📂 components/                  # React 组件
│   └── 📂 layouts/
│       ├── MainLayout.tsx          # 主布局
│       └── Sidebar.tsx             # 侧边栏导航
├── 📂 pages/                       # 页面组件
│   ├── 📂 QuickConvert/
│   │   └── index.tsx               # 快速转换页
│   ├── 📂 RecipeBuilder/
│   │   └── index.tsx               # 食谱构建器页
│   ├── 📂 Queue/
│   │   └── index.tsx               # 任务队列页
│   ├── 📂 Preview/
│   │   └── index.tsx               # 预览页
│   ├── 📂 Plugins/
│   │   └── index.tsx               # 插件管理页
│   └── 📂 Settings/
│       └── index.tsx               # 设置页
├── 📂 lib/                         # 工具库
│   └── tauri.ts                    # Tauri IPC 封装
├── 📂 types/                       # 类型定义
│   └── ipc.ts                      # IPC 契约类型
├── 📂 router/                      # 路由配置
│   └── index.tsx                   # 路由定义
├── 📂 styles/                      # 样式文件
│   └── index.css                   # 全局样式 + Tailwind
├── 📂 hooks/                       # 自定义 Hooks（待添加）
├── 📂 store/                       # 状态管理（待添加）
├── App.tsx                         # 根组件
└── main.tsx                        # 前端入口
```

## 📁 Rust 后端 (src-tauri/)

```
src-tauri/
├── 📂 src/
│   ├── 📂 core/                    # 核心模块
│   │   ├── mod.rs                  # 模块导出
│   │   ├── registry.rs             # 能力注册表
│   │   ├── planner.rs              # 转换规划器
│   │   ├── pipeline.rs             # 流水线执行器
│   │   ├── adapter.rs              # 工具适配器
│   │   ├── storage.rs              # 存储管理
│   │   └── quality.rs              # 质量评估
│   ├── 📂 types/                   # 类型定义
│   │   ├── mod.rs                  # 模块导出
│   │   ├── capability.rs           # 能力相关类型
│   │   ├── job.rs                  # 任务相关类型
│   │   └── conversion.rs           # 转换相关类型
│   ├── 📂 commands/                # Tauri 命令
│   │   └── mod.rs                  # IPC 命令实现
│   ├── error.rs                    # 错误处理
│   └── main.rs                     # Rust 主入口
├── 📂 icons/                       # 应用图标（需要添加）
│   └── README.md                   # 图标说明
├── 📄 Cargo.toml                   # Rust 依赖配置
├── 📄 Cargo.lock                   # Rust 锁文件
├── 📄 build.rs                     # 构建脚本
├── 📄 tauri.conf.json              # Tauri 配置
└── 📄 rustfmt.toml                 # Rust 格式化配置
```

## 📊 文件统计

### 按类型

| 类型 | 数量 | 说明 |
|------|------|------|
| TypeScript/TSX | ~20 | 前端代码 |
| Rust | ~15 | 后端代码 |
| 配置文件 | ~12 | 各种配置 |
| 文档 | 5 | 项目文档 |
| **总计** | **~52** | |

### 按模块

| 模块 | 文件数 | 代码行数（估算） |
|------|--------|------------------|
| Rust 核心 | 10 | ~1500 |
| 前端 UI | 15 | ~800 |
| 类型定义 | 5 | ~500 |
| 配置 | 12 | ~300 |
| 文档 | 5 | ~1200 |
| **总计** | **47** | **~4300** |

## 📦 关键依赖

### 前端

```json
{
  "核心框架": {
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-router-dom": "^6.27.0"
  },
  "构建工具": {
    "vite": "^5.4.10",
    "@vitejs/plugin-react-swc": "^3.7.1"
  },
  "UI": {
    "tailwindcss": "^3.4.14",
    "lucide-react": "^0.454.0"
  },
  "状态管理": {
    "zustand": "^5.0.0",
    "jotai": "^2.10.0",
    "@tanstack/react-query": "^5.61.0"
  },
  "Tauri": {
    "@tauri-apps/api": "^2.2.0",
    "@tauri-apps/cli": "^2.2.0",
    "@tauri-apps/plugin-shell": "^2.0.0"
  }
}
```

### Rust

```toml
[dependencies]
tauri = "2.0"
tauri-plugin-shell = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.42", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
dashmap = "6.1"
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2_sqlite = "0.24"
# ... 更多依赖见 Cargo.toml
```

## 🎯 待实现功能

### 高优先级（M1）

- [ ] 应用图标文件
- [ ] 文件拖拽上传组件
- [ ] 第一个转换器实现
- [ ] IPC 调用测试

### 中优先级（M2-M3）

- [ ] 食谱构建器 UI
- [ ] DAG 可视化
- [ ] 插件系统实现
- [ ] 设置持久化

### 低优先级（M4+）

- [ ] OCR 集成
- [ ] LLM 集成
- [ ] 性能优化
- [ ] 单元测试

## 🔧 开发工具配置

### VS Code 推荐扩展

创建 `.vscode/extensions.json`:

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tauri-apps.tauri-vscode",
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "bradlc.vscode-tailwindcss"
  ]
}
```

### VS Code 设置

创建 `.vscode/settings.json`:

```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

## 📝 待创建文件

### 优先级 P0

```
src-tauri/icons/
├── icon.ico          # Windows 图标
├── icon.icns         # macOS 图标
├── 32x32.png
├── 128x128.png
├── 128x128@2x.png
└── icon.png
```

### 优先级 P1

```
src/
├── components/ui/    # shadcn/ui 组件
├── hooks/            # 自定义 Hooks
└── store/            # 状态管理
```

### 优先级 P2

```
tests/                # 测试文件
├── e2e/              # 端到端测试
└── unit/             # 单元测试
```

## 🎨 代码风格

### TypeScript

- **缩进**: 2 空格
- **引号**: 双引号
- **分号**: 使用
- **行宽**: 100 字符
- **命名**: camelCase（函数/变量），PascalCase（组件/类型）

### Rust

- **缩进**: 4 空格
- **行宽**: 100 字符
- **命名**: snake_case（函数/变量），PascalCase（类型/trait）
- **格式化**: `cargo fmt`

## 🔍 快速查找

### "我想..."

- **添加新页面** → `src/pages/` + `src/router/index.tsx`
- **修改 IPC 命令** → `src-tauri/src/commands/mod.rs`
- **添加类型定义** → `src/types/ipc.ts` + `src-tauri/src/types/`
- **修改样式** → `src/styles/index.css` + Tailwind 类名
- **查看架构** → `docs/ARCHITECTURE.md`
- **开发指南** → `docs/DEVELOPMENT.md`
- **快速开始** → `QUICKSTART.md`

## 📚 学习资源

### 官方文档

- **Tauri**: https://tauri.app/
- **React**: https://react.dev/
- **Rust Book**: https://doc.rust-lang.org/book/
- **TypeScript**: https://www.typescriptlang.org/docs/
- **Tailwind CSS**: https://tailwindcss.com/docs

### 项目文档

1. `README.md` - 项目概览
2. `QUICKSTART.md` - 快速开始
3. `PROJECT_SUMMARY.md` - 详细总结
4. `docs/ARCHITECTURE.md` - 架构设计
5. `docs/DEVELOPMENT.md` - 开发指南

---

**最后更新**: 2025-11-12

**文件数量**: ~52 个文件（不包括 node_modules 和 target）

**代码行数**: ~4300 行（不包括依赖）
