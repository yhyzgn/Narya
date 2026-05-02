# Phase 9 Design: Subscription Parsing & Platform Polish

**Goal:** Implement the logic for remote node acquisition and enhance the multi-platform experience.

**Architecture:** 
- **`narya-subscription`**: A new crate dedicated to parsing various subscription formats into the shared `Node` model.
- **`MacOSProxy`**: A new backend for `SystemProxy` using the `networksetup` utility.
- **`AppState` UI Logic**: Enhanced sorting and filtering mechanisms for large node lists.

**Tech Stack:** Rust, `serde_yaml`, `base64`, `reqwest`.

---

## 1. Subscription Parsing (`narya-subscription`)

We will support three primary formats:

### Parser Logic:
- **Base64**: Standard SS/V2Ray lists. Decode and parse individual URI schemes.
- **Clash YAML**: Extract items from the `proxies` field.
- **Merge Strategy**: New nodes from subscriptions are merged into the active `Profile`, avoiding duplicates based on `id` or `address`.

## 2. macOS System Proxy Implementation

The `SystemProxy` trait will be extended with a macOS backend.

```rust
pub struct MacOSNetworkSetup;

impl SystemProxy for MacOSNetworkSetup {
    async fn set_enabled(&self, enabled: bool) -> Result<()> {
        let state = if enabled { "on" } else { "off" };
        // Example: networksetup -setwebproxy "Wi-Fi" 127.0.0.1 2080
        // networksetup -setwebproxystate "Wi-Fi" on
        Command::new("networksetup")
            .args(["-setwebproxystate", "Wi-Fi", state])
            .status().await?;
        Ok(())
    }
}
```

## 3. UI UX Polish (Nodes View)

We will update `AppState` to support a "View State" for the nodes list.

### Features:
- **Filtering**: A `filter_text: String` field in `AppState`. The `render_nodes_view` will only map over nodes containing this string.
- **Sorting**: An `enum SortMode { Name, Latency, Usage }`.
- **Performance**: Sorting/Filtering will be performed on the `nodes` vector before rendering.

---

## 4. Verification

- **Parser Check**: Run a unit test in `narya-subscription` with a sample Clash YAML and verify it returns the expected number of `Node` structs.
- **macOS Toggle**: On a macOS machine, toggle the proxy and verify "System Settings -> Network -> Proxies" reflects the change.
- **UI Search**: Type "SG" in the search box -> Observe only Singapore nodes are visible.
