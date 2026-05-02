# Phase 4 Core Logic Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate real data models from `narya-core` into the GPUI frontend and implement dynamic state synchronization.

**Architecture:** Define domain models (`Node`, `Subscription`) in the `narya-core` crate. Introduce a centralized `AppState` model in the `narya-app` crate. The `AppShell` and its child views will read from and mutate this model, using GPUI's `update` mechanism to trigger UI rerenders automatically when the state changes.

**Tech Stack:** Rust, GPUI.

---

### Task 1: Define Domain Models in `narya-core`

**Files:**
- Modify: `crates/narya-core/src/lib.rs`

- [ ] **Step 1: Write a basic test to ensure models can be instantiated**

```rust
// In crates/narya-core/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_instantiation() {
        let details = NodeDetails {
            address: "test.example.com".to_string(),
            encryption: "none".to_string(),
            udp: true,
            tls: false,
            skip_cert_verify: false,
            transport: "tcp".to_string(),
            last_test: "Just now".to_string(),
        };
        let node = Node {
            id: "1".to_string(),
            name: "Test Node".to_string(),
            country_code: "US".to_string(),
            protocol: "Shadowsocks".to_string(),
            tag: None,
            latency: Some(42),
            usage_pct: 0,
            download_speed: 0.0,
            upload_speed: 0.0,
            details,
        };
        assert_eq!(node.name, "Test Node");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p narya-core`
Expected: FAIL (structs not found)

- [ ] **Step 3: Implement domain models**

```rust
// Replace contents of crates/narya-core/src/lib.rs
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub country_code: String,
    pub protocol: String,
    pub tag: Option<String>,
    pub latency: Option<u32>,
    pub usage_pct: u8,
    pub download_speed: f32,
    pub upload_speed: f32,
    pub details: NodeDetails,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeDetails {
    pub address: String,
    pub encryption: String,
    pub udp: bool,
    pub tls: bool,
    pub skip_cert_verify: bool,
    pub transport: String,
    pub last_test: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: String,
    pub node_count: u32,
    pub used_nodes: u32,
    pub update_time: String,
    pub traffic_used: f64,
    pub traffic_total: f64,
    pub expiration: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_instantiation() {
        let details = NodeDetails {
            address: "test.example.com".to_string(),
            encryption: "none".to_string(),
            udp: true,
            tls: false,
            skip_cert_verify: false,
            transport: "tcp".to_string(),
            last_test: "Just now".to_string(),
        };
        let node = Node {
            id: "1".to_string(),
            name: "Test Node".to_string(),
            country_code: "US".to_string(),
            protocol: "Shadowsocks".to_string(),
            tag: None,
            latency: Some(42),
            usage_pct: 0,
            download_speed: 0.0,
            upload_speed: 0.0,
            details,
        };
        assert_eq!(node.name, "Test Node");
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p narya-core`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/narya-core/src/lib.rs
git commit -m "feat(core): define Node and Subscription domain models"
```

---

### Task 2: Create `AppState` Model in `narya-app`

**Files:**
- Create: `crates/narya-app/src/state.rs`
- Modify: `crates/narya-app/src/lib.rs`

- [ ] **Step 1: Write a basic test for AppState initialization**

```rust
// At the bottom of crates/narya-app/src/state.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_mock() {
        let state = AppState::mock_data();
        assert!(!state.nodes.is_empty());
        assert!(!state.subscriptions.is_empty());
        assert!(state.active_node_id.is_some());
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p narya-app`
Expected: FAIL (module not found)

- [ ] **Step 3: Implement `AppState` with mock data**

```rust
// In crates/narya-app/src/state.rs
use narya_core::{Node, NodeDetails, Subscription};

pub struct AppState {
    pub nodes: Vec<Node>,
    pub subscriptions: Vec<Subscription>,
    pub active_node_id: Option<String>,
}

