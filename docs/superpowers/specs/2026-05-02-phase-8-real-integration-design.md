# Phase 8 Design: UI/Backend Convergence

**Goal:** Achieve full functional parity by connecting the GPUI frontend to the `narya-daemon` via a robust IPC bridge.

**Architecture:** 
- **Notification Loop**: The `IpcClient` in `narya-app` will run a background loop to listen for `IpcNotification` messages and update `AppState`.
- **Command Dispatcher**: UI components (like the Dashboard Toggle) will use `AppState` to send `IpcRequest` commands to the daemon.
- **State Reconciliation**: Ensure the UI accurately reflects the real backend state (e.g., if the proxy is already on, the toggle should be in the 'on' state upon app launch).

**Tech Stack:** Rust, GPUI, `tokio`, `serde_json`.

---

## 1. IPC Client Enhancement (`narya-app/src/ipc.rs`)

We need a persistent stream-based reader.

```rust
impl IpcClient {
    pub async fn start_notification_loop(mut self, model: WeakEntity<AppState>, mut cx: AsyncApp) {
        let mut buf = [0u8; 4096];
        loop {
            match self.stream.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    if let Ok(notif) = serde_json::from_slice::<IpcNotification>(&buf[..n]) {
                        model.update(&mut cx, |state, cx| {
                            state.handle_notification(notif, cx);
                        }).ok();
                    }
                }
                _ => break, // Connection lost, trigger reconnect logic
            }
        }
    }
}
```

## 2. Dashboard Interaction (`narya-app/src/views/dashboard.rs`)

Update the "One-Click Connect" logic.

### Components:
- **Toggle Switch**: A custom interactive component or a standard button that sends `SetSystemProxy(true/false)`.
- **Status Badge**: Shows "Running" or "Stopped" based on `AppState::kernel_status`.

## 3. AppState Logic Update

```rust
impl AppState {
    pub fn handle_notification(&mut self, notif: IpcNotification, cx: &mut Context<Self>) {
        match notif {
            IpcNotification::TrafficUpdate { down, up } => {
                // Update active node metrics
            }
            IpcNotification::StatusUpdate { running } => {
                // Update global kernel status
            }
        }
        cx.notify();
    }
}
```

---

## 4. Verification

- **Real-time Synchronization**: Toggle the proxy in the Dashboard -> Observe system proxy settings change -> Observe Sidebar Footer status update automatically.
- **Resilience**: Stop the Daemon while the App is running -> Observe UI showing "Backend Offline" -> Restart Daemon -> Observe UI automatically reconnecting.
- **Data Integrity**: Ensure net speed displayed in UI matches what the backend reports (verified via logs).
