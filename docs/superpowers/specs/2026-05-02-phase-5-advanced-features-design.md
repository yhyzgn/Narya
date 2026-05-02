# Phase 5 Design: Advanced Features & Interactive Logic

**Goal:** Implement interactive business logic including real-time latency simulation, dynamic traffic monitoring, and high-fidelity detail panels.

**Architecture:** Extend the `AppState` model to include asynchronous methods for testing and monitoring. Use GPUI's `cx.spawn` to handle concurrent state updates without blocking the UI thread.

**Tech Stack:** Rust, GPUI, `narya-core`.

---

## 1. Latency Testing Logic

We will add a `test_latency` method to `AppState` in `crates/narya-app/src/state.rs`.

```rust
impl AppState {
    pub fn test_all_latency(&self, cx: &mut ModelContext<Self>) {
        for node in &self.nodes {
            let id = node.id.clone();
            cx.spawn(|model, mut cx| async move {
                // Simulate network delay
                cx.background_executor().timer(Duration::from_millis(500 + rand::random::<u64>() % 2000)).await;
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

## 2. Dynamic Traffic Monitoring

Implement a background loop in `AppShell::open` or `AppState::new`.

- **Frequency**: Every 1 second.
- **Logic**: Randomly fluctuate `download_speed` and `upload_speed` values for the active node.
- **UI Integration**: The Sidebar Status Card will automatically reflect these changes via model observation.

## 3. Node Details Panel (Option 1 - Fixed)

Following `ui/nodes.png`, the `Nodes` view will be split into:
- **Top**: Toolbar (Search + Filters).
- **Middle**: Grid/List of nodes.
- **Bottom**: 
    - Left: Latency Trend Chart.
    - Right: **Node Details Card** (Fixed position).

### Details Card Fields:
- Address (e.g., `hkg01.narya.net:443`)
- Protocol (`Shadowsocks`)
- Encryption (`2022-blake3...`)
- Transport (`tcp`)
- TLS (`否`)
- UDP (`已启用`)

## 4. UI Polish

- **Latency Badges**: Update color based on value (Green < 100ms, Yellow < 300ms, Red > 300ms).
- **Hover Effects**: Ensure all list items and buttons have the #F3F4F6 hover feedback.

---

## 5. Verification

- **Latency Check**: Clicking "Test All" should show progress/loading states on badges, then return values.
- **Traffic Check**: Observe the Sidebar speed metrics; they should change every second.
- **Detail Check**: Selecting different nodes must update the bottom-right details card instantly.
