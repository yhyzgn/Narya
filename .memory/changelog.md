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

## 2026-05-02 — Business Pages & Routing (GPUI)

### 已完成
- **实现视图路由**：在 `AppShell` 中引入了 `ActiveView` Enum，实现了侧边栏点击切换视图的机制。
- **还原 Nodes 页面**：完成了节点列表页面的高保真开发，包含搜索栏、协议/延迟 Badge 和节点卡片。
- **还原 Subscriptions 页面**：完成了订阅管理页面的开发，包含流量使用进度条、过期提醒和操作按钮。
- **封装通用组件**：新增了 `Badge` 和 `SearchInput` 两个原子组件。
- **解决状态更新难题**：通过在 `AppShell` 创建时捕获 `WeakEntity` 句柄，解决了 GPUI 0.2.x 闭包内更新 View 状态的难题。
- **全量验证**：修复了所有 Clippy 警告，确保代码符合 Rust 惯用法。

### 修改文件
- `src/main.rs` (核心逻辑更新)
- `src/components.rs` (组件库扩充)
- `.memory/status.md`
- `.memory/tasks.md`
- `.memory/changelog.md`

### 下一步
- Phase 4: 将 UI 视图与 `narya-core` 核心逻辑对接，实现真实数据驱动。
