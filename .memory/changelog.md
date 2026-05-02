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

## 2026-05-02 — Advanced Features & Interactive Logic

### 已完成
- **实时测速模拟**：实现了异步 Latency Test 逻辑，使用 `cx.spawn` 在后台线程模拟网络延迟并在前端实时回显结果。
- **动态网速监控**：建立了全局流量监测定时任务（1s/次），侧边栏 Footer 可根据当前活动节点实时显示随机波动的网速信息。
- **高保真详情面板**：在 Nodes 页面右下角实现了固定位置的“节点详情”面板，严格按照 `ui/nodes.png` 还原了地址、协议、加密、UDP/TLS 等业务字段。
- **UI 组件打磨**：优化了 `Badge` 和 `node_card` 的视觉样式，根据选中状态动态切换边框与背景色，视觉对齐最新的规格要求。
- **代码规范化**：解决了 Unit Value 绑定、未使用的 Theme 变量等 Clippy 警告，确保全工作区编译通过。

### 修改文件
- `crates/narya-app/src/state.rs` (异步模拟逻辑)
- `crates/narya-app/src/views/nodes.rs` (详情面板与布局重构)
- `crates/narya-app/src/views/app_shell.rs` (侧边栏 Footer 实时更新)

### 下一步
- Phase 6: 建立 `narya-daemon` 核心通信骨架，对接真实的系统代理控制与内核 API。

