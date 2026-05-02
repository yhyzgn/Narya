# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 5: 高级功能与后端交互 (GPUI)。
项目已成功实现**核心逻辑集成**，建立了基于 `AppState` 的集中式状态管理。目前 UI 视图（Nodes, Subscriptions, Sidebar）已完全由真实（或高保真 Mock）领域模型驱动，并实现了状态的实时同步。接下来需要开始实现更复杂的业务功能，如实时的节点测速、规则引擎交互等。

## 当前仓库内容

```text
Cargo.toml         # 工作区主配置
src/
  main.rs          # 最小化入口，调用 narya_app::run()
crates/
  narya-app/       # GPUI 应用主 crate (视图、组件、主题、状态管理、路由)
  narya-core/      # 核心领域模型与业务逻辑 (Node, Subscription 定义)
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
- **工程化重构**：实现了 Workspace 模式，建立了独立的 `narya-app` UI crate。
- **领域模型对接**：在 `narya-core` 中定义了 `Node` 和 `Subscription` 模型，包含 UI 设计中要求的所有字段。
- **集中状态管理**：实现了 `AppState` (GPUI Entity)，在 `AppShell` 中观察状态变化，实现了“点击节点，侧边栏状态同步更新”的动态交互。
- **数据驱动视图**：重构了 `Nodes` 和 `Subscriptions` 页面，通过遍历 `AppState` 数据进行动态渲染。
- **高保真还原**：Splash 启动页及 Dashboard 核心布局已完全对齐 UI 设计稿（含 Logo、主题驱动背景、加载项等）。
- **验证通过**：全工作区编译 (`cargo clippy`) 与格式化 (`cargo fmt`) 均通过。

## 尚未完成

- 实现实时的节点延迟测试（Latency Test）逻辑。
- 实现订阅更新与解析的后端逻辑集成。
- 对接 `narya-daemon` 实现系统级代理控制切换。
- 完善高级交互组件 (Dropdown, Modal 等)。

## 阻塞/风险

- 由于当前 CLI 环境缺少 Display Server (X11/Wayland)，无法进行实时的图形界面自动化测试。

## 下一个建议任务

执行 `.prompt/006-phase-5-advanced-features.md`：实现节点实时延迟测试与数据反馈。