impl AppState {
    pub fn mock_data() -> Self {
        let nodes = vec![
            Node {
                id: "sg-01".to_string(),
                name: "SG-01 (Singapore)".to_string(),
                country_code: "SG".to_string(),
                protocol: "Shadowsocks".to_string(),
                tag: Some("推荐".to_string()),
                latency: Some(12),
                usage_pct: 23,
                download_speed: 12.4,
                upload_speed: 4.6,
                details: NodeDetails {
                    address: "sg01.narya.net:443".to_string(),
                    encryption: "2022-blake3-aes-128-gcm".to_string(),
                    udp: true,
                    tls: false,
                    skip_cert_verify: false,
                    transport: "tcp".to_string(),
                    last_test: "刚刚".to_string(),
                },
            },
            Node {
                id: "us-01".to_string(),
                name: "US-West (Oregon)".to_string(),
                country_code: "US".to_string(),
                protocol: "VLESS".to_string(),
                tag: None,
                latency: Some(156),
                usage_pct: 32,
                download_speed: 8.7,
                upload_speed: 2.9,
                details: NodeDetails {
                    address: "us01.narya.net:443".to_string(),
                    encryption: "none".to_string(),
                    udp: true,
                    tls: true,
                    skip_cert_verify: false,
                    transport: "ws".to_string(),
                    last_test: "10 mins ago".to_string(),
                },
            },
        ];

        let subscriptions = vec![
            Subscription {
                id: "sub-1".to_string(),
                name: "Premium Plan".to_string(),
                url: "https://example.com/sub1".to_string(),
                icon: "plane".to_string(),
                node_count: 128,
                used_nodes: 38,
                update_time: "刚刚".to_string(),
                traffic_used: 436.0,
                traffic_total: 1280.0, // 1.28 TB in GB
                expiration: "2026-06-10".to_string(),
                status: "运行中".to_string(),
            },
            Subscription {
                id: "sub-2".to_string(),
                name: "Free Trial".to_string(),
                url: "https://example.com/sub2".to_string(),
                icon: "box".to_string(),
                node_count: 5,
                used_nodes: 5,
                update_time: "2 days ago".to_string(),
                traffic_used: 8.2,
                traffic_total: 10.0,
                expiration: "2026-05-04".to_string(),
                status: "已过期".to_string(),
            },
        ];

        Self {
            active_node_id: Some("sg-01".to_string()),
            nodes,
            subscriptions,
        }
    }
}

// Keep the test block here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_mock() {
        let state = AppState::mock_data();
        assert!(!state.nodes.is_empty());
        assert!(!state.subscriptions.is_empty());
        assert!(state.active_node_id.is_some());
    }
}
```

- [ ] **Step 4: Expose `state` module**

In `crates/narya-app/src/lib.rs`:
```rust
// Add near top:
pub mod state;
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test -p narya-app`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/narya-app/src/state.rs crates/narya-app/src/lib.rs
git commit -m "feat(app): implement AppState model with mock data"
```

---

### Task 3: Integrate `AppState` into `AppShell`

**Files:**
- Modify: `crates/narya-app/src/views/app_shell.rs`

- [ ] **Step 1: Add Model<AppState> to AppShell**

```rust
use crate::state::AppState;
// ... other imports

pub struct AppShell {
    pub(super) active_view: ActiveView,
    pub(super) handle: WeakEntity<Self>,
    pub(super) state: Model<AppState>,
}
```

- [ ] **Step 2: Initialize AppState when opening main window**

Update `AppShell::open`:
```rust
    pub fn open(cx: &mut App) {
        let state = cx.new_model(|_| AppState::mock_data()); // Create the model
        let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |_, cx| {
                cx.new(|cx| AppShell {
                    active_view: ActiveView::Dashboard,
                    handle: cx.entity().downgrade(),
                    state,
                })
            },
        )
        .expect("failed to open main window");
    }
```

- [ ] **Step 3: Render dynamic side-bar footer status**

In `AppShell::render`:
```rust
        let state_ref = self.state.read(cx);
        let active_node_name = state_ref.active_node_id.as_ref()
            .and_then(|id| state_ref.nodes.iter().find(|n| n.id == *id))
            .map(|n| n.name.clone())
            .unwrap_or_else(|| "Not Connected".to_string());

        // ... inside the Sidebar Footer section ...
                                    div()
                                        .ml_3()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child(format!("Connected to {}", active_node_name)),
```

- [ ] **Step 4: Verify compilation**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add crates/narya-app/src/views/app_shell.rs
git commit -m "feat(ui): inject AppState model into AppShell and render dynamic footer status"
```

---

### Task 4: Refactor `Nodes` View to be Data-Driven

**Files:**
- Modify: `crates/narya-app/src/views/nodes.rs`
- Modify: `crates/narya-app/src/views/app_shell.rs`

- [ ] **Step 1: Update `render_nodes_view` signature**

Modify `render_nodes_view` in `nodes.rs` to accept the model and the context:
```rust
use crate::state::AppState;
// ... imports

