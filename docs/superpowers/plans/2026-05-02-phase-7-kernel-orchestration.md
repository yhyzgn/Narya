# Phase 7 Kernel Orchestration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement system proxy control and `sing-box` kernel management in `narya-daemon`.

**Architecture:** 
- `SystemProxy`: Cross-platform command executor for network settings.
- `KernelManager`: Lifecycle management for the `sing-box` process.
- `ConfigStore`: YAML persistence for profiles in `narya-config`.

**Tech Stack:** Rust, `tokio::process`, `serde_yaml`.

---

### Task 1: Implement `SystemProxy` Control

**Files:**
- Create: `crates/narya-daemon/src/proxy.rs`
- Modify: `crates/narya-daemon/src/main.rs`

- [ ] **Step 1: Implement `SystemProxy` trait and Linux backend**

```rust
// crates/narya-daemon/src/proxy.rs
use anyhow::Result;
use tokio::process::Command;

pub trait SystemProxy {
    async fn set_enabled(&self, enabled: bool) -> Result<()>;
}

pub struct LinuxGSettings;

impl SystemProxy for LinuxGSettings {
    async fn set_enabled(&self, enabled: bool) -> Result<()> {
        let mode = if enabled { "manual" } else { "none" };
        Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", mode])
            .status()
            .await?;
        Ok(())
    }
}
```

- [ ] **Step 2: Expose proxy module in `main.rs`**

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p narya-daemon`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add crates/narya-daemon/src/proxy.rs crates/narya-daemon/src/main.rs
git commit -m "feat(daemon): implement system proxy control abstraction"
```

---

### Task 2: Implement `KernelManager`

**Files:**
- Create: `crates/narya-daemon/src/kernel.rs`
- Modify: `crates/narya-daemon/src/main.rs`

- [ ] **Step 1: Implement `KernelManager` for process control**

```rust
// crates/narya-daemon/src/kernel.rs
use tokio::process::{Child, Command};
use anyhow::Result;

pub struct KernelManager {
    child: Option<Child>,
}

impl KernelManager {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub async fn start(&mut self, binary_path: &str, config_path: &str) -> Result<()> {
        self.stop().await?;
        let child = Command::new(binary_path)
            .args(["run", "-c", config_path])
            .spawn()?;
        self.child = Some(child);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill().await?;
        }
        Ok(())
    }
}
```

- [ ] **Step 2: Expose kernel module in `main.rs`**

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p narya-daemon`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add crates/narya-daemon/src/kernel.rs crates/narya-daemon/src/main.rs
git commit -m "feat(daemon): implement kernel process manager"
```

---

### Task 3: Implement Config Persistence in `narya-config`

**Files:**
- Modify: `crates/narya-config/Cargo.toml`
- Modify: `crates/narya-config/src/lib.rs`

- [ ] **Step 1: Add `serde_yaml` dependency**

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
narya-core = { path = "../narya-core" }
```

- [ ] **Step 2: Implement Profile Save/Load**

```rust
use narya_core::Node;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub nodes: Vec<Node>,
}

pub fn save_profile(path: PathBuf, profile: &Profile) -> anyhow::Result<()> {
    let yaml = serde_yaml::to_string(profile)?;
    fs::write(path, yaml)?;
    Ok(())
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p narya-config`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add crates/narya-config/src/lib.rs crates/narya-config/Cargo.toml
git commit -m "feat(config): implement YAML profile persistence"
```

---

### Task 4: Integrate Orchestration into Daemon Main

**Files:**
- Modify: `crates/narya-daemon/src/main.rs`

- [ ] **Step 1: Handle `SetSystemProxy` and `StartKernel` messages**

Update the IPC message handler in `main.rs` to use `SystemProxy` and `KernelManager`.

- [ ] **Step 2: Verify full backend flow**

- [ ] **Step 3: Commit and Push**
