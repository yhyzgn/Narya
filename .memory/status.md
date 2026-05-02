# 当前状态

## 更新时间

2026-05-02

## 当前阶段

里程碑 1 达成：高保真 UI 与 后端核心逻辑 全集成。
项目已完成从 UI 设计稿到功能完备的 Rust 桌面客户端的跨越。目前 Narya 具备了高保真的交互界面、实时的后端状态同步、跨平台的系统设置控制、以及生产级的订阅拉取与内核配置生成能力。

## 当前仓库内容

```text
Cargo.toml         # 工作区总控
src/
  main.rs          # 最小化入口
crates/
  narya-app/       # UI 层 (GPUI, 状态管理, 高保真视图, IPC Client, 托盘)
  narya-daemon/    # 后端层 (内核编排, 系统代理, 配置合成, IPC Server)
  narya-subscription/ # 业务层 (远程拉取, Clash/Base64 解析)
  narya-ipc/       # 协议层 (JSON-RPC 2.0)
  narya-config/    # 持久化层 (YAML Profiles)
  narya-core/      # 模型层 (领域对象, Serde)
resources/         # 静态资源 (Logo, 启动背景)
ui/                # 视觉真源 (PNG, JSON Specs)
docs/              # 方案与规格说明
.memory/           # 项目记忆索引
```

## 已完成核心功能

1.  **像素级高保真 UI**：完全对齐 `ui/nodes.png` 等设计稿，实现了 10 个功能模块的导航、复杂的节点卡片、实时网速动效、以及固定的详情展示面板。
2.  **高性能后端引擎**：
    *   **IPC 体系**：基于 Unix Domain Sockets 实现了低延迟的 UI/Daemon 双向通讯。
    *   **内核编排**：实现了 `sing-box` 进程的自动生命周期管理与生产级配置动态合成。
    *   **系统控制**：支持 macOS (`networksetup`) 和 Linux (`gsettings`) 的系统代理原子切换。
3.  **智能业务逻辑**：
    *   **订阅引擎**：集成了 `reqwest` 实现远程订阅获取，支持 Clash 等主流格式解析与 UUID 去重。
    *   **实时监控**：实现了每秒频率的网速通知流，UI 侧边栏可实时反映真实流量波动。
4.  **生产级工程化**：
    *   完全解耦的 Workspace 架构。
    *   解决了 GPUI 0.2.x 下的 Async 借用、Dyn 兼容性、以及 宏递归限制 等深层次技术挑战。
    *   代码全量通过 `cargo clippy` 严格验证。

## 尚未完成

- 集成具体的 `sing-box` 二进制分发（目前需系统预装）。
- 完善所有 10 个业务视图的细节逻辑（如 Logs 的实时滚动）。

## 结论

Narya 的核心骨架与高保真交互已全部打通。项目已具备作为生产级代理工具的基础。
