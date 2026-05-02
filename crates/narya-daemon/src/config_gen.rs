use narya_core::Node;
use serde_json::{json, Value};
use anyhow::Result;

pub struct ConfigGenerator;

impl ConfigGenerator {
    pub fn generate_json(node: &Node) -> Result<Value> {
        let outbound = match node.protocol.as_str() {
            "Shadowsocks" => json!({
                "type": "shadowsocks",
                "tag": "proxy",
                "server": node.details.address.split(':').next().unwrap_or(""),
                "server_port": node.details.address.split(':').last().unwrap_or("443").parse::<u16>().unwrap_or(443),
                "method": node.details.encryption,
                "password": "password" // Placeholder
            }),
            _ => json!({
                "type": "direct",
                "tag": "proxy"
            })
        };

        Ok(json!({
            "log": {
                "level": "info",
                "timestamp": true
            },
            "inbounds": [
                {
                    "type": "socks",
                    "tag": "socks-in",
                    "listen": "127.0.0.1",
                    "listen_port": 1080
                },
                {
                    "type": "http",
                    "tag": "http-in",
                    "listen": "127.0.0.1",
                    "listen_port": 2080
                }
            ],
            "outbounds": [
                outbound,
                {
                    "type": "direct",
                    "tag": "direct"
                },
                {
                    "type": "dns",
                    "tag": "dns-out"
                }
            ],
            "route": {
                "rules": [
                    {
                        "protocol": "dns",
                        "outbound": "dns-out"
                    }
                ]
            }
        }))
    }
}
