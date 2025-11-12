# 快速开始指南

## 🚀 立即开始开发

本项目的基础架构已经搭建完成！按照以下步骤即可开始开发。

## ⚠️ 首要任务：解决图标问题

Rust 编译目前因缺少图标文件而受阻。请选择以下任一方法解决：

### 方法 1: 使用占位图标（快速）

创建一个简单的 1x1 像素图标作为占位符：

```bash
# 安装 ImageMagick（如果没有）
# Windows: choco install imagemagick
# macOS: brew install imagemagick

cd src-tauri/icons

# 创建占位图标
magick -size 512x512 xc:blue icon.png
magick icon.png -resize 32x32 32x32.png
magick icon.png -resize 128x128 128x128.png
magick icon.png -resize 128x128 128x128@2x.png

# Windows
magick icon.png icon.ico

# macOS
# 需要使用 iconutil 或在线工具
```

### 方法 2: 使用在线工具

1. 访问 https://www.icoconverter.com/
2. 上传一张 512x512 的 PNG 图片
3. 下载生成的图标文件
4. 将文件放入 `src-tauri/icons/` 目录

### 方法 3: 暂时禁用打包（开发模式）

修改 `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "active": false,
    "targets": "all"
  }
}
```

**注意**: 这样可以跳过图标检查，但无法生成可分发的安装包。

## ✅ 验证环境

图标问题解决后，验证编译：

```bash
# 前端类型检查
pnpm type-check

# Rust 编译检查
cd src-tauri
cargo check
cd ..
```

都应该成功！

## 🎯 开始开发

### 启动开发服务器

```bash
# 启动 Tauri 开发模式（前端 + 后端）
pnpm tauri:dev
```

首次启动会比较慢（Rust 编译），后续会快很多。

### 只启动前端

如果暂时不需要 Rust 后端：

```bash
pnpm dev
```

访问 http://localhost:1420

## 📝 开发建议

### 1. 从简单功能开始

推荐按以下顺序开发：

1. **完善前端 UI**
   - 美化现有页面
   - 添加拖拽上传组件
   - 实现文件列表展示

2. **实现第一个转换器**
   - Pandoc DOCX → Markdown
   - 最简单的转换示例

3. **连通前后端**
   - 测试 IPC 调用
   - 实现文件探测
   - 显示转换结果

### 2. 代码风格

运行格式化工具：

```bash
# 前端
pnpm format

# Rust
cd src-tauri && cargo fmt
```

### 3. 查看文档

- 架构设计: `docs/ARCHITECTURE.md`
- 开发指南: `docs/DEVELOPMENT.md`
- 项目总结: `PROJECT_SUMMARY.md`

## 🔧 常见问题

### Q: Rust 编译很慢？

A: 首次编译需要 2-5 分钟，这是正常的。后续增量编译只需 10-30 秒。

### Q: 如何添加新的 IPC 命令？

A: 查看 `docs/DEVELOPMENT.md` 中的"添加新的 IPC 命令"章节。

### Q: 前端如何调用 Rust 函数？

A: 使用 `src/lib/tauri.ts` 中封装好的函数：

```typescript
import { detectCapabilities } from "@/lib/tauri";

const capabilities = await detectCapabilities();
console.log(capabilities);
```

### Q: 如何调试？

**前端**:
- 按 F12 打开 Chrome DevTools
- 在 Tauri 窗口中调试

**Rust**:
```bash
RUST_LOG=debug pnpm tauri:dev
```

## 📦 推荐的开发工具

### VS Code 扩展

- Rust Analyzer
- Tauri
- ESLint
- Prettier
- Tailwind CSS IntelliSense
- Error Lens

### 浏览器

- Chrome 或 Edge（DevTools 支持最好）

## 🎨 UI 开发

项目已配置 Tailwind CSS，可以使用：

```tsx
<div className="flex items-center gap-4 p-8">
  <button className="bg-primary text-primary-foreground px-4 py-2 rounded-lg">
    开始转换
  </button>
</div>
```

预留了 shadcn/ui 集成位置，可按需添加组件。

## 🔄 开发工作流

```bash
# 1. 修改代码（前端或 Rust）
# 2. 保存文件
# 3. Vite/Cargo 自动重新编译
# 4. 浏览器/窗口自动刷新
# 5. 查看结果
```

## 🚢 下一步

完成基础设置后，建议按照 `PROJECT_SUMMARY.md` 中的"下一步工作 (M1 里程碑)"进行开发。

优先级：
1. ✅ 解决图标问题
2. ✅ 验证编译通过
3. 📝 实现文件拖拽上传
4. 📝 实现第一个转换器
5. 📝 连通前后端 IPC

## 💡 小贴士

- **频繁提交**: Git 提交越频繁越好，方便回滚
- **先测试后编码**: 先确保现有功能正常
- **参考文档**: 遇到问题先查看 docs/
- **增量开发**: 每次只实现一小块功能

## 🆘 需要帮助？

- 查看 `docs/DEVELOPMENT.md` 常见问题章节
- 阅读 Tauri 官方文档: https://tauri.app/
- React 文档: https://react.dev/

---

**祝开发顺利！** 🎉

如有问题，请参考项目文档或查阅相关技术栈的官方文档。
