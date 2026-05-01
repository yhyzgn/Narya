# Narya 开发设计技术文档：GPUI + Rust 原生桌面客户端

**文档版本**：V2.0
**修订日期**：2026-05-02
**方案定位**：以 GPUI + Rust 作为桌面端 V1 主 UI 技术路线，Rust Daemon 继续负责系统权限、核心调度与网络控制。
**替代方案**：废弃 Tauri + Vue + Element Plus 方案；保留当前 UI 图、交互规格和已沉淀的页面信息架构，彻底转向 GPUI 原生渲染，以获得极致的性能、稳定性，并完全自研所有视觉组件。
**核心原则**：功能不削减、视觉不降级、交互不缩水；通过 GPUI 原生窗口、GPU 加速渲染和 Rust 单栈状态模型彻底消除跨语言通信（IPC）的成本和 WebView/前端组件库的约束。

---

## 0. 技术结论

Narya 桌面端路线全面切换为 **GPUI 原生 Rust 客户端 + Rust Control Plane Daemon + Sidecar Proxy Core**：

```text
GPUI Native Desktop UI
        ↓
In-process App State / Async Tasks / Command Dispatcher
        ↓
Rust Control Plane Daemon
        ↓
sing-box / mihomo / xray-core Sidecar
        ↓
System Proxy / TUN / VPN Service / NetworkExtension
```

路线调整原因：
- 实际开发中发现 Tauri + Vue 在某些平台上的窗口表现（如尺寸控制、多窗口流转、阴影与动画）不够稳定。
- 引入前端组件库（如 Element Plus）与 Narya 的**定制化高保真玻璃拟态**设计存在严重冲突，修改成本过高。
- GPUI 是 Rust 编写、GPU 加速的高性能 UI 框架，更适合需要实时绘制巨量连接日志、网络图表和极致响应速度的网络控制平面客户端。
- 单语言栈（Rust）可以减少维护成本、序列化开销、并在内存管理上获得绝对的安全性和掌控力。

---

## 1. 产品定位与范围

Narya 是一款跨平台 PC + App 端全能 VPN / 代理客户端，面向普通与高级用户。桌面 V1 以 GPUI 原生客户端实现，目标不削减：

- Splash 高保真启动页（完整的动态加载效果）
- Dashboard 实时监控（布局、系统控制、延迟、流量趋势图）
- 节点、配置、订阅、连接、规则、日志、工具箱、设置 等所有业务页面
- 图表使用 GPUI 原生渲染（Canvas Path）绘制
- 自研按钮、开关、滚动条、表单输入框等组件体系，深度还原 UI

---

## 2. 视觉真源与反向工程输入

GPUI 实现必须以 `ui/` 为最终视觉真源，绝不能因为框架限制而妥协：

```text
ui/*.png                 # 页面/弹窗视觉真源
ui/specs/*.spec.md       # 从 PNG 反向提取的实现规格
```

**开发约束**：
1. 没有现成前端组件库，所有的卡片、按钮、复选框、进度条必须利用 GPUI 的 Styling API (类似 Tailwind) 和 Elements 手工构建，完美对齐 1.6x 或 1:1 坐标比例。
2. 玻璃拟态 (Glassmorphism)：通过 GPUI 支持的图层混合和颜色渐变特性，尽可能逼近 `ui/` 的透明毛玻璃和阴影效果。

---

## 3. 技术架构

### 3.1 桌面 UI (narya-app)
- **框架**：`gpui` (v0.x)
- **状态管理**：`gpui::Model` / `gpui::View`，采用 Rust 原生的所有权和并发机制
- **路由/视图切换**：自研轻量级视图切换器，管理侧边栏路由状态
- **图形与动画**：`gpui` 原生动画，Canvas 用于网络与流量波形图

### 3.2 核心逻辑层 (narya-core / narya-daemon)
- 将原本计划放在 Tauri 命令中的后端逻辑无缝作为 Rust Lib 暴露给 `narya-app`。
- **并发与网络**：`tokio`
- **本地存储**：`rusqlite`
- **系统代理/TUN**：原生操作系统 API 调用

---

## 4. 目录结构规划

```text
narya/
  crates/
    narya-app/       # GPUI 主应用、UI 组件系统、视图渲染
    narya-core/      # 核心模型、领域实体
    narya-daemon/    # 后台常驻服务与调度
    narya-config/    # 配置与持久化
    narya-kernel/    # sidecar 管理 (sing-box)
    narya-platform/  # 系统代理、网卡接口
    narya-rules/     # 分流规则引擎
    narya-log/       # 日志捕获与追踪
    narya-traffic/   # 流量监控
  resources/
  architecture/
  ui/
```

## 5. 项目重置指南
所有历史中 Tauri 相关的代码、依赖（`apps/desktop`）已被移除。我们将从 `crates/narya-app` 开始，搭建 GPUI 的 `App` 与基础视窗。