pub fn render_nodes_view(model: &Model<AppState>, cx: &mut ViewContext<AppShell>) -> impl IntoElement {
    let theme = Theme::default();
    let state = model.read(cx);
    
    div()
        .flex_col()
        .size_full()
        // ... toolbar ...
        .child(
            // Node List
            div()
                .flex_1()
                .overflow_hidden()
                .flex_col()
                .gap_3()
                .children(state.nodes.iter().map(|n| {
                    let is_selected = state.active_node_id.as_deref() == Some(&n.id);
                    node_card(n, is_selected, {
                        let model = model.clone();
                        let node_id = n.id.clone();
                        move |_, cx| {
                            model.update(cx, |state, cx| {
                                state.active_node_id = Some(node_id.clone());
                                cx.notify();
                            });
                        }
                    })
                }))
        )
}
```

- [ ] **Step 2: Update `node_card` signature and implementation**

```rust
use narya_core::Node;

pub fn node_card(
    node: &Node,
    selected: bool,
    on_click: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
) -> impl IntoElement {
    let theme = Theme::default();
    let name = node.name.clone();
    let protocol = node.protocol.clone();
    let latency_str = node.latency.map(|l| format!("{}ms", l)).unwrap_or_else(|| "--".to_string());
    
    // Use on_click on the Connect button or the card itself. Let's put it on the card for ease.
    glass_card().p_4()
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, on_click)
        .child(
        // ... existing card layout using `name`, `protocol`, `latency_str` ...
```

- [ ] **Step 3: Update `AppShell::render` to pass dependencies**

In `crates/narya-app/src/views/app_shell.rs`:
```rust
                                ActiveView::Nodes => render_nodes_view(&self.state, cx).into_any_element(),
```

- [ ] **Step 4: Verify compilation**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add crates/narya-app/src/views/nodes.rs crates/narya-app/src/views/app_shell.rs
git commit -m "feat(ui): make Nodes view dynamic and handle node selection state"
```

---

### Task 5: Refactor `Subscriptions` View to be Data-Driven

**Files:**
- Modify: `crates/narya-app/src/views/subscriptions.rs`
- Modify: `crates/narya-app/src/views/app_shell.rs`

- [ ] **Step 1: Update `render_subscriptions_view` signature**

```rust
use crate::state::AppState;
// ... imports

pub fn render_subscriptions_view(model: &Model<AppState>, cx: &mut ViewContext<AppShell>) -> impl IntoElement {
    let state = model.read(cx);
    div()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .gap_6()
                .children(state.subscriptions.iter().map(|sub| subscription_card(sub)))
        )
}
```

- [ ] **Step 2: Update `subscription_card` signature and logic**

```rust
use narya_core::Subscription;

pub fn subscription_card(sub: &Subscription) -> impl IntoElement {
    let theme = Theme::default();
    let usage_ratio = (sub.traffic_used / sub.traffic_total) as f32;
    let title = sub.name.clone();
    let expiry = sub.expiration.clone();
    let status = sub.status.clone();
    let usage_str = format!("{:.1} / {:.1} GB", sub.traffic_used, sub.traffic_total);
    
    // ... existing layout ...
                    .child(badge(status, theme.success)),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.text_secondary)
                    .mb_1()
                    .child("Traffic Usage")
                    .child(usage_str),
            )
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .bg(theme.border)
                    .rounded_full()
                    .mb_4()
                    .child(
                        div()
                            .h_full()
                            .w(relative(usage_ratio))
                            .bg(if usage_ratio > 0.8 {
                                theme.danger
                            } else {
                                theme.primary
                            })
                            .rounded_full(),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(div().text_xs().text_color(theme.text_muted).child(format!("Expires: {}", expiry)))
    // ...
```

- [ ] **Step 3: Update `AppShell::render` to pass dependencies**

In `crates/narya-app/src/views/app_shell.rs`:
```rust
                                ActiveView::Subscriptions => {
                                    render_subscriptions_view(&self.state, cx).into_any_element()
                                }
```

- [ ] **Step 4: Verify compilation**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add crates/narya-app/src/views/subscriptions.rs crates/narya-app/src/views/app_shell.rs
git commit -m "feat(ui): make Subscriptions view dynamic driven by AppState model"
```

---

### Task 6: Final Memory Update & Cleanup

**Files:**
- Modify: `.memory/status.md`
- Modify: `.memory/tasks.md`
- Modify: `.memory/changelog.md`
- Create: `.prompt/006-phase-5-advanced-features.md`

- [ ] **Step 1: Update status and tasks**
- [ ] **Step 2: Write next stage prompt**
- [ ] **Step 3: Commit and push**

```bash
git add .
git commit -m "docs: complete Phase 4 implementation and update status"
git push origin main
```