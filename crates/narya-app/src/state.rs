use narya_core::{Node, NodeDetails, Subscription};

pub struct AppState {
    pub nodes: Vec<Node>,
    pub subscriptions: Vec<Subscription>,
    pub active_node_id: Option<String>,
}

impl AppState {
    pub fn mock_data() -> Self {
        let nodes = vec![
            Node {
                id: "hk-01".to_string(),
                name: "香港 HK 01".to_string(),
                country_code: "HK".to_string(),
                protocol: "Shadowsocks".to_string(),
                tag: Some("推荐".to_string()),
                latency: Some(48),
                usage_pct: 23,
                download_speed: 12.45,
                upload_speed: 3.26,
                details: NodeDetails {
                    address: "hkg01.narya.net:443".to_string(),
                    encryption: "2022-blake3-aes-128-gcm".to_string(),
                    udp: true,
                    tls: false,
                    skip_cert_verify: false,
                    transport: "tcp".to_string(),
                    last_test: "刚刚".to_string(),
                },
            },
            Node {
                id: "sg-01".to_string(),
                name: "新加坡 SG 01".to_string(),
                country_code: "SG".to_string(),
                protocol: "Hysteria2".to_string(),
                tag: None,
                latency: Some(55),
                usage_pct: 20,
                download_speed: 11.2,
                upload_speed: 4.1,
                details: NodeDetails {
                    address: "sgp01.narya.net:443".to_string(),
                    encryption: "none".to_string(),
                    udp: true,
                    tls: true,
                    skip_cert_verify: true,
                    transport: "udp".to_string(),
                    last_test: "1 min ago".to_string(),
                },
            },
            Node {
                id: "jp-01".to_string(),
                name: "日本 JP 01".to_string(),
                country_code: "JP".to_string(),
                protocol: "Vmess".to_string(),
                tag: None,
                latency: Some(62),
                usage_pct: 18,
                download_speed: 9.8,
                upload_speed: 3.2,
                details: NodeDetails {
                    address: "tyo01.narya.net:443".to_string(),
                    encryption: "auto".to_string(),
                    udp: true,
                    tls: true,
                    skip_cert_verify: false,
                    transport: "tcp".to_string(),
                    last_test: "5 mins ago".to_string(),
                },
            },
            Node {
                id: "us-01".to_string(),
                name: "美国 US 01".to_string(),
                country_code: "US".to_string(),
                protocol: "VLESS Reality".to_string(),
                tag: None,
                latency: Some(110),
                usage_pct: 32,
                download_speed: 8.7,
                upload_speed: 2.9,
                details: NodeDetails {
                    address: "lax01.narya.net:443".to_string(),
                    encryption: "none".to_string(),
                    udp: true,
                    tls: true,
                    skip_cert_verify: false,
                    transport: "grpc".to_string(),
                    last_test: "10 mins ago".to_string(),
                },
            },
        ];

        let subscriptions = vec![
            Subscription {
                id: "sub-1".to_string(),
                name: "机场 A".to_string(),
                url: "https://example.com/sub1".to_string(),
                icon: "plane".to_string(),
                node_count: 128,
                used_nodes: 38,
                update_time: "刚刚".to_string(),
                traffic_used: 436.0,
                traffic_total: 1280.0,
                expiration: "2026-06-10".to_string(),
                status: "当前使用".to_string(),
            },
            Subscription {
                id: "sub-2".to_string(),
                name: "Work Proxy".to_string(),
                url: "https://example.com/work".to_string(),
                icon: "briefcase".to_string(),
                node_count: 42,
                used_nodes: 2,
                update_time: "2 hours ago".to_string(),
                traffic_used: 15.0,
                traffic_total: 100.0,
                expiration: "2026-08-15".to_string(),
                status: "运行中".to_string(),
            },
            Subscription {
                id: "sub-3".to_string(),
                name: "Global Backup".to_string(),
                url: "https://example.com/backup".to_string(),
                icon: "globe".to_string(),
                node_count: 86,
                used_nodes: 1,
                update_time: "1 day ago".to_string(),
                traffic_used: 670.0,
                traffic_total: 1000.0,
                expiration: "2026-12-31".to_string(),
                status: "运行中".to_string(),
            },
        ];

        Self {
            active_node_id: Some("hk-01".to_string()),
            nodes,
            subscriptions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_mock() {
        let state = AppState::mock_data();
        assert!(!state.nodes.is_empty());
        assert!(!state.subscriptions.is_empty());
        assert!(state.active_node_id.is_some());
    }
}
