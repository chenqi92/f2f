# 架构设计文档

## 概述

F2F Converter 采用分层架构设计，核心使用 Rust 实现高性能转换引擎，通过 Tauri 提供跨平台桌面应用能力，前端使用 React 构建现代化 UI。

## 系统架构

### 整体架构图

```
┌─────────────────────────────────────────────────────────┐
│                      用户界面层                          │
│  React + TypeScript + Tailwind + shadcn/ui             │
│  ┌──────────┬──────────┬──────────┬──────────────┐    │
│  │ 快速转换 │ 食谱编辑 │   队列   │ 插件/设置    │    │
│  └──────────┴──────────┴──────────┴──────────────┘    │
└─────────────────────────────────────────────────────────┘
                         ↕ IPC (Tauri Commands/Events)
┌─────────────────────────────────────────────────────────┐
│                    Tauri 中间层                          │
│  • 窗口管理  • 文件系统  • IPC 桥接  • 事件分发        │
└─────────────────────────────────────────────────────────┘
                         ↕
┌─────────────────────────────────────────────────────────┐
│                    Rust 核心引擎                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 能力注册表 (Registry)                            │   │
│  │ • 转换器能力声明                                 │   │
│  │ • 转换图构建                                     │   │
│  │ • 工具健康检查                                   │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 转换规划器 (Planner)                             │   │
│  │ • 路径搜索 (BFS/Dijkstra)                       │   │
│  │ • 质量评分                                       │   │
│  │ • 依赖检查                                       │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 流水线执行器 (Pipeline)                          │   │
│  │ • DAG 调度                                       │   │
│  │ • 并发控制                                       │   │
│  │ • 重试/回退                                      │   │
│  │ • 进度跟踪                                       │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 工具适配器 (Adapter)                             │   │
│  │ • 沙箱执行                                       │   │
│  │ • 资源限制                                       │   │
│  │ • 错误处理                                       │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 存储管理 (Storage)                               │   │
│  │ • 工作区管理                                     │   │
│  │ • 内容寻址缓存                                   │   │
│  │ • SQLite 元数据                                  │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 质量评估 (QA)                                    │   │
│  │ • OCR 准确率                                     │   │
│  │ • 结构一致性                                     │   │
│  │ • 表格召回                                       │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                         ↕
┌─────────────────────────────────────────────────────────┐
│                   外部工具层                             │
│  Pandoc │ LibreOffice │ FFmpeg │ ImageMagick │ ...     │
└─────────────────────────────────────────────────────────┘
```

## 核心模块详解

### 1. 能力注册表 (Registry)

**职责**:
- 管理所有转换器的能力声明
- 构建格式转换图
- 检查工具可用性

**数据结构**:
```rust
pub struct CapabilityRecord {
    pub id: String,
    pub inputs: Vec<FormatSpec>,
    pub outputs: Vec<FormatSpec>,
    pub quality: f32,          // 质量评分 0-1
    pub speed: f32,            // 速度评分 0-1
    pub cost: ResourceCost,    // 资源成本
    pub requires: Requirements, // 依赖要求
    pub preserve: PreserveFeatures, // 保留特性
    pub risks: Vec<String>,    // 潜在风险
    // ...
}
```

**转换图**:
- 节点: MIME 类型
- 边: 转换器 ID + 质量评分
- 算法: 多源最短路径 (Dijkstra 变体)

### 2. 转换规划器 (Planner)

**职责**:
- 在转换图上寻找最优路径
- 考虑质量、速度、成本权衡
- 处理依赖缺失情况

**规划流程**:
```
1. 探测源文件格式 (libmagic + 启发式)
2. 获取可达目标格式集合
3. 对每个目标:
   a. 搜索所有可行路径 (BFS/Dijkstra)
   b. 过滤质量低于阈值的路径
   c. 检查依赖是否满足
   d. 计算综合评分
4. 排序并返回 Top-N 候选
```

**评分公式**:
```
score = w_quality * quality + w_speed * speed - w_cost * cost
```

### 3. 流水线执行器 (Pipeline)

**职责**:
- 管理任务队列
- 执行 DAG 转换步骤
- 处理并发和重试

**任务生命周期**:
```
Queued → Running → [Succeeded | Failed | Canceled]
           ↓
      progress events
```

**并发策略**:
- CPU 密集型任务: `num_cpus` 个并发
- IO 密集型任务: 可配置并发数
- GPU 任务: 串行或根据 GPU 数量

**错误处理**:
```rust
match result {
    Ok(_) => transition_to_succeeded(),
    Err(e) if is_retriable(e) => retry_with_backoff(),
    Err(e) => try_fallback_path().or_else(fail),
}
```

### 4. 工具适配器 (Adapter)

**职责**:
- 以沙箱方式调用外部工具
- 资源限制 (CPU/内存/时间)
- 错误码解析

**沙箱机制**:
```rust
pub fn execute_sandboxed(
    tool: &str,
    args: &[&str],
    input: &Path,
    output: &Path,
    limits: ResourceLimits,
) -> Result<Output> {
    Command::new(tool)
        .args(args)
        .timeout(limits.timeout)
        .memory_limit(limits.max_mem)
        .cpu_limit(limits.max_cpu)
        .env_clear()  // 清理环境变量
        .current_dir(temp_dir)  // 限制工作目录
        .output()
}
```

### 5. 存储管理 (Storage)

