# 更新日志

## 2026-05-02 — Pivot to GPUI Native Architecture

### 已完成
- **架构大调与决策重构**：响应最终决策，项目正式废弃 Tauri + Vue + Vite + Element Plus 方案，由于前端生态在原生系统级管控和复杂网格渲染性能上面临不稳定和样式干预过多等问题，决定彻底转型为 **GPUI + Rust 原生桌面客户端**。
- **清除旧代码**：删除了原有的 `apps/desktop` 前端目录及其所有 Tauri 和 Vue 相关逻辑。
- **文档与记忆重写**：
  - 更新了 `architecture/narya-gpui-architecture-design.md`，确立了纯 Rust + GPUI 的状态模型和 UI 渲染架构。
  - 重写了 `prompt.md`，使其只针对 GPUI 和 Rust 开发。
  - 清理了所有的 `.prompt` 和 `.memory` 中有关 Web 前端的接力上下文。

### 修改文件
- `prompt.md`
- `architecture/narya-gpui-architecture-design.md`
- `.memory/status.md`
- `.memory/tasks.md`
- `.memory/decisions.md`
- `.memory/project.md`
- `.memory/handoff.md`
- `apps/desktop/` (Deleted)
- `.prompt/` (Cleared and recreated)

### 下一步
- Phase 2: 在 GPUI 中重建 Splash 与 Dashboard 页面。

## 2026-05-02 — Core Logic Integration & Centralized State

### 已完成
- **建立核心领域模型**：在 `narya-core` 中实现了 `Node` 和 `Subscription` 模型，严格对齐 UI 设计图中的所有业务字段。
- **集中状态管理 (AppState)**：引入了全局 `AppState` 实体，通过 GPUI `Model` 观察者模式，实现了跨视图的状态同步（如 Nodes 选中状态实时反映在 Sidebar Footer）。
- **动态视图重构**：将 `Nodes` 和 `Subscriptions` 页面从静态 Mock 重构为数据驱动模式，支持基于模型的列表渲染。
- **高保真 UI 最终修正**：实现了 `AssetSource` 以支持本地图片加载，还原了 Splash 品牌 Logo 及背景细节，并补全了侧边栏所有 10 个业务入口。
- **代码库规范化**：移除了冗余的 let 绑定与闭包，代码完全符合 Clippy 最新建议。

### 修改文件
- `crates/narya-core/src/lib.rs` (模型定义)
- `crates/narya-app/src/state.rs` (状态管理)
- `crates/narya-app/src/views/*` (动态化重构)
- `src/main.rs` (最小化入口)

### 下一步
- Phase 5: 实现节点实时测速与后端数据流的持续更新，开发通用对话框组件。

