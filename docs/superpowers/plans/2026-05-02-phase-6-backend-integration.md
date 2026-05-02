# Phase 6 Backend Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the IPC communication bridge between `narya-app` and `narya-daemon`.

**Architecture:** 
- `narya-ipc`: Shared JSON-RPC request/response types.
- `narya-daemon`: Tokio-based UDS server handling commands.
- `narya-app`: IPC client in `AppState` for real-time synchronization.

**Tech Stack:** Rust, `tokio`, `serde_json`, `interprocess` (for UDS).

---

### Task 1: Define IPC Protocol in `narya-ipc`

**Files:**
- Modify: `crates/narya-ipc/Cargo.toml`
- Modify: `crates/narya-ipc/src/lib.rs`

- [ ] **Step 1: Add dependencies to `narya-ipc`**

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

- [ ] **Step 2: Define Request/Response types**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[tag = "type"]
pub enum IpcNotification {
    TrafficUpdate { down: f32, up: f32 },
    StatusUpdate { running: bool },
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p narya-ipc`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add crates/narya-ipc/src/lib.rs crates/narya-ipc/Cargo.toml
git commit -m "feat(ipc): define JSON-RPC protocol structs"
```

---

### Task 2: Implement `narya-daemon` Skeleton

**Files:**
- Modify: `crates/narya-daemon/Cargo.toml`
- Create: `crates/narya-daemon/src/main.rs`

- [ ] **Step 1: Add dependencies to `narya-daemon`**

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
anyhow = "1.0"
narya-ipc = { path = "../narya-ipc" }
```

- [ ] **Step 2: Implement UDS listener loop**

```rust
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::os::unix::net::UnixListener as StdUnixListener;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let socket_path = "/tmp/narya.sock";
    let _ = std::fs::remove_file(socket_path);
    
    let listener = UnixListener::bind(socket_path)?;
    println!("Daemon listening on {}", socket_path);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 { break; }
                // Echo for now
                socket.write_all(&buf[..n]).await.unwrap();
            }
        });
    }
}
```

- [ ] **Step 3: Update `Cargo.toml` to use `src/main.rs`**

- [ ] **Step 4: Verify daemon starts**

Run: `cargo run -p narya-daemon`
Expected: Prints "Daemon listening on /tmp/narya.sock"

- [ ] **Step 5: Commit**

```bash
git add crates/narya-daemon/src/main.rs crates/narya-daemon/Cargo.toml
git commit -m "feat(daemon): implement basic UDS listener skeleton"
```

---

### Task 3: Implement IPC Client in `narya-app`

**Files:**
- Create: `crates/narya-app/src/ipc.rs`
- Modify: `crates/narya-app/src/lib.rs`
- Modify: `crates/narya-app/src/state.rs`

- [ ] **Step 1: Implement `IpcClient` helper**

```rust
// crates/narya-app/src/ipc.rs
use tokio::net::UnixStream;
use narya_ipc::{IpcRequest, IpcResponse};

pub struct IpcClient {
    stream: UnixStream,
}

impl IpcClient {
    pub async fn connect() -> anyhow::Result<Self> {
        let stream = UnixStream::connect("/tmp/narya.sock").await?;
        Ok(Self { stream })
    }
}
```

- [ ] **Step 2: Integrate `IpcClient` into `AppState`**

Update `AppState::start_traffic_monitor` to try connecting to the daemon and reading real data if possible, or fallback to mock if connection fails.

- [ ] **Step 3: Verify App compiles and can connect to Daemon**

- [ ] **Step 4: Commit**

```bash
git add crates/narya-app/src/ipc.rs crates/narya-app/src/state.rs
git commit -m "feat(app): add IPC client logic to AppState"
```

---

### Task 4: Final Memory Update & Milestone Handoff

**Files:**
- Modify: `.memory/status.md`
- Modify: `.memory/tasks.md`

- [ ] **Step 1: Mark Phase 6 tasks as completed**
- [ ] **Step 2: Update project status**
- [ ] **Step 3: Commit and push**
