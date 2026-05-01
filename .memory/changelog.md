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

## 2026-05-02 — Design System & AppShell Implementation

### 已完成
- **建立 Theme 模块**：定义了全局色彩（Cold White, Indigo, Success Green）和字体层级。
- **封装 GlassCard**：实现了支持 16px 圆角和 24px 间距的毛玻璃基础容器。
- **搭建 AppShell**：构建了符合 1536x1024 标准的侧边栏、页头、页脚和内容区域结构。
- **验证通过**：全工作区编译通过，`AppShell` 布局验证正确。

### 修改文件
- `src/theme.rs`
- `src/components.rs`
- `src/main.rs`
- `.memory/status.md`
- `.memory/tasks.md`
