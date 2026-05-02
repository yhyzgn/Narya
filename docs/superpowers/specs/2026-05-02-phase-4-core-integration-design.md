# Phase 4 Design: Core Logic Integration & Centralized State

**Goal:** Replace static UI components with a data-driven model using a centralized `AppState` in GPUI.

**Architecture:** Use a GPUI `Model` to hold application-wide state. `AppShell` and its child views will observe this model to ensure synchronized updates (e.g., selecting a node in the Nodes view updates the Sidebar).

**Tech Stack:** Rust, GPUI, `narya-core`.

---

## 1. Domain Models (`narya-core`)

We will define the following structs in `crates/narya-core/src/lib.rs` based strictly on the provided UI designs.

### Node Struct
```rust
pub struct Node {
    pub id: String,
    pub name: String,
    pub country_code: String, // For flag icon
    pub protocol: String,     // e.g., "Shadowsocks", "VLESS"
    pub tag: Option<String>,  // e.g., "推荐", "Hysteria2"
    pub latency: Option<u32>, // in ms
    pub usage_pct: u8,
    pub download_speed: f32,  // in MB/s
    pub upload_speed: f32,    // in MB/s
    pub is_selected: bool,
    pub details: NodeDetails,
}

pub struct NodeDetails {
    pub address: String,
    pub encryption: String,
    pub udp: bool,
    pub tls: bool,
    pub skip_cert_verify: bool,
    pub transport: String,
    pub last_test: String,
}
```

### Subscription Struct
```rust
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: String,         // Key for icon type
    pub node_count: u32,
    pub used_nodes: u32,
    pub update_time: String,
    pub traffic_used: f64,    // in GB
    pub traffic_total: f64,   // in GB
    pub expiration: String,   // Date string
    pub status: String,       // e.g., "当前使用", "运行中"
}
```

---

## 2. App State Management (`narya-app`)

We will introduce a global `AppState` model.

```rust
// crates/narya-app/src/state.rs
pub struct AppState {
    pub nodes: Vec<Node>,
    pub subscriptions: Vec<Subscription>,
    pub active_node_id: Option<String>,
}

impl AppState {
    pub fn mock_data() -> Self {
        // Initialize with high-fidelity mock data matching nodes.png and subscriptions.png
    }
}
```

---

## 3. Integration Plan

### AppShell Update
- `AppShell` will hold a `Model<AppState>`.
- In `render`, it will observe the model.

### Views Update
- `render_nodes_view` will accept `Model<AppState>` or the data from it.
- Clicking a node card will call `cx.update_model` to change `active_node_id`.
- The Sidebar Footer will dynamically display the `active_node_id`'s name and latency.

### Components Refinement
- Update `node_item` and `subscription_card` to accept the new structs.

---

## 4. Verification

- **Sync Check**: Selecting a node in the Nodes list must immediately change the connection status in the Sidebar Footer.
- **Data Fidelity**: Text labels and metrics (e.g., "12.4 MB/s") must match the UI design samples.
- **Clippy**: Ensure all cross-crate imports are correctly handled.
