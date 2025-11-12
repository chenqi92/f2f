# 开发指南

## 环境搭建

### 必需工具

1. **Node.js** (v18+)
   ```bash
   node --version  # 应该 >= 18.0.0
   ```

2. **Rust** (v1.70+)
   ```bash
   rustc --version  # 应该 >= 1.70.0
   ```

3. **包管理器**: npm / pnpm / yarn

### 推荐工具

- **VS Code** + 插件:
  - Rust Analyzer
  - Tauri
  - ESLint
  - Prettier
  - Tailwind CSS IntelliSense

- **浏览器**: Chrome/Edge (用于 DevTools)

## 项目初始化

```bash
# 克隆仓库
git clone <repo-url>
cd f2f

# 安装前端依赖
pnpm install

# 检查 Tauri 依赖
pnpm tauri info
```

## 开发流程

### 启动开发服务器

```bash
# 启动前端 + Tauri (热重载)
pnpm tauri:dev
```

这会:
1. 启动 Vite 开发服务器 (端口 1420)
2. 编译 Rust 代码
3. 打开应用窗口
4. 监听文件变化并自动重载

### 仅前端开发

如果暂时不需要 Rust 后端:

```bash
# 只启动前端
pnpm dev
```

### 构建生产版本

```bash
# 构建前端资源
pnpm build

# 构建 Tauri 应用
pnpm tauri:build
```

产物位置:
- Windows: `src-tauri/target/release/f2f-converter.exe`
- macOS: `src-tauri/target/release/bundle/dmg/`
- Linux: `src-tauri/target/release/bundle/`

## 代码规范

### 前端

**Lint**:
```bash
pnpm lint
```

**格式化**:
```bash
pnpm format
```

**类型检查**:
```bash
pnpm type-check
```

### Rust

**格式化**:
```bash
cd src-tauri
cargo fmt
```

**Lint**:
```bash
cd src-tauri
cargo clippy
```

**测试**:
```bash
cd src-tauri
cargo test
```

## 项目结构详解

### 前端 (`src/`)

```
src/
├── components/        # React 组件
│   ├── layouts/       # 布局组件 (Sidebar, MainLayout)
│   └── ui/            # shadcn/ui 组件
├── pages/             # 页面组件
│   ├── QuickConvert/  # 快速转换页
│   ├── RecipeBuilder/ # 食谱构建器
│   ├── Queue/         # 任务队列
│   ├── Preview/       # 预览页
│   ├── Plugins/       # 插件管理
│   └── Settings/      # 设置页
├── lib/               # 工具库
│   └── tauri.ts       # Tauri IPC 封装
├── hooks/             # 自定义 Hooks
├── types/             # TypeScript 类型
│   └── ipc.ts         # IPC 契约定义
├── store/             # 状态管理
├── styles/            # 全局样式
│   └── index.css      # Tailwind + CSS 变量
├── router/            # 路由配置
│   └── index.tsx
├── App.tsx            # 根组件
└── main.tsx           # 入口文件
```

### 后端 (`src-tauri/src/`)

```
src-tauri/src/
├── core/              # 核心模块
│   ├── registry.rs    # 能力注册表
│   ├── planner.rs     # 转换规划器
│   ├── pipeline.rs    # 流水线执行器
│   ├── adapter.rs     # 工具适配器
│   ├── storage.rs     # 存储管理
│   ├── quality.rs     # 质量评估
│   └── mod.rs
├── types/             # 类型定义
│   ├── capability.rs  # 能力相关
│   ├── job.rs         # 任务相关
│   ├── conversion.rs  # 转换相关
│   └── mod.rs
├── commands/          # Tauri 命令 (IPC 接口)
│   └── mod.rs
├── error.rs           # 错误处理
└── main.rs            # 主入口
```

## 添加新功能

### 1. 添加新的转换器

**步骤**:

1. 在 `src-tauri/src/core/registry.rs` 中注册能力:

```rust
registry.register(CapabilityRecord {
    id: "my-converter".to_string(),
    name: "My Converter".to_string(),
    version: "1.0.0".to_string(),
    inputs: vec![FormatSpec {
        mime: "image/jpeg".to_string(),
        extensions: vec!["jpg".to_string()],
    }],
    outputs: vec![FormatSpec {
        mime: "image/png".to_string(),
        extensions: vec!["png".to_string()],
    }],
    quality: 0.95,
    // ... 其他字段
})?;
```

2. 在 `src-tauri/src/core/adapter.rs` 中实现转换逻辑:

```rust
pub fn convert_jpeg_to_png(input: &Path, output: &Path) -> Result<()> {
    Adapter::execute_sandboxed(
        "convert",  // ImageMagick
        &[input.to_str().unwrap(), output.to_str().unwrap()],
        input,
        output,
        Some(60),  // timeout
        Some(512), // max_mem_mb
    )?;
    Ok(())
}
```

### 2. 添加新的 IPC 命令

