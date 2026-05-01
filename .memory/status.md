# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 2: Splash 与 Dashboard 重建 (GPUI)。
项目已成功建立基础 Design System 和 AppShell 布局，现在需要开始还原具体的业务页面，首要是 Splash 启动页和 Dashboard 核心面板。

## 当前仓库内容

```text
Cargo.toml         # 主应用配置
src/
  main.rs          # GPUI 主应用入口
crates/
  narya-core/      # 核心类型 (待完善)
  narya-daemon/    # Daemon (待完善)
  narya-config/
  narya-kernel/
  narya-platform/
  narya-subscription/
  narya-rules/
  narya-traffic/
  narya-log/
resources/
architecture/narya-gpui-architecture-design.md  # 最新的 GPUI 架构文档
ui/                # UI 视觉真源与 specs
.memory/
.prompt/
```

## 已完成

- **架构大重构**：删除了 Tauri / Vue / Element Plus 的所有代码。
- **文档更新**：重写了 `prompt.md` 和 `architecture/narya-gpui-architecture-design.md`。
- **GPUI Bootstrap**：成功在项目根目录初始化 GPUI 环境并实现基础窗口。
- **Design System Phase 1**：建立了基于 `Theme` 模块的 Design Tokens，实现了 `GlassCard` 基础组件，并搭建了 canonical 1536x1024 的 `AppShell` 布局结构。

## 尚未完成

- 实现 GPUI 的基础 Design System (Tokens, AppShell)。
- 逐步在 GPUI 中重建 Splash 和 Dashboard 等页面。

## 阻塞/风险

- GPUI 生态处于早期，需要高度手工实现基础组件（Button, Switch, Card 等）。所有组件需对照 `ui/specs` 精确开发。

## 下一个建议任务

执行 `.prompt/003-phase-2-splash-dashboard.md`：重建 Splash 和 Dashboard 页面。
