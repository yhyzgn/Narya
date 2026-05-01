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
状态：进行中 🏗️

- [ ] 在 GPUI 中还原 Splash 页面（像素级坐标对照 PNG/specs）
- [ ] 实现 Splash 动画逻辑与窗口切换
- [ ] 在 GPUI 中还原 Dashboard 页面（Sidebar, TopBar, 控制面板）

## Phase 3：高级与业务页面迁移
状态：待开始

- [ ] 节点、配置、连接、规则等页面的 GPUI 实现
- [ ] 连接 Rust 核心提供数据驱动
