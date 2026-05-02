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
                id: "sg-01".to_string(),
                name: "SG-01 (Singapore)".to_string(),
                country_code: "SG".to_string(),
                protocol: "Shadowsocks".to_string(),
                tag: Some("推荐".to_string()),
                latency: Some(12),
                usage_pct: 23,
                download_speed: 12.4,
                upload_speed: 4.6,
                details: NodeDetails {
                    address: "sg01.narya.net:443".to_string(),
                    encryption: "2022-blake3-aes-128-gcm".to_string(),
                    udp: true,
                    tls: false,
                    skip_cert_verify: false,
                    transport: "tcp".to_string(),
                    last_test: "刚刚".to_string(),
                },
            },
            Node {
                id: "us-01".to_string(),
                name: "US-West (Oregon)".to_string(),
                country_code: "US".to_string(),
                protocol: "VLESS".to_string(),
                tag: None,
                latency: Some(156),
                usage_pct: 32,
                download_speed: 8.7,
                upload_speed: 2.9,
                details: NodeDetails {
                    address: "us01.narya.net:443".to_string(),
                    encryption: "none".to_string(),
                    udp: true,
                    tls: true,
                    skip_cert_verify: false,
                    transport: "ws".to_string(),
                    last_test: "10 mins ago".to_string(),
                },
            },
        ];

        let subscriptions = vec![
            Subscription {
                id: "sub-1".to_string(),
                name: "Premium Plan".to_string(),
                url: "https://example.com/sub1".to_string(),
                icon: "plane".to_string(),
                node_count: 128,
                used_nodes: 38,
                update_time: "刚刚".to_string(),
                traffic_used: 436.0,
                traffic_total: 1280.0, // 1.28 TB in GB
                expiration: "2026-06-10".to_string(),
                status: "运行中".to_string(),
            },
            Subscription {
                id: "sub-2".to_string(),
                name: "Free Trial".to_string(),
                url: "https://example.com/sub2".to_string(),
                icon: "box".to_string(),
                node_count: 5,
                used_nodes: 5,
                update_time: "2 days ago".to_string(),
                traffic_used: 8.2,
                traffic_total: 10.0,
                expiration: "2026-05-04".to_string(),
                status: "已过期".to_string(),
            },
        ];

        Self {
            active_node_id: Some("sg-01".to_string()),
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
