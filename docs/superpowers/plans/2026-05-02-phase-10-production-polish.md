# Phase 10 Production Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Finalize the application with remote fetching, tray support, and production-grade kernel configs.

**Architecture:** 
- `narya-subscription`: HTTP client layer.
- `narya-daemon`: Logic for synthesizing `sing-box` JSON.
- `narya-app`: UI notifications and system tray management.

**Tech Stack:** Rust, `reqwest`, `tray-icon`, `serde_json`.

---

### Task 1: Implement Remote Fetching

**Files:**
- Modify: `crates/narya-subscription/Cargo.toml`
- Modify: `crates/narya-subscription/src/lib.rs`

- [ ] **Step 1: Add `reqwest` dependency**

```toml
[dependencies]
# ... existing ...
reqwest = { version = "0.11", features = ["rustls-tls", "json"] }
```

- [ ] **Step 2: Implement `fetch_remote_subscription`**

```rust
pub async fn fetch_remote_subscription(url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("Clash/1.0 Narya/1.0")
        .build()?;
    let content = client.get(url).send().await?.text().await?;
    Ok(content)
}
```

- [ ] **Step 3: Verify compilation**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-subscription/*
git commit -m "feat(subscription): implement remote network fetching"
```

---

### Task 2: Implement Config Generator

**Files:**
- Modify: `crates/narya-daemon/src/kernel.rs`

- [ ] **Step 1: Implement `generate_config` helper**

Create a function that returns a `serde_json::Value` with standard sing-box sections (`inbounds`, `outbounds`, `dns`, `route`).

- [ ] **Step 2: Update `KernelManager::start` to use the generated config**

- [ ] **Step 3: Verify kernel starts with real config**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-daemon/src/kernel.rs
git commit -m "feat(daemon): implement production sing-box config generator"
```

---

### Task 3: Add System Tray and Toast UI

**Files:**
- Modify: `crates/narya-app/Cargo.toml`
- Modify: `crates/narya-app/src/lib.rs`
- Modify: `crates/narya-app/src/components.rs`

- [ ] **Step 1: Add `tray-icon` dependency**

- [ ] **Step 2: Initialize Tray Icon in `narya_app::run`**

- [ ] **Step 3: Implement `Toast` component in `components.rs`**

- [ ] **Step 4: Verify UI polish**

- [ ] **Step 5: Commit and Push**
