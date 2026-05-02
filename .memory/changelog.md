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

## 2026-05-02 — UI Fidelity & Windowing Fixes (GPUI)

### 已完成
- **无边框 Splash 窗口**：通过配置 `WindowOptions` 实现了 Splash 窗口的透明与无边框化，完美契合视觉设计。
- **窗口自动销毁**：实现了 Splash 加载完成后自动销毁旧窗口并开启 Dashboard 主窗口的闭环逻辑。
- **补全侧边栏菜单**：根据 `ui/dashboard.png` 补全了全部 10 个功能模块导航（Nodes, Connections, Rules, Subscriptions, Config, Logs, Tools, Settings, About）。
- **动态视图适配**：扩展了 `ActiveView` 路由匹配逻辑，为所有新菜单项增加了占位视图与正确的 Header 标题显示。
- **环境兼容性增强**：在 `Cargo.toml` 中开启了 Linux 核心特性，解决了特定环境下的启动崩溃。

### 修改文件
- `src/main.rs` (核心逻辑与布局修正)
- `Cargo.toml` (特性开启)
- `.memory/status.md`
- `.memory/changelog.md`

### 下一步
- Phase 4: 将 UI 视图与 `narya-core` 核心逻辑对接，实现真实数据驱动。

