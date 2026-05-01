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

## 2026-05-02 — Splash & Dashboard Implementation (GPUI)

### 已完成
- **还原 Splash 页面**：对照 `ui/splash.png`，实现了居中的品牌 Logo、加载进度条和状态文本。
- **进度条动画**：使用 GPUI 的异步执行器实现了平滑的进度条加载模拟（3秒周期）。
- **Dashboard 核心布局**：在 `AppShell` 中填充了侧边栏（导航项、连接状态卡片）、顶部操作栏以及主内容区的 Proxy Cards 和 Quick Connect 列表。
- **自研基础控件**：封装了像素级对齐的 `Switch`（52x29px）和 `NodeItem` 组件。
- **窗口切换逻辑**：实现了从 Splash 窗口自动销毁并打开 Dashboard 主窗口的完整流程。
- **代码验证**：修复了多项 GPUI 异步上下文相关的编译错误，确保 `cargo clippy` 与 `cargo fmt` 全绿。

### 修改文件
- `src/main.rs` (核心逻辑更新)
- `src/theme.rs` (Tokens 细化)
- `src/components.rs` (控件库扩充)
- `.memory/status.md`
- `.memory/tasks.md`
- `.memory/changelog.md`

### 下一步
- Phase 3: 迁移 Nodes 与 Subscriptions 业务页面，建立视图切换路由。
