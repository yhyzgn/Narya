# Narya 开发接力总提示词（Universal AI Handoff Prompt）

> 适用对象：Gemini / OpenCode / Codex / Claude / Cursor / Windsurf / 任意具备代码读写能力的 AI Agent。
> 目标：让任何 AI 工具在 Narya 开发阶段的任意时间点，都能读取本文件、记忆库与阶段 Prompt，安全无缝接手后续开发，确保上下文从不丢失。

---

## 0. 最高优先级工作原则

你正在接手 **Narya** 项目开发。Narya 是一款跨平台 PC + App 端全能 VPN / 代理客户端，底层支持切换 `sing-box`、`mihomo`、`xray-core` 等内核。技术路线已彻底改为 **GPUI 原生 Rust 客户端**，废弃了旧版的 Tauri + Vue 方案。

必须遵守：

1. **先读上下文，再动代码**：任何开发前必须读取：
   - `prompt.md`
   - `./.memory/README.md`
   - `./.memory/status.md`
   - `./.memory/decisions.md`
   - `./.memory/tasks.md`
   - `./.prompt/README.md`
   - `./.prompt/` 中编号最新或用户指定的阶段 Prompt
   - `./architecture/narya-gpui-architecture-design.md`
   - UI 任务必须额外读取：`./ui/specs/README.md`、`./ui/specs/_global-design-system.spec.md`、`./ui/specs/_page-component-matrix.spec.md`、对应 PNG 的 `*.spec.md`
2. **UI 实现必须严格参考 `./ui` 资源库**，不得自由发挥导致风格漂移；PNG 是最终视觉真源，`./ui/specs` 是 Gemini/Codex 的实现规格手册。组件必须在 GPUI 中按照 1:1 的视觉还原度自研，不能依赖前端组件库（无 Element Plus）。
3. **每次任务完成后必须更新记忆库 `./.memory`**，并为下一阶段在 `./.prompt` 写入可接力 Prompt。
4. **每次开发推进后必须编译、测试、运行验证**。只有全部通过后，才允许提交代码。
5. **每次有效开发推进后必须 Git 提交并推送远程仓库**。提交 message 要结构化、信息丰富，可使用少量必要 emoji / sticker 图标。
6. **不能丢上下文**：若任务中断，必须先写入 `.memory/status.md` 与 `.prompt/next-*.md`，确保下一个 AI 可恢复。
7. **不要覆盖他人工作**：开发前后都要执行 `git status`，如发现非自己产生的改动，必须保护并说明。
8. **不能伪造验证结果**：编译/测试/运行失败必须修复；无法修复则记录失败命令、错误摘要、下一步建议。

---

## 1. 项目简要技术方案

完整文档：

```text
./architecture/narya-gpui-architecture-design.md
```

### 1.1 产品定位

Narya 是跨平台 PC + App 端全能 VPN / 代理客户端，面向普通用户与高级用户，提供：

- 系统代理模式
- TUN/VPN 模式
- 订阅导入与更新
- 节点测速与快速连接
- Dashboard 实时监控
- 规则分流与规则模拟
- 连接追踪
- 日志诊断
- 多内核切换
- 自动更新、托盘、通知、安全配置

### 1.2 V1 技术路线

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

### 1.3 技术栈

- **UI 框架**：GPUI (Zed 的 Rust 高性能原生 UI 库)
- **语言**：Rust 1.75+
- **异步运行时**：Tokio / GPUI executor
- **数据与存储**：SQLite + rusqlite/sqlx, serde_json
- **侧边栏内核**：sing-box 优先，后续 mihomo、xray-core

### 1.4 推荐代码结构

实现时优先按架构文档落地：

```text
narya/
  Cargo.toml       # 主应用配置
  src/
    main.rs        # GPUI 主应用入口
  crates/
    narya-core/      # 领域模型与公共类型
    narya-daemon/    # 控制平面、内核管理
    narya-config/    # 配置解析与持久化
    narya-kernel/    # 内核适配层
    narya-platform/  # 系统代理、TUN适配
    narya-log/       # 日志处理
  resources/
  architecture/
  ui/
  .memory/
  .prompt/
```

```

---

## 2. UI 资源库说明

UI 效果图位于：

```text
./ui
```

这是开发 UI 时的视觉真源（Visual Source of Truth）。实现页面时必须逐张对照，在 GPUI 中自研所有组件（卡片、开关、表单、图表），不得使用任何现成的 Web 组件库。

### 2.1 全局设计风格

- 浅色冷白渐变背景
- 玻璃拟态白色半透明卡片
- 细蓝灰边框
- 柔和阴影
- 蓝紫渐变主色
- 绿色成功、橙色警告、红色危险
- Lucide 风格线性图标 (需转换为 GPUI SVG/Paths)
- 圆角卡片与按钮
- 左侧固定导航 + 顶部操作栏 + 底部状态栏

---

## 3. UI Specs 工作规范（Gemini / Codex 必须遵守）

为降低“看图猜 UI”的成本，`ui/specs/` 已为 `ui/` 下所有 PNG 生成文字化规格。**PNG 仍是最终视觉真源，spec 是实现手册，不是替代品。**

必须优先读取：

```text
ui/specs/README.md
ui/specs/_global-design-system.spec.md
ui/specs/_page-component-matrix.spec.md
ui/specs/gemini-handoff.json
ui/specs/spec-index.json
```

开发具体页面时，还必须读取同名 spec：
```text
ui/dashboard.png                  -> ui/specs/dashboard.spec.md
ui/settings/appearance.png        -> ui/specs/settings/appearance.spec.md
```

执行顺序：
1. 先打开源 PNG，确认视觉目标。
2. 再读取对应 `*.spec.md`，提取画布尺寸、页面用途、颜色、边界线索、组件清单。
3. 对照 `_page-component-matrix.spec.md` 确认交互控件。
4. 使用 GPUI 的原生能力自研组件并组合。
5. 实现后必须按源 PNG 同尺寸截图对比。

---

## 4. 开发接手流程

### 4.1 启动检查

进入仓库根目录后执行：

```bash
git status --short
git branch --show-current
git remote -v
```

读取记忆库和 Prompt。

### 4.2 开发后验证

所有开发验证转移到 Rust 领域：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p narya-app
```

---

## 5. 最短接力启动语

如果新会话只需要一句话开始，请使用：

```text
请先阅读 prompt.md、.memory/status.md、.memory/tasks.md、.prompt/README.md 和 .prompt 中最新阶段 Prompt，然后根据其中要求继续开发 Narya (GPUI)。开发前检查 git 状态，开发后必须编译、测试、运行，全部通过后更新 .memory 与 .prompt，最后提交并推送。
```
