# Phase 6 Design: Backend Kernel Integration & IPC Skeleton

**Goal:** Establish the `narya-daemon` background service and implement a robust IPC mechanism for system-level control.

**Architecture:** 
- **`narya-daemon`**: A `tokio`-based standalone service that runs as a background process.
- **`narya-ipc`**: A shared library defining the JSON-RPC communication protocol and UDS (Unix Domain Socket) transport logic.
- **`narya-app`**: The GPUI frontend, which acts as an IPC client to send commands (e.g., `set_proxy`) and receive stream updates (e.g., `traffic_stats`).

**Tech Stack:** Rust, `tokio`, `serde_json`, `uds`.

---

## 1. IPC Protocol Definition (`narya-ipc`)

We will use a simplified JSON-RPC 2.0 style over UDS.

### Shared Structs:
```rust
#[derive(Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum IpcNotification {
    TrafficUpdate { down: f32, up: f32 },
    KernelStatus { running: bool, mode: String },
}
```

## 2. Daemon Implementation (`narya-daemon`)

The daemon will:
1. Initialize a `tokio::net::UnixListener` at `/tmp/narya.sock`.
2. Spawn a handler task for each incoming UI connection.
3. Orchestrate the `sing-box` kernel process.
4. Execute shell commands for system proxy settings.

### System Proxy Logic (Linux Example):
```rust
pub async fn set_system_proxy(enabled: bool) -> Result<()> {
    let mode = if enabled { "manual" } else { "none" };
    Command::new("gsettings")
        .args(["set", "org.gnome.system.proxy", "mode", mode])
        .status()
        .await?;
    Ok(())
}
```

## 3. App Integration (`narya-app`)

- Add a `DaemonClient` to `AppState`.
- Use `cx.spawn` to maintain a persistent UDS connection.
- Update `AppState` fields dynamically when `IpcNotification` is received.

---

## 4. Verification

- **Connectivity**: Run daemon, then launch app. Check "Narya v1.0.0 | Kernels: sing-box (Active)" in footer.
- **Proxy Toggle**: Click a (future) switch in Settings, verify `gsettings` or `networksetup` output change.
- **Real Metrics**: Replace Phase 5 random simulation with data streamed from the daemon.
