use narya_core::{Node, NodeDetails};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ClashConfig {
    proxies: Option<Vec<ClashProxy>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClashProxy {
    name: String,
    #[serde(rename = "type")]
    proxy_type: String,
    server: String,
    port: u16,
    cipher: Option<String>,
    password: Option<String>,
    uuid: Option<String>,
    #[serde(default)]
    udp: bool,
    #[serde(default)]
    tls: bool,
}

pub fn parse_clash_yaml(content: &str) -> Result<Vec<Node>> {
    let config: ClashConfig = serde_yaml::from_str(content)?;
    let mut nodes = Vec::new();
    
    if let Some(proxies) = config.proxies {
        for p in proxies {
            let details = NodeDetails {
                address: format!("{}:{}", p.server, p.port),
                encryption: p.cipher.unwrap_or_else(|| p.uuid.unwrap_or_default()),
                udp: p.udp,
                tls: p.tls,
                skip_cert_verify: false,
                transport: "tcp".to_string(),
                last_test: "Never".to_string(),
            };
            
            nodes.push(Node {
                id: uuid::Uuid::new_v4().to_string(),
                name: p.name,
                country_code: "UN".to_string(), // Unknown for now
                protocol: p.proxy_type,
                tag: None,
                latency: None,
                usage_pct: 0,
                download_speed: 0.0,
                upload_speed: 0.0,
                details,
            });
        }
    }
    
    Ok(nodes)
}
