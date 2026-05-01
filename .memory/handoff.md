# 交接说明

## 当前进度

已完成项目的架构转向，正式废弃了 Tauri/Vue/Element Plus，切换至 **GPUI + Rust 原生方案**。历史前端代码已被完全删除，架构文档与全局提示词已全部更新。

目前处于 **Phase 0：初始化 GPUI 骨架阶段**，需要从零开始搭建 GPUI 渲染。

## 快速上手

项目现为纯 Rust Workspace：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

运行 GPUI 主程序（待创建）：
```bash
cargo run -p narya-app
```

## 关键文件

- `architecture/narya-gpui-architecture-design.md`：核心架构设计，必须通读以理解 GPUI 单语言栈模型。
- `prompt.md`：全局提示词，记录了 GPUI 的严格高保真 UI 要求。
- `ui/` 和 `ui/specs/`：这是 UI 开发的唯一依据，禁止在 GPUI 中随便写标准样式，所有尺寸需依据 spec.md。

## GPUI 开发须知

- **无现成组件**：你无法使用 `el-button` 等现成前端标签。必须用 GPUI 的 `div()`, `text()`, `bg()`, `rounded()` 等 API 构建每一个像素。
- **玻璃态挑战**：玻璃卡片需要探索 GPUI 的图形绘制层或多层背景混合。
- **并发要求**：后端调用必须异步在 `cx.spawn` 或 `Tokio` 运行时中处理，避免阻塞主线程。

## 下阶段目标

接手后，立刻阅读 `.prompt/001-phase-0-gpui-bootstrap.md` 并执行。
