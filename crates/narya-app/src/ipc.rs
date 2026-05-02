use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use narya_ipc::{IpcRequest, IpcResponse};
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
}
