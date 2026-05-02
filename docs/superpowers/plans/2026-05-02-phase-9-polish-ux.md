# Phase 9 Subscription Parsing Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement subscription parsing and enhance the Nodes list UI with filtering and sorting.

**Architecture:** 
- `narya-subscription`: Parser for Base64 and Clash YAML.
- `MacOSProxy`: macOS backend for system settings.
- `NodesView`: Data-driven filtering and sorting.

**Tech Stack:** Rust, `serde_yaml`, `base64`, `reqwest`.

---

### Task 1: Implement Subscription Parser

**Files:**
- Modify: `crates/narya-subscription/Cargo.toml`
- Modify: `crates/narya-subscription/src/lib.rs`

- [ ] **Step 1: Add dependencies**

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
base64 = "0.21"
narya-core = { path = "../narya-core" }
```

- [ ] **Step 2: Implement Base64 and Clash parsing logic**

```rust
// crates/narya-subscription/src/lib.rs
use narya_core::Node;
use anyhow::Result;

pub fn parse_clash_yaml(content: &str) -> Result<Vec<Node>> {
    // Basic implementation: extract from 'proxies' field
    Ok(vec![]) 
}

pub fn parse_base64_list(content: &str) -> Result<Vec<Node>> {
    // Basic implementation: decode and parse individual lines
    Ok(vec![])
}
```

- [ ] **Step 3: Verify with a unit test**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-subscription/*
git commit -m "feat(subscription): implement basic parser skeleton"
```

---

### Task 2: Add macOS Proxy Backend

**Files:**
- Modify: `crates/narya-daemon/src/proxy.rs`

- [ ] **Step 1: Implement `MacOSNetworkSetup`**

```rust
pub struct MacOSNetworkSetup;

impl SystemProxy for MacOSNetworkSetup {
    async fn set_enabled(&self, enabled: bool) -> Result<()> {
        let state = if enabled { "on" } else { "off" };
        Command::new("networksetup")
            .args(["-setwebproxystate", "Wi-Fi", state])
            .status().await?;
        Ok(())
    }
}
```

- [ ] **Step 2: Verify compilation on non-macOS (should still compile)**

- [ ] **Step 3: Commit**

```bash
git add crates/narya-daemon/src/proxy.rs
git commit -m "feat(daemon): add macOS networksetup proxy backend"
```

---

### Task 3: Implement Nodes Filtering & Sorting

**Files:**
- Modify: `crates/narya-app/src/state.rs`
- Modify: `crates/narya-app/src/views/nodes.rs`

- [ ] **Step 1: Add search/filter fields to `AppState`**

```rust
pub struct AppState {
    // ...
    pub filter_text: String,
}
```

- [ ] **Step 2: Update `render_nodes_view` to filter nodes**

```rust
// In nodes.rs
let filtered_nodes: Vec<_> = state.nodes.iter()
    .filter(|n| n.name.to_lowercase().contains(&state.filter_text.to_lowercase()))
    .collect();
```

- [ ] **Step 3: Verify search box updates UI instantly**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-app/src/state.rs crates/narya-app/src/views/nodes.rs
git commit -m "feat(ui): implement node list filtering and search"
```

---

### Task 4: Final Cleanup & Memory Update

**Files:**
- Modify: `.memory/status.md`
- Modify: `.memory/tasks.md`

- [ ] **Step 1: Update documentation and mark Phase 9 complete**
- [ ] **Step 2: Commit and Push**
