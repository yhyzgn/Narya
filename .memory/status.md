# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 6: 核心内核对接与系统集成 (GPUI)。
项目已成功实现**高级交互逻辑**，建立了基于 `AppState` 的实时动态更新机制（含异步测速模拟、实时流量监控）。目前 UI 已达到像素级高保真还原，且核心业务流程（节点切换、详情展示、测速触发）已闭环。接下来需要开始将这些 UI 逻辑与真正的后端 `narya-daemon` 及 `sing-box` 内核对接。

## 当前仓库内容

```text
Cargo.toml         # 工作区主配置
src/
  main.rs          # 最小化入口
crates/
  narya-app/       # GPUI 应用主 crate (状态管理、实时交互、高保真视图)
  narya-core/      # 核心领域模型与业务逻辑
  narya-daemon/    # Daemon 核心集成 (待开发)
  ... (其他功能 crate)
resources/         # 静态资源 (Logo, 背景图)
ui/                # UI 视觉真源
.memory/
.prompt/
```

## 已完成

- **架构工程化**：完全转为 Workspace 模式，解耦 UI 与核心逻辑。
- **高保真 UI 还原**：基于最新的组件树 JSON 与 PNG 原图，还原了 AppShell、Splash、Nodes (含详情面板) 和 Subscriptions 页面。
- **实时状态同步**：实现了基于 `AppState` 模型的核心状态观察机制。
- **动态交互功能**：
    - **一键测速**：实现了异步测速模拟逻辑，支持 UI 触发与数据自动回显。
    - **流量监测**：实现了后台定时任务，侧边栏 Footer 可实时显示动态波动的网速。
    - **详情面板**：在 Nodes 页面实现了随选中项切换的固定详情卡片，字段对齐设计稿。
- **验证通过**：全工作区 `cargo clippy` 与 `cargo fmt` 均通过。

## 尚未完成

- 集成 `narya-daemon` 实现真实的系统代理开关逻辑。
- 对接 `sing-box` 接口获取真实的节点状态与测速结果。
- 实现配置文件 (Profiles) 的 CRUD 编辑器。

## 阻塞/风险

- 无。

## 下一个建议任务

执行 `.prompt/007-phase-6-kernel-integration.md`：开始对接 Daemon RPC 接口实现真实内核控制。
