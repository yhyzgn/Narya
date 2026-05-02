#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub country_code: String,
    pub protocol: String,
    pub tag: Option<String>,
    pub latency: Option<u32>,
    pub usage_pct: u8,
    pub download_speed: f32,
    pub upload_speed: f32,
    pub details: NodeDetails,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeDetails {
    pub address: String,
    pub encryption: String,
    pub udp: bool,
    pub tls: bool,
    pub skip_cert_verify: bool,
    pub transport: String,
    pub last_test: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: String,
    pub node_count: u32,
    pub used_nodes: u32,
    pub update_time: String,
    pub traffic_used: f64,
    pub traffic_total: f64,
    pub expiration: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_instantiation() {
        let details = NodeDetails {
            address: "test.example.com".to_string(),
            encryption: "none".to_string(),
            udp: true,
            tls: false,
            skip_cert_verify: false,
            transport: "tcp".to_string(),
            last_test: "Just now".to_string(),
        };
        let node = Node {
            id: "1".to_string(),
            name: "Test Node".to_string(),
            country_code: "US".to_string(),
            protocol: "Shadowsocks".to_string(),
            tag: None,
            latency: Some(42),
            usage_pct: 0,
            download_speed: 0.0,
            upload_speed: 0.0,
            details,
        };
        assert_eq!(node.name, "Test Node");
    }
}
