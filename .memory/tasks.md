# 阶段任务计划

## Phase 0：GPUI 项目骨架
状态：已完成 ✅

- [x] 删除旧版 Tauri/Vue 前端工程与配置
- [x] 更新架构文档与总提示词，确立 GPUI 路线
- [x] 在根目录初始化 GPUI 环境并实现基础窗口验证
- [ ] 设置全局字体与初始基础 Theme/Tokens (移至 Phase 1)

## Phase 1：基础 UI 组件系统 (GPUI)
状态：已完成 ✅

- [x] 实现 GlassCard 容器
- [x] 实现基础 Typography (Theme 驱动)
- [x] 实现 AppShell 布局骨架
- [ ] 实现自研 Switch、Button 控件 (移至 Phase 2)

## Phase 2：Splash 与 Dashboard 重建 (GPUI)
状态：已完成 ✅

- [x] 在 GPUI 中还原 Splash 页面（像素级坐标对照 PNG/specs）
- [x] 实现 Splash 动画逻辑与窗口切换
- [x] 在 GPUI 中还原 Dashboard 页面（Sidebar, TopBar, 控制面板）

## Phase 3：高级与业务页面迁移
状态：已完成 ✅

- [x] 实现 Nodes 节点列表页面（支持表格/网格切换，延迟排序）
- [x] 实现 Subscriptions 订阅管理（添加、更新、流量统计）
- [x] 实现基础业务组件库 (Dropdown, Input, Table skeleton) -> Badge & SearchInput
- [x] 建立视图路由机制，在 Dashboard 侧边栏点击后切换主区域视图
- [x] **工程化重构**：将 UI 逻辑迁移至 `crates/narya-app`，实现 Workspace 模式

## Phase 4：核心逻辑集成
状态：已完成 ✅

- [x] 对接 `narya-core` 领域模型，实现节点列表的真实数据驱动
- [x] 实现订阅更新逻辑的视图模型对接
- [x] 实现侧边栏连接状态的实时更新
- [x] 重构为 Workspace 模式，解耦 `narya-app` 与 `narya-core`

## Phase 5：高级业务功能与交互
状态：已完成 ✅

- [x] 实现 Nodes 页面的“一键测速”逻辑，并异步回显延迟数据
- [x] 实现侧边栏 Footer 的实时流量网速显示 (Mock 实时更新)
- [x] 开发 Nodes 页面右下角的固定“节点详情”展示面板，严格对齐 UI
- [x] 实现节点选中后的全局状态同步

## Phase 6：后端 IPC 通信集成
状态：已完成 ✅

- [x] 在 `narya-ipc` 中定义 Request/Response/Notification 协议
- [x] 实现 `narya-daemon` 基础 UDS 监听服务
- [x] 在 `narya-app` 中集成 IPC Client 并对接 `AppState`
- [x] 解决 `narya-core` 与 `gpui` 的命名空间冲突问题

## Phase 7：内核编排与系统控制
状态：进行中 🏗️

- [ ] 实现 `narya-daemon` 对 `sing-box` 二进制文件的进程生命周期管理
- [ ] 编写跨平台（Linux/macOS）系统代理切换原子逻辑
- [ ] 建立基于 `narya-config` 的 YAML 配置持久化层
- [ ] 实现从 Daemon 到 App 的实时流量数据流推送
