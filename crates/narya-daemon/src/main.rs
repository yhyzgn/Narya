mod config_gen;
mod kernel;
mod proxy;

use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use std::fs;
use narya_ipc::{IpcRequest, IpcResponse};
use crate::kernel::KernelManager;
use crate::proxy::{SystemProxy, LinuxGSettings, MacOSNetworkSetup, ProxyBackend};
use std::sync::Arc;
use tokio::sync::Mutex;

struct DaemonState {
    kernel: KernelManager,
    proxy: ProxyBackend,
}

#[tokio::main]
async fn main() -> Result<()> {
    let socket_path = "/tmp/narya.sock";
    
    // Cleanup old socket if exists
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }
    
    let listener = UnixListener::bind(socket_path)?;
    println!("Daemon listening on {}", socket_path);

    // Detect OS and choose proxy backend
    let proxy = if cfg!(target_os = "macos") {
        ProxyBackend::MacOS(MacOSNetworkSetup)
    } else {
        ProxyBackend::Linux(LinuxGSettings)
    };

    let state = Arc::new(Mutex::new(DaemonState {
        kernel: KernelManager::new(),
        proxy,
    }));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let state = Arc::clone(&state);
        
        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(request) = serde_json::from_slice::<IpcRequest>(&buf[..n]) {
                            let mut state = state.lock().await;
                            let response = match request.method.as_str() {
                                "SetSystemProxy" => {
                                    let enabled = request.params.as_bool().unwrap_or(false);
                                    match state.proxy.set_enabled(enabled).await {
                                        Ok(_) => IpcResponse { id: request.id, result: Some(serde_json::json!(true)), error: None },
                                        Err(e) => IpcResponse { id: request.id, result: None, error: Some(e.to_string()) },
                                    }
                                },
                                "StartKernel" => {
                                    // Placeholder for binary and config paths
                                    match state.kernel.start("sing-box", "/tmp/config.json").await {
                                        Ok(_) => IpcResponse { id: request.id, result: Some(serde_json::json!(true)), error: None },
                                        Err(e) => IpcResponse { id: request.id, result: None, error: Some(e.to_string()) },
                                    }
                                },
                                "StopKernel" => {
                                    match state.kernel.stop().await {
                                        Ok(_) => IpcResponse { id: request.id, result: Some(serde_json::json!(true)), error: None },
                                        Err(e) => IpcResponse { id: request.id, result: None, error: Some(e.to_string()) },
                                    }
                                },
                                _ => IpcResponse { id: request.id, result: None, error: Some("Unknown method".to_string()) },
                            };
                            
                            let res_json = serde_json::to_vec(&response).unwrap();
                            let _ = socket.write_all(&res_json).await;
                        }
                    },
                    Err(_) => break,
                }
            }
        });
    }
}
