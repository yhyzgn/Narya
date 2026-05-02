use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use narya_ipc::{IpcRequest, IpcResponse, IpcNotification};
use anyhow::Result;

pub struct IpcClient {
    stream: UnixStream,
}

impl IpcClient {
    pub async fn connect(path: &str) -> Result<Self> {
        let stream = UnixStream::connect(path).await?;
        Ok(Self { stream })
    }

    pub async fn send_request(&mut self, request: IpcRequest) -> Result<IpcResponse> {
        let json = serde_json::to_vec(&request)?;
        self.stream.write_all(&json).await?;
        
        let mut buf = [0u8; 4096];
        let n = self.stream.read(&mut buf).await?;
        let response: IpcResponse = serde_json::from_slice(&buf[..n])?;
        Ok(response)
    }

    pub async fn next_notification(&mut self) -> Result<IpcNotification> {
        let mut buf = [0u8; 4096];
        let n = self.stream.read(&mut buf).await?;
        if n == 0 { anyhow::bail!("Connection closed"); }
        // Note: In a real app, we'd need a proper framing codec here (newline or length-prefix)
        // for now we assume one JSON object per read for simplicity.
        let notification: IpcNotification = serde_json::from_slice(&buf[..n])?;
        Ok(notification)
    }
}
