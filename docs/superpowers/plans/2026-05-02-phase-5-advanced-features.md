# Phase 5 Advanced Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement interactive business logic: real-time latency simulation, dynamic traffic monitoring, and the fixed "Node Details" panel.

**Architecture:** 
- Extend `AppState` with a `test_all_latency` method that spawns background tasks.
- Implement a background loop in `AppShell` or `AppState` to simulate traffic fluctuations.
- Update `Nodes` view layout to include the "Node Details" card at the bottom right.
- Enhance UI components (`Badge`, `node_card`) for state-based styling (e.g., latency colors).

**Tech Stack:** Rust, GPUI, `rand` (for simulation).

---

### Task 1: Implement Latency Simulation in `AppState`

**Files:**
- Modify: `crates/narya-app/Cargo.toml`
- Modify: `crates/narya-app/src/state.rs`

- [ ] **Step 1: Add `rand` dependency to `narya-app`**

```toml
[dependencies]
# ... existing ...
rand = "0.8"
```

- [ ] **Step 2: Implement `test_all_latency` in `AppState`**

```rust
impl AppState {
    pub fn test_all_latency(&self, cx: &mut ModelContext<Self>) {
        for node in &self.nodes {
            let id = node.id.clone();
            // Clear current latency to show loading state
            cx.update(|state, cx| {
                if let Some(node) = state.nodes.iter_mut().find(|n| n.id == id) {
                    node.latency = None; 
                }
                cx.notify();
            });

            cx.spawn(|model, mut cx| async move {
                // Simulate network delay
                let delay = 500 + rand::random::<u64>() % 2000;
                cx.background_executor().timer(Duration::from_millis(delay)).await;
                let new_latency = Some(20 + rand::random::<u32>() % 200);
                
                model.update(&mut cx, |state, cx| {
                    if let Some(node) = state.nodes.iter_mut().find(|n| n.id == id) {
                        node.latency = new_latency;
                    }
                    cx.notify();
                }).ok();
            }).detach();
        }
    }
}
```

- [ ] **Step 3: Add "Test All" button to Nodes view toolbar**

In `crates/narya-app/src/views/nodes.rs`:
```rust
// Inside render_nodes_view toolbar section
.child(
    div()
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, {
            let model = model.clone();
            move |_, _, cx| {
                model.update(cx, |state, cx| {
                    state.test_all_latency(cx);
                });
            }
        })
        .child(badge("一键测速", theme.primary))
)
```

- [ ] **Step 4: Verify compilation and manual test**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add crates/narya-app/Cargo.toml crates/narya-app/src/state.rs crates/narya-app/src/views/nodes.rs
git commit -m "feat(logic): implement real-time latency simulation and trigger"
```

---

### Task 2: Implement Dynamic Traffic Monitoring Loop

**Files:**
- Modify: `crates/narya-app/src/state.rs`
- Modify: `crates/narya-app/src/views/app_shell.rs`

- [ ] **Step 1: Implement `start_traffic_monitor` in `AppState`**

```rust
impl AppState {
    pub fn start_traffic_monitor(model: Entity<Self>, cx: &mut AppContext) {
        cx.spawn(|mut cx| async move {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                let _ = model.update(&mut cx, |state, cx| {
                    if let Some(active_id) = &state.active_node_id {
                        if let Some(node) = state.nodes.iter_mut().find(|n| n.id == *active_id) {
                            // Random fluctuation +/- 1.0 MB/s
                            node.download_speed = (node.download_speed + (rand::random::<f32>() - 0.5) * 2.0).max(0.0);
                            node.upload_speed = (node.upload_speed + (rand::random::<f32>() - 0.5) * 1.0).max(0.0);
                        }
                    }
                    cx.notify();
                });
            }
        }).detach();
    }
}
```

- [ ] **Step 2: Initialize monitor in `AppShell::open`**

```rust
    pub fn open(cx: &mut App) {
        let state = cx.new(|_| AppState::mock_data());
        AppState::start_traffic_monitor(state.clone(), cx); // Start the loop
        // ... existing open_window logic ...
    }
```

- [ ] **Step 3: Update Sidebar Status Card to show live metrics**

Ensure `AppShell::render` reads `download_speed` and `upload_speed` from the active node.

- [ ] **Step 4: Verify metrics update every second**

- [ ] **Step 5: Commit**

```bash
git add crates/narya-app/src/state.rs crates/narya-app/src/views/app_shell.rs
git commit -m "feat(logic): implement background traffic monitoring loop"
```

---

### Task 3: Implement "Node Details" Fixed Panel

**Files:**
- Modify: `crates/narya-app/src/views/nodes.rs`

- [ ] **Step 1: Refactor `render_nodes_view` to use a split bottom layout**

```rust
// Inside render_nodes_view
div()
    .flex_1()
    .flex_row() // Split bottom
    .gap_6()
    .child(div().flex_1().child("Latency Chart Placeholder")) // Left: Chart
    .child(render_node_details_card(state.active_node_id.as_deref(), &state.nodes)) // Right: Fixed Card
```

- [ ] **Step 2: Implement `render_node_details_card`**

Strictly follow `ui/nodes.png` (bottom right corner):
- Rows for Address, Protocol, Encryption, UDP, TLS, Transport, Last Test.
- Use `Theme` secondary text color for labels.

- [ ] **Step 3: Verify detail synchronization when selecting nodes**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-app/src/views/nodes.rs
git commit -m "feat(ui): implement high-fidelity Node Details fixed panel"
```

---

### Task 4: Final Polish & Memory Update

**Files:**
- Modify: `.memory/status.md`
- Modify: `.memory/tasks.md`
- Modify: `.memory/changelog.md`
- Create: `.prompt/007-phase-6-kernel-integration.md`

- [ ] **Step 1: Update status and tasks**
- [ ] **Step 2: Write next stage prompt (Kernel Integration)**
- [ ] **Step 3: Commit and push**

```bash
git add .
git commit -m "docs: complete Phase 5 implementation and update status"
git push origin main
```
