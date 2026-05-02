# Phase 7 Design: Kernel Orchestration & System Control

**Goal:** Implement the core functional engine in `narya-daemon` to manage the `sing-box` kernel and system-wide network settings.

**Architecture:** 
- **`KernelManager`**: Manages the `sing-box` subprocess using `tokio::process`. Handles configuration generation and status monitoring.
- **`SystemProxy`**: A trait-based abstraction for enabling/disabling system proxy settings across different operating systems.
- **`ConfigStore`**: Handles YAML persistence for profiles and application settings in `narya-config`.

**Tech Stack:** Rust, `tokio`, `serde_yaml`, `anyhow`.

---

## 1. Kernel Management (`narya-daemon`)

The daemon will maintain a singleton `KernelManager`.

### Struct Definition:
```rust
pub struct KernelManager {
    child: Option<Child>,
    status: KernelStatus,
}

#[derive(Serialize, Deserialize)]
pub enum KernelStatus {
    Idle,
    Running { pid: u32, uptime: u64 },
    Error(String),
}
```

### Flow:
1. UI sends `StartKernel(profile_id)`.
2. Daemon generates `config.json` for `sing-box`.
3. Daemon spawns `sing-box` and monitors its lifecycle.
4. Daemon streams stdout/stderr to extract traffic metrics.

## 2. System Proxy Control

We will implement a cross-platform command executor.

### Supported Backends:
- **Linux (GNOME/KDE)**: `gsettings` commands.
- **macOS**: `networksetup` utility.

### Example Logic:
```rust
async fn toggle_proxy(enable: bool) -> Result<()> {
    let mode = if enable { "manual" } else { "none" };
    // Linux example
    Command::new("gsettings")
        .args(["set", "org.gnome.system.proxy", "mode", mode])
        .status().await?;
    Ok(())
}
```

## 3. Configuration Persistence (`narya-config`)

- **Location**: `~/.config/narya/profiles.yaml`.
- **Logic**: Use `serde_yaml` to serialize/deserialize node lists and rules.

---

## 4. IPC Extension

Add new methods to `narya-ipc`:
- `ApplyProfile(String)`
- `SetSystemProxy(bool)`
- `GetKernelLogs`

## 5. Verification

- **Process Isolation**: Closing the App should leave the Daemon running (if configured), but the Daemon must reliably kill the Kernel when requested.
- **State Feedback**: Toggle the proxy in UI -> observe OS system settings change -> observe "Connected" status in UI Footer.
- **Real Metrics**: UI Sidebar Footer should show non-zero网速 when traffic is passing through `sing-box`.
