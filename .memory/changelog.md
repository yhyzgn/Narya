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

## 2026-05-02 — Backend IPC Integration & Crate Decoupling

### 已完成
- **建立 IPC 通信层**：在 `narya-ipc` 中定义了标准 JSON-RPC 2.0 协议，包含异步 Request/Response 和 Notification 机制。
- **后台 Daemon 服务**：实现了 `narya-daemon` 独立服务，基于 `tokio` 监听 Unix Domain Socket (`/tmp/narya.sock`)。
- **UI 端集成**：在 `narya-app` 中开发了 `IpcClient` 模块，并将其无缝接入 `AppState` 的实时更新循环中。
- **架构冲突修复**：解决了 `narya_core` 与 `gpui` (如 `Subscription`) 的命名冲突，通过全限定路径引用确保了类型安全性。
- **递归限制优化**：在 `narya-app` crate root 增加了 `recursion_limit`，解决了 GPUI 复杂宏展开导致的栈溢出问题。

### 修改文件
- `crates/narya-ipc/*` (新 crate)
- `crates/narya-daemon/*` (新 crate)
- `crates/narya-app/src/ipc.rs` (新模块)
- `crates/narya-app/src/state.rs` (集成逻辑)

### 下一步
- Phase 7: 实现真实的 sing-box 内核编排、系统代理原子切换及配置持久化。


