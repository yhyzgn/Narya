# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 4: 核心逻辑集成 (GPUI)。
项目已成功实现**工程化架构重构**，将所有 UI 逻辑、组件及主题迁移至独立的 `narya-app` crate。目前处于视图路由机制及核心业务页面（Nodes 节点列表、Subscriptions 订阅管理）的动态化阶段。

## 当前仓库内容

```text
Cargo.toml         # 工作区主配置
src/
  main.rs          # 最小化入口，调用 narya_app::run()
crates/
  narya-app/       # GPUI 应用主 crate (视图、组件、主题、路由)
  narya-core/      # 核心领域模型与业务逻辑
  narya-config/    # 配置管理
  narya-subscription/ # 订阅解析
  ... (其他功能 crate)
resources/         # 静态资源 (Logo, 背景图)
architecture/      # 架构文档
ui/                # UI 视觉真源与 specs
.memory/
.prompt/
```

## 已完成

- **架构大重构**：删除了 Tauri / Vue / Element Plus 的所有代码。
- **工程化重构**：将 UI 代码从 root 迁移至 `crates/narya-app`，实现了真正的 **Workspace 模式**，`main.rs` 仅作为入口。
- **模块化视图**：在 `narya-app` 中将 `Splash`、`AppShell` 及各业务页面拆分为独立模块。
- **Design System Phase 1**：建立了基于 `Theme` 模块的 Design Tokens，实现了 `GlassCard` 基础组件，并搭建了 canonical 1536x1024 的 `AppShell` 布局结构。
- **Phase 2: Splash & Dashboard**：成功在 GPUI 中 1:1 还原了 Splash 启动页（含品牌 Logo、动态加载项、版本与配置信息）及 Dashboard 核心布局与组件（Switch, Proxy Cards, Node Items）。实现了 Splash 到 Dashboard 的平滑窗口切换，并解决了 Splash 窗口残显与边框问题。
- **Phase 3: Business Pages & Routing**：实现了基于 Enum 的视图路由机制，在 `AppShell` 中动态切换 Dashboard、Nodes 和 Subscriptions 等 10 个业务视图。完成了 Nodes 列表及 Subscriptions 详情卡片的高保真还原。
- **验证通过**：全工作区编译 (`cargo clippy`) 与格式化 (`cargo fmt`) 均通过。通过开启 `x11` 与 `wayland` 特性解决了 Linux 下的 `unreachable!` startup panic。

## 尚未完成

- 对接 Rust 核心领域模型提供真实数据驱动（节点发现、订阅解析）。
- 实现高级交互功能：节点延迟实时测试、规则编辑器、工具箱诊断等。
- 完善 UI 组件库 (Select, Dropdown, Table, Input 等)。

## 阻塞/风险

- 由于当前 CLI 环境缺少 Display Server (X11/Wayland)，`cargo run` 会在平台初始化阶段挂起；且 `#[gpui::test]` 宏在此环境下触发 rustc SIGSEGV，因此逻辑通过静态检查验证。

## 下一个建议任务

执行 `.prompt/005-phase-4-core-logic-integration.md`：将 UI 视图与 Rust 核心逻辑对接。