**步骤**:

1. 在 `src-tauri/src/commands/mod.rs` 中添加命令:

```rust
#[tauri::command]
pub async fn my_command(param: String) -> Result<String, ErrorResponse> {
    // 实现逻辑
    Ok("result".to_string())
}
```

2. 在 `src-tauri/src/main.rs` 中注册命令:

```rust
.invoke_handler(tauri::generate_handler![
    commands::detect_capabilities,
    // ... 其他命令
    commands::my_command,  // 新增
])
```

3. 在 `src/types/ipc.ts` 中添加类型定义:

```typescript
export interface MyCommandParams {
  param: string;
}

export interface MyCommandResult {
  result: string;
}
```

4. 在 `src/lib/tauri.ts` 中添加封装:

```typescript
export async function myCommand(param: string): Promise<MyCommandResult> {
  return invoke<MyCommandResult>("my_command", { param });
}
```

### 3. 添加新页面

**步骤**:

1. 创建页面组件:

```tsx
// src/pages/MyPage/index.tsx
export function MyPage() {
  return (
    <div className="p-8">
      <h2 className="text-2xl font-bold mb-4">My Page</h2>
      {/* 页面内容 */}
    </div>
  );
}
```

2. 在路由中注册:

```tsx
// src/router/index.tsx
import { MyPage } from "@/pages/MyPage";

<Route path="my-page" element={<MyPage />} />
```

3. 在侧边栏添加导航:

```tsx
// src/components/layouts/Sidebar.tsx
const navItems = [
  // ... 其他项
  { to: "/my-page", icon: Star, label: "My Page" },
];
```

## 调试技巧

### 前端调试

1. **浏览器 DevTools**:
   - 在 Tauri 窗口中按 `F12` 或 `Ctrl+Shift+I`
   - 查看 Console / Network / Sources

2. **React DevTools**:
   - 安装浏览器扩展
   - 在 DevTools 中查看组件树和状态

### Rust 调试

1. **日志输出**:
```rust
tracing::info!("Processing file: {:?}", path);
tracing::error!("Failed to convert: {}", err);
```

2. **环境变量**:
```bash
RUST_LOG=debug npm run tauri:dev
```

3. **GDB/LLDB**:
```bash
# 在 release 模式生成调试符号
cargo build --release --features debug-symbols
```

### IPC 调试

在前端查看 IPC 调用:

```typescript
import { invoke } from "@tauri-apps/api/core";

const result = await invoke("my_command", { param: "test" })
  .then(res => {
    console.log("Success:", res);
    return res;
  })
  .catch(err => {
    console.error("Error:", err);
    throw err;
  });
```

## 常见问题

### Q: 前端修改后未生效？

A: 检查:
1. Vite 是否正在运行？
2. 浏览器缓存是否清除？
3. 是否重启了 Tauri？

### Q: Rust 编译错误？

A: 尝试:
```bash
cd src-tauri
cargo clean
cargo build
```

### Q: IPC 调用失败？

A: 检查:
1. 命令是否在 `invoke_handler` 中注册？
2. 参数类型是否匹配？
3. 查看 Rust 日志输出错误信息

### Q: 依赖安装失败？

A: 尝试:
```bash
# 清理缓存
pnpm store prune
rm -rf node_modules pnpm-lock.yaml

# 重新安装
pnpm install
```

## 性能优化建议

### 前端

1. **使用 React.memo** 避免不必要的重渲染
2. **虚拟滚动** 处理大列表 (react-virtual)
3. **懒加载** 路由和组件
4. **代码分割** (Vite 自动处理)

### Rust

1. **避免不必要的克隆**:
```rust
// ❌ 不好
let data = expensive_data.clone();

// ✅ 更好
let data = &expensive_data;
```

2. **使用并发**:
```rust
use tokio::task;

let handles: Vec<_> = files
    .into_iter()
    .map(|file| task::spawn(process_file(file)))
    .collect();

for handle in handles {
    handle.await?;
}
```

3. **缓存计算结果**

## 提交规范

遵循 Conventional Commits:

```
feat: 添加 PDF 转 Markdown 功能
fix: 修复文件路径解析问题
docs: 更新开发指南
style: 格式化代码
refactor: 重构转换规划器
perf: 优化大文件处理性能
test: 添加单元测试
chore: 升级依赖版本
```

## 发布流程

1. 更新版本号:
   - `package.json`
   - `src-tauri/Cargo.toml`
   - `src-tauri/tauri.conf.json`

2. 更新 CHANGELOG

3. 构建并测试:
```bash
npm run build
npm run tauri:build
```

4. 创建 Git Tag:
```bash
git tag v0.1.0
git push origin v0.1.0
```

5. 发布 Release (GitHub Actions 自动处理)

## 资源

- [Tauri 文档](https://tauri.app/v1/guides/)
- [React 文档](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Tailwind CSS](https://tailwindcss.com/docs)
