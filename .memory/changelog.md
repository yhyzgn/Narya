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

## 2026-05-02 — UI/Backend Convergence & Real Integration

### 已完成
- **持久化 IPC 通讯**：在 `AppState` 中集成了基于 `AsyncApp` 的异步消息循环，支持实时监听后端推送的 `IpcNotification`。
- **真实业务闭环**：Dashboard 的一键连接功能已完全对接后端 `SetSystemProxy` 接口，实现了从 UI 触发真实系统代理变更。
- **状态动态回显**：UI 的连接状态、网速、内核状态现在均优先由后端 IPC 数据驱动，并具备自动重连机制。
- **全量联调通过**：完成了从“点击按钮 -> 后端执行 -> 状态推送回前端”的完整测试流程。

### 修改文件
- `crates/narya-app/src/state.rs` (核心集成)
- `crates/narya-app/src/ipc.rs` (通知循环)
- `crates/narya-app/src/views/dashboard.rs` (交互集成)

### 下一步
- Phase 9: 实现真实的订阅解析引擎、远程节点加载与 Profiles 持久化编辑器。




