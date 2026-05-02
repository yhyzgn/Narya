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
        println!("Starting kernel: {} with config: {}", binary_path, config_path);
        
        let child = Command::new(binary_path)
            .args(["run", "-c", config_path])
            .spawn()?;
            
        self.child = Some(child);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            println!("Stopping kernel process...");
            child.kill().await?;
            // Wait for it to exit to avoid zombies
            let _ = child.wait().await;
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }
}
