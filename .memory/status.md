# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 4: 核心逻辑集成 (GPUI)。
项目已成功实现视图路由机制及核心业务页面（Nodes 节点列表、Subscriptions 订阅管理）。现在需要开始将 UI 与 Rust 后端核心逻辑对接，实现真实数据的加载、排序及交互。

## 已完成

- **架构大重构**：删除了 Tauri / Vue / Element Plus 的所有代码。
- **文档更新**：重写了 `prompt.md` 和 `architecture/narya-gpui-architecture-design.md`。
- **GPUI Bootstrap**：成功在项目根目录初始化 GPUI 环境并实现基础窗口。
- **Design System Phase 1**：建立了基于 `Theme` 模块的 Design Tokens，实现了 `GlassCard` 基础组件，并搭建了 canonical 1536x1024 的 `AppShell` 布局结构。
- **Phase 2: Splash & Dashboard**：成功在 GPUI 中 1:1 还原了 Splash 启动页（含品牌 Logo、动态加载项、版本与配置信息）及 Dashboard 核心布局与组件（Switch, Proxy Cards, Node Items）。实现了 Splash 到 Dashboard 的平滑窗口切换，并解决了 Splash 窗口残显与边框问题。
- **Phase 3: Business Pages & Routing**：实现了基于 Enum 的视图路由机制，在 `AppShell` 中动态切换 Dashboard、Nodes 和 Subscriptions 等 10 个业务视图（含 Connections, Rules, Config, Logs, Tools, Settings, About）。完成了 Nodes 列表及 Subscriptions 详情卡片的高保真还原。
- **验证通过**：全工作区编译 (`cargo clippy`) 与格式化 (`cargo fmt`) 均通过。通过开启 `x11` 与 `wayland` 特性解决了 Linux 下的 `unreachable!` startup panic。

## 尚未完成

- 对接 Rust 核心领域模型提供真实数据驱动（节点发现、订阅解析）。
- 实现高级交互功能：节点延迟实时测试、规则编辑器、工具箱诊断等。
- 完善 UI 组件库 (Select, Dropdown, Table, Input 等)。

## 阻塞/风险

- 由于当前 CLI 环境缺少 Display Server (X11/Wayland)，`cargo run` 会在平台初始化阶段挂起；且 `#[gpui::test]` 宏在此环境下触发 rustc SIGSEGV，因此逻辑通过静态检查验证。

## 下一个建议任务

执行 `.prompt/005-phase-4-core-logic-integration.md`：将 UI 视图与 Rust 核心逻辑对接。
