use anyhow::Result;
use tokio::process::Command;

pub trait SystemProxy: Send + Sync {
    async fn set_enabled(&self, enabled: bool) -> Result<()>;
}

pub struct LinuxGSettings;

impl SystemProxy for LinuxGSettings {
    async fn set_enabled(&self, enabled: bool) -> Result<()> {
        let mode = if enabled { "manual" } else { "none" };
        println!("Setting Linux system proxy mode to: {}", mode);
        
        let status = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", mode])
            .status()
            .await?;
            
        if !status.success() {
            anyhow::bail!("Failed to set gsettings proxy mode");
        }
        Ok(())
    }
}
