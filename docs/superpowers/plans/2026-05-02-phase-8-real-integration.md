# Phase 8 UI/Backend Convergence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the functional loop between the high-fidelity UI and the system-level daemon.

**Architecture:** 
- `IpcClient`: Persistent notification loop using `AsyncApp`.
- `AppState`: Real-time state updates from `IpcNotification`.
- `Dashboard`: Interactive controls for proxy orchestration.

**Tech Stack:** Rust, GPUI, `tokio`, `serde_json`.

---

### Task 1: Enhance `IpcClient` with Notification Loop

**Files:**
- Modify: `crates/narya-app/src/ipc.rs`
- Modify: `crates/narya-app/src/state.rs`

- [ ] **Step 1: Implement `read_notification` in `IpcClient`**

```rust
// crates/narya-app/src/ipc.rs
use narya_ipc::IpcNotification;

impl IpcClient {
    pub async fn next_notification(&mut self) -> Result<IpcNotification> {
        let mut buf = [0u8; 4096];
        let n = self.stream.read(&mut buf).await?;
        if n == 0 { anyhow::bail!("Connection closed"); }
        let notification: IpcNotification = serde_json::from_slice(&buf[..n])?;
        Ok(notification)
    }
}
```

- [ ] **Step 2: Update `start_traffic_monitor` to handle notifications**

Refactor `AppState::start_traffic_monitor` to loop over `client.next_notification()` and call `state.handle_notification()`.

- [ ] **Step 3: Implement `handle_notification` in `AppState`**

```rust
// crates/narya-app/src/state.rs
use narya_ipc::IpcNotification;

impl AppState {
    pub fn handle_notification(&mut self, notif: IpcNotification, cx: &mut Context<Self>) {
        match notif {
            IpcNotification::TrafficUpdate { down, up } => {
                if let Some(active_id) = &self.active_node_id {
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == *active_id) {
                        node.download_speed = down;
                        node.upload_speed = up;
                    }
                }
            }
            IpcNotification::StatusUpdate { running } => {
                // Update a new field: pub kernel_running: bool
            }
        }
        cx.notify();
    }
}
```

- [ ] **Step 4: Verify compilation**

- [ ] **Step 5: Commit**

```bash
git add crates/narya-app/src/ipc.rs crates/narya-app/src/state.rs
git commit -m "feat(app): implement IPC notification loop and state synchronization"
```

---

### Task 2: Implement Dashboard Interactive Toggle

**Files:**
- Modify: `crates/narya-app/src/views/dashboard.rs`
- Modify: `crates/narya-app/src/state.rs`

- [ ] **Step 1: Add `toggle_proxy` method to `AppState`**

```rust
impl AppState {
    pub fn toggle_proxy(&mut self, cx: &mut Context<Self>) {
        // Send SetSystemProxy request via IpcClient
        // For now, toggle the local state to simulate until IPC is fully wired
    }
}
```

- [ ] **Step 2: Update Dashboard UI to use the toggle**

Find the "One-Click Connect" button and hook up the `on_mouse_down` event.

- [ ] **Step 3: Verify toggle updates the status card in sidebar**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-app/src/views/dashboard.rs crates/narya-app/src/state.rs
git commit -m "feat(ui): connect Dashboard toggle to proxy orchestration logic"
```

---

### Task 3: Final Integration & Handoff

**Files:**
- Modify: `.memory/status.md`
- Modify: `.memory/tasks.md`
- Modify: `.memory/changelog.md`

- [ ] **Step 1: Perform full workspace verification**
- [ ] **Step 2: Update project documentation**
- [ ] **Step 3: Commit and Push**
