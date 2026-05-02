# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 7: 真实内核集成与配置持久化 (GPUI)。
项目已成功实现**后端通信骨架 (IPC)**，建立了 `narya-daemon` (UDS Server) 与 `narya-app` (Client) 之间的双向通信链路。目前 UI 已能够尝试与后端建立连接并预留了真实数据推送接口。下一步将开始在 Daemon 中集成真实的 `sing-box` 运行逻辑及跨平台系统代理控制。

## 当前仓库内容

```text
Cargo.toml         # 工作区主配置
src/
  main.rs          # 最小化入口
crates/
  narya-app/       # UI (含 IPC Client, 状态同步)
  narya-daemon/    # 后端服务 (含 UDS Listener, 系统控制预留)
  narya-ipc/       # 共享协议 (Request/Response/Notification 定义)
  narya-core/      # 核心领域模型
  ...
resources/         # 静态资源
ui/                # UI 视觉真源
.memory/
.prompt/
```

## 已完成

- **IPC 通信体系**：
    - **narya-ipc**：定义了基于 JSON-RPC 2.0 的请求、响应及通知模型。
    - **narya-daemon**：实现了基于 Tokio 的 Unix Domain Socket 监听器，支持多客户端并发连接。
    - **narya-app**：在 `AppState` 中集成了 `IpcClient`，实现了启动自动重连及数据流占位。
- **高保真 UI 与逻辑**：完成了 Nodes 详情、实时网速模拟（预留 IPC 接入点）、一键测速等功能。
- **工程化架构**：全工作区模块化拆分完成，通过 `cargo clippy` 严格验证。

## 尚未完成

- 在 Daemon 中实现具体的 `sing-box` 进程管理。
- 编写跨平台系统代理切换脚本 (macOS `networksetup` / Linux `nmcli`)。
- 实现 Profiles (yaml) 的持久化读写与多方案切换。

## 阻塞/风险

- 无。

## 下一个建议任务

执行 `.prompt/008-phase-7-kernel-orchestration.md`：实现真实的 sing-box 内核生命周期管理与 IPC 状态实时回显。