**目录结构**:
```
~/.local/share/f2f-converter/
├── workspace/          # 临时工作区
│   └── jobs/
│       └── {job_id}/
│           ├── input/
│           ├── intermediate/
│           └── output/
├── cache/              # 内容寻址缓存
│   └── {hash}/
│       └── result
└── data/
    └── jobs.db         # SQLite 任务元数据
```

**缓存策略**:
- Key: SHA-256(input_content + params)
- LRU 淘汰策略
- 可配置水位线

### 6. 质量评估 (QA)

**指标**:
| 维度 | 指标 | 阈值 |
|------|------|------|
| OCR | CER (字符准确率) | ≥ 95% |
| 文档 | 结构一致度 | ≥ 90% |
| 表格 | 召回率 | ≥ 90% |
| 媒体 | 码率偏差 | ≤ ±10% |

**评估流程**:
```
1. 转换前: 记录源文件特征
2. 转换后: 分析结果文件
3. 计算各项指标
4. 与阈值比较
5. 未达标 → 触发回退或建议
```

## 数据流

### 快速转换流程

```
1. 用户拖拽文件 → 前端
   ↓
2. plan_targets(file_path) → Rust
   ↓
3. Planner.plan(source, targets) → 返回候选路径
   ↓
4. 用户选择目标 + 参数 → 前端
   ↓
5. create_job(inputs, target, options) → Rust
   ↓
6. Pipeline.submit(job) → 入队
   ↓
7. Pipeline.execute(job) → 执行
   ├─ 步骤 1: Adapter.execute(step1)
   ├─ 步骤 2: Adapter.execute(step2)
   └─ ...
   ↓
8. 发送进度事件 → 前端更新 UI
   ↓
9. 完成 → 发送 artifact_ready 事件
   ↓
10. 用户预览/下载结果
```

### 食谱执行流程

```
1. 用户在画布上编排 DAG
   ↓
2. 保存为 Recipe (JSON)
   ↓
3. create_job(inputs, recipe_id=...) → Rust
   ↓
4. Pipeline 解析 Recipe 为执行计划
   ↓
5. 按拓扑序执行节点
   ├─ 预处理节点
   ├─ 转换节点 (可并行)
   ├─ 后处理节点
   └─ 验证节点
   ↓
6. 质量门槛检查
   ├─ 通过 → 继续
   └─ 未通过 → 回退到备选路径
   ↓
7. 输出最终产物
```

## IPC 契约

### 命令 (Command)

前端 → Rust

| 命令 | 参数 | 返回 |
|------|------|------|
| `detect_capabilities` | - | `CapabilityRecord[]` |
| `plan_targets` | `file_path` | `ConversionTarget[]` |
| `create_job` | `inputs, target, recipe_id?, options?` | `job_id` |
| `list_jobs` | `filters?, page?` | `Job[]` |
| `get_job` | `job_id` | `Job?` |
| `control_job` | `job_id, action` | `void` |
| `get_artifacts` | `job_id` | `Artifact[]` |
| `get_logs` | `job_id, cursor?` | `LogEntry[]` |
| `get_settings` | - | `Settings` |
| `set_settings` | `settings` | `void` |
| `run_health_check` | - | `HealthReport` |

### 事件 (Event)

Rust → 前端

| 事件 | 载荷 | 说明 |
|------|------|------|
| `job_progress` | `JobProgress` | 任务进度更新 |
| `job_state_changed` | `JobStateChanged` | 状态变更 |
| `artifact_ready` | `ArtifactReady` | 产物就绪 |
| `health_update` | `HealthReport` | 工具状态变更 |

## 安全考虑

### 沙箱隔离

- 外部工具以子进程运行
- 清理环境变量
- 限制文件系统访问 (白名单)
- 资源配额 (CPU/内存/时间)

### 输入验证

- 文件类型校验 (magic number)
- 路径遍历防护
- 命令注入防护
- 文件大小限制

### 隐私保护

- 默认本地离线处理
- 云服务需显式授权
- 日志脱敏 (路径/PII)
- 用户数据不上传

## 性能优化

### 缓存策略

- 内容寻址缓存 (去重)
- LRU 淘汰
- 增量转换 (部分更新)

### 并发优化

- CPU/IO/GPU 任务分类
- 异步任务队列
- 批处理合并

### 内存管理

- 流式处理大文件
- 及时释放临时文件
- 内存映射大对象

## 可扩展性

### 插件系统

```rust
pub trait ConverterPlugin {
    fn id(&self) -> String;
    fn capability(&self) -> CapabilityRecord;
    fn convert(&self, input: &Path, output: &Path, params: Value) -> Result<()>;
}
```

### 自定义节点

```typescript
interface CustomNode {
  type: string;
  execute: (input: File, params: any) => Promise<File>;
  validate: (params: any) => ValidationResult;
}
```

## 测试策略

### 单元测试

- Rust: `cargo test`
- 前端: Vitest

### 集成测试

- IPC 调用测试
- 端到端转换测试

### 金样本测试

- 标准测试集 (文档/图片/视频/数据)
- 质量指标回归测试

## 监控与诊断

### 日志

- 结构化日志 (tracing)
- 按级别过滤
- 导出诊断包

### 指标

- 任务成功率
- 平均转换时间
- 工具可用率
- 缓存命中率

## 下一步

- [ ] 实现完整的图搜索算法
- [ ] 添加 SQLite 持久化
- [ ] 实现实际的工具适配器
- [ ] 添加事件发送机制
- [ ] 完善错误处理和重试逻辑
