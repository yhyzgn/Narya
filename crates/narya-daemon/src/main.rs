mod proxy;

use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let socket_path = "/tmp/narya.sock";
    
    // Cleanup old socket if exists
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }
    
    let listener = UnixListener::bind(socket_path)?;
    println!("Daemon listening on {}", socket_path);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("New connection accepted");
        
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Connection closed");
                        break;
                    },
                    Ok(n) => {
                        println!("Received {} bytes", n);
                        // Echo back for testing
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("Failed to write to socket: {}", e);
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
