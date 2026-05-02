# Phase 10 Design: Production Polish & System Tray

**Goal:** Finalize the application for production usage with remote data fetching, background management, and production-grade kernel configurations.

**Architecture:** 
- **`narya-subscription`**: Adds networking layer using `reqwest` (with `rustls`).
- **`narya-daemon`**: Implement a template-based `sing-box` config generator.
- **`narya-app`**: System tray integration for background operations and "Toast" notifications.

**Tech Stack:** Rust, `reqwest`, `tray-icon`, `serde_json`.

---

## 1. Remote Fetching (`narya-subscription`)

We will add a secure network client.

```rust
pub async fn fetch_remote_subscription(url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("Clash/1.0 Narya/1.0")
        .timeout(Duration::from_secs(10))
        .build()?;
        
    let response = client.get(url).send().await?;
    let content = response.text().await?;
    Ok(content)
}
```

## 2. Config Synthesis (`narya-daemon`)

The daemon will generate a complete `sing-box` JSON configuration.

### Template Sections:
- **Inbounds**: SOCKS5 (1080) and HTTP (2080).
- **DNS**: System DNS + Rule-based hijacking.
- **Outbounds**: 
    - The selected proxy node (Shadowsocks, Hysteria2, etc.).
    - Direct (bypass).
    - DNS (for resolutions).
- **Route**: Rule sets for auto-bypass.

## 3. System Tray (`narya-app`)

We will use the `tray-icon` crate to provide a persistent control point.

### Menu Items:
1. **Show MainWindow**: Restore the GPUI window.
2. **One-Click Connect**: Toggle proxy without opening the app.
3. **Exit**: Terminate both App and Daemon.

## 4. Feedback UI (Toasts)

A floating notification component in `crates/narya-app/src/components.rs`.

```rust
pub fn toast(message: &str, kind: ToastKind) -> impl IntoElement {
    // Floating card with entry/exit animations (simulated)
    div()
        .absolute()
        .bottom_10()
        .right_10()
        .child(glass_card().child(message))
}
```

---

## 5. Verification

- **Real Fetch**: Add a real Clash URL -> Verify nodes list is populated from the web.
- **Tray Check**: Minimize app -> Right click tray icon -> Click 'Connect' -> Verify system proxy is on.
- **Production Config**: Verify that `sing-box` starts with the generated JSON and actually proxies traffic.
