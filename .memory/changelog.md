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

## 2026-05-02 — Kernel Orchestration & Config Persistence

### 已完成
- **内核生命周期管理**：在 `narya-daemon` 中实现了 `KernelManager`，支持通过 `tokio::process` 异步启动、监控并销毁 `sing-box` 内核进程。
- **系统代理自动切换**：实现了 `SystemProxy` 抽象层及 Linux `gsettings` 后端，能够真实修改操作系统的代理设置。
- **配置持久化体系**：在 `narya-config` 中实现了基于 YAML 的 Profile 读写功能，并为 `narya-core` 的领域模型补全了 `serde` 序列化链。
- **后端指令集扩展**：在 `main.rs` 中路由了 `StartKernel`、`StopKernel` 和 `SetSystemProxy` 等核心业务指令。

### 修改文件
- `crates/narya-daemon/src/kernel.rs` (新模块)
- `crates/narya-daemon/src/proxy.rs` (新模块)
- `crates/narya-config/src/lib.rs` (重构)
- `crates/narya-core/src/lib.rs` (派生 Trait)

### 下一步
- Phase 8: 实现 UI 与后端 Daemon 的全功能闭环，替换所有前端 Mock 数据为真实后端流。



