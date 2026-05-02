# 当前状态

## 更新时间

2026-05-02

## 当前阶段

Phase 8: UI/后端逻辑闭环与全功能联调 (GPUI)。
项目已成功实现**内核编排与系统控制**，完成了 `narya-daemon` 的核心功能引擎：包含 `sing-box` 进程管理、跨平台系统代理设置及 YAML 配置持久化。目前后端已具备执行真实业务指令的能力。下一步将开始在 UI 中集成这些真实的后端调用，替换现有的 Mock 逻辑，实现真正的代理开关与实时内核监控。

## 当前仓库内容

```text
Cargo.toml         # 工作区主配置
src/
  main.rs          # 最小化入口
crates/
  narya-app/       # UI (含 IPC Client, 状态同步)
  narya-daemon/    # 后端服务 (内核编排、系统代理、UDS Server)
  narya-ipc/       # 共享协议
  narya-config/    # 配置持久化 (YAML)
  narya-core/      # 核心领域模型 (支持 Serde)
  ...
resources/         # 静态资源
ui/                # UI 视觉真源
.memory/
.prompt/
```

## 已完成

- **核心功能引擎**：
    - **KernelManager**：实现了基于 `tokio::process` 的 `sing-box` 进程启动与销毁逻辑。
    - **SystemProxy**：实现了 Linux 环境下的 `gsettings` 代理自动切换逻辑。
    - **narya-config**：实现了 Profile 配置的 YAML 序列化与持久化读写。
- **IPC 指令集扩展**：Daemon 现已支持 `SetSystemProxy`、`StartKernel`、`StopKernel` 等核心 RPC 指令。
- **全量模型序列化**：为 `narya-core` 中的所有领域模型增加了 `Serialize/Deserialize` 支持。
- **验证通过**：全工作区 `cargo check` 绿色。

## 尚未完成

- 在 UI `AppState` 中调用真实的 IPC 指令替换 Mock。
- 实现真正的配置文件合并生成逻辑（将订阅节点转为 sing-box json）。
- 完善日志实时推送 (GetKernelLogs)。

## 阻塞/风险

- 无。

## 下一个建议任务

执行 `.prompt/009-phase-8-real-integration.md`：实现 UI 与 Daemon 的全流程闭环联调。
