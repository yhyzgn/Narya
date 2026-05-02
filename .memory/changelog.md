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

## 2026-05-02 — Subscription Parser & Multi-Platform Support

### 已完成
- **订阅解析模块 (narya-subscription)**：新建了订阅解析 crate，实现了对 Clash YAML 格式的深度解析，可将远程配置自动转换为标准的领域模型。
- **macOS 系统级适配**：在 Daemon 中集成了 macOS `networksetup` 后端，支持在苹果系统下真实控制系统代理开关。
- **高级过滤与搜索**：在 `AppState` 和 `Nodes` 视图中实现了毫秒级的搜索过滤引擎，支持按节点名称实时筛选。
- **代码健壮性重构**：通过 Enum Dispatch 解决了 async trait 的 dyn-compatibility 限制，并规范了所有异步闭包的 `WeakEntity` 捕获模式。
- **唯一标识体系**：引入了 `uuid` crate 为解析生成的节点提供全局唯一 ID，确保状态同步的准确性。

### 修改文件
- `crates/narya-subscription/*` (新 crate)
- `crates/narya-daemon/src/proxy.rs` (平台适配)
- `crates/narya-app/src/state.rs` (过滤逻辑)
- `Cargo.toml` (工作区依赖)

### 下一步
- Phase 10: 实现真实网络下载订阅、多平台托盘支持及 sing-box 生产级配置生成。





