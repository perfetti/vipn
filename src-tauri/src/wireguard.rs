use serde::{Deserialize, Serialize};

// Platform-specific implementations
pub mod platform;

/// WireGuard configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardConfig {
    pub name: String,
    pub private_key: String,
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: String,
    pub dns: Option<String>,
    pub address: String,
    pub persistent_keepalive: Option<u16>,
}

/// Server config item (simplified for list view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfigItem {
    pub id: String,
    pub name: String,
    pub location: String,
    pub endpoint: String,
}

/// Full server config response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfigResponse {
    pub configs: Vec<ServerConfigItem>,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub connected: bool,
    pub current_config: Option<String>, // config name or ID
    pub interface: Option<String>,
}

/// Mock function to get a fake WireGuard config
/// This will be replaced with actual WireGuard library calls later
pub fn get_config() -> WireGuardConfig {
    WireGuardConfig {
        name: "Default Config".to_string(),
        private_key: "cF7B1i7pHVXo0jRGyTqNy5GZgQWQ6Y5vN8jH9kL2mP3q=".to_string(),
        public_key: "xTk2K9vN8jH9kL2mP3qRGyTqNy5GZgQWQ6Y5vN8jH=".to_string(),
        endpoint: "vpn.example.com:51820".to_string(),
        allowed_ips: "0.0.0.0/0".to_string(),
        dns: Some("1.1.1.1, 8.8.8.8".to_string()),
        address: "10.0.0.2/24".to_string(),
        persistent_keepalive: Some(25),
    }
}

/// Mock function to get a config by ID from server
/// This simulates fetching a specific config from the server
pub fn get_config_by_id(id: &str) -> Option<WireGuardConfig> {
    // Mock different configs based on ID
    match id {
        "us-east-1" => Some(WireGuardConfig {
            name: "US East Server".to_string(),
            private_key: "eF8C2j8qIYWYp1kSHzUrO6HahRXR7Z6wO9kI0lM3nQ4r=".to_string(),
            public_key: "yUl3L0wOxO9kI0lM3nQ4rSHzUrO6HahRXR7Z6wO9kI=".to_string(),
            endpoint: "us-east.vpn.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("1.1.1.1".to_string()),
            address: "10.0.0.3/24".to_string(),
            persistent_keepalive: Some(25),
        }),
        "us-west-1" => Some(WireGuardConfig {
            name: "US West Server".to_string(),
            private_key: "fG9D3k9rJZXZq2lTIaVsP7IbiSY8a7xP0lJ1mN4oR5s=".to_string(),
            public_key: "zVm4M1xPyP0lJ1mN4oR5sTIaVsP7IbiSY8a7xP0lJ=".to_string(),
            endpoint: "us-west.vpn.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("8.8.8.8".to_string()),
            address: "10.0.0.4/24".to_string(),
            persistent_keepalive: Some(25),
        }),
        "eu-central-1" => Some(WireGuardConfig {
            name: "EU Central Server".to_string(),
            private_key: "gH0E4l0sKaYar3mUJbWtQ8JcjTZ9b8yQ1mK2nO5pS6t=".to_string(),
            public_key: "aWn5N2yQzQ1mK2nO5pS6tUJbWtQ8JcjTZ9b8yQ1mK=".to_string(),
            endpoint: "eu-central.vpn.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("1.1.1.1, 1.0.0.1".to_string()),
            address: "10.0.0.5/24".to_string(),
            persistent_keepalive: Some(25),
        }),
        "asia-pacific-1" => Some(WireGuardConfig {
            name: "Asia Pacific Server".to_string(),
            private_key: "hI1F5m1tLbZbs4nVKcXuR9KdkUa0c9zR2nL3oP6qT7u=".to_string(),
            public_key: "bXo6O3zR0R2nL3oP6qT7uVKcXuR9KdkUa0c9zR2nL=".to_string(),
            endpoint: "asia-pac.vpn.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("8.8.8.8, 8.8.4.4".to_string()),
            address: "10.0.0.6/24".to_string(),
            persistent_keepalive: Some(25),
        }),
        _ => None,
    }
}

/// Mock function to fetch list of available configs from server
/// This simulates an API call that returns a list of available VPN servers
pub async fn fetch_config_list_from_server() -> ServerConfigResponse {
    // Simulate network delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    ServerConfigResponse {
        configs: vec![
            ServerConfigItem {
                id: "us-east-1".to_string(),
                name: "US East Server".to_string(),
                location: "New York, USA".to_string(),
                endpoint: "us-east.vpn.example.com:51820".to_string(),
            },
            ServerConfigItem {
                id: "us-west-1".to_string(),
                name: "US West Server".to_string(),
                location: "San Francisco, USA".to_string(),
                endpoint: "us-west.vpn.example.com:51820".to_string(),
            },
            ServerConfigItem {
                id: "eu-central-1".to_string(),
                name: "EU Central Server".to_string(),
                location: "Frankfurt, Germany".to_string(),
                endpoint: "eu-central.vpn.example.com:51820".to_string(),
            },
            ServerConfigItem {
                id: "asia-pacific-1".to_string(),
                name: "Asia Pacific Server".to_string(),
                location: "Tokyo, Japan".to_string(),
                endpoint: "asia-pac.vpn.example.com:51820".to_string(),
            },
        ],
    }
}

/// Mock function to get current connection status
/// This will be replaced with actual WireGuard status checks later
pub fn get_connection_status() -> ConnectionStatus {
    ConnectionStatus {
        connected: false,
        current_config: None,
        interface: None,
    }
}

/// Mock function to apply a WireGuard config
/// This is a placeholder that will later interface with wg-quick or WireGuard system calls
/// Returns Ok(()) on success, Err(String) on failure
pub async fn apply_config(config: WireGuardConfig) -> Result<String, String> {
    // Simulate applying config delay
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // Mock: For now, just return success
    // TODO: Integrate with actual WireGuard on macOS
    // This will likely use:
    // - wg-quick up/down commands
    // - Or direct WireGuard API calls
    // - Network permission handling on macOS

    Ok(format!("Config '{}' applied successfully (mock)", config.name))
}

/// Mock function to disconnect from VPN
pub async fn disconnect() -> Result<String, String> {
    // Simulate disconnection delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // TODO: Implement actual WireGuard disconnection
    // This will likely use: wg-quick down <interface>

    Ok("Disconnected successfully (mock)".to_string())
}

/// Convert WireGuardConfig to wg-quick format string
/// This is the standard format that wg-quick expects
pub fn config_to_wg_quick_format(config: &WireGuardConfig) -> String {
    let mut wg_config = String::new();

    wg_config.push_str("[Interface]\n");
    wg_config.push_str(&format!("PrivateKey = {}\n", config.private_key));
    wg_config.push_str(&format!("Address = {}\n", config.address));

    if let Some(dns) = &config.dns {
        wg_config.push_str(&format!("DNS = {}\n", dns));
    }

    wg_config.push_str("\n[Peer]\n");
    wg_config.push_str(&format!("PublicKey = {}\n", config.public_key));
    wg_config.push_str(&format!("Endpoint = {}\n", config.endpoint));
    wg_config.push_str(&format!("AllowedIPs = {}\n", config.allowed_ips));

    if let Some(keepalive) = config.persistent_keepalive {
        wg_config.push_str(&format!("PersistentKeepalive = {}\n", keepalive));
    }

    wg_config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let config = get_config();
        assert_eq!(config.name, "Default Config");
        assert!(!config.private_key.is_empty());
        assert!(!config.public_key.is_empty());
        assert!(!config.endpoint.is_empty());
        assert!(!config.address.is_empty());
    }

    #[test]
    fn test_get_config_by_id() {
        // Test existing configs
        let us_east = get_config_by_id("us-east-1");
        assert!(us_east.is_some());
        assert_eq!(us_east.unwrap().name, "US East Server");

        let us_west = get_config_by_id("us-west-1");
        assert!(us_west.is_some());
        assert_eq!(us_west.unwrap().name, "US West Server");

        let eu_central = get_config_by_id("eu-central-1");
        assert!(eu_central.is_some());
        assert_eq!(eu_central.unwrap().name, "EU Central Server");

        let asia_pacific = get_config_by_id("asia-pacific-1");
        assert!(asia_pacific.is_some());
        assert_eq!(asia_pacific.unwrap().name, "Asia Pacific Server");

        // Test non-existent config
        let invalid = get_config_by_id("invalid-id");
        assert!(invalid.is_none());
    }

    #[tokio::test]
    async fn test_fetch_config_list_from_server() {
        let response = fetch_config_list_from_server().await;
        assert_eq!(response.configs.len(), 4);

        assert_eq!(response.configs[0].id, "us-east-1");
        assert_eq!(response.configs[0].name, "US East Server");
        assert_eq!(response.configs[1].id, "us-west-1");
        assert_eq!(response.configs[2].id, "eu-central-1");
        assert_eq!(response.configs[3].id, "asia-pacific-1");
    }

    #[test]
    fn test_get_connection_status() {
        let status = get_connection_status();
        assert_eq!(status.connected, false);
        assert!(status.current_config.is_none());
        assert!(status.interface.is_none());
    }

    #[tokio::test]
    async fn test_apply_config() {
        let config = WireGuardConfig {
            name: "Test Config".to_string(),
            private_key: "test-private-key".to_string(),
            public_key: "test-public-key".to_string(),
            endpoint: "test.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("1.1.1.1".to_string()),
            address: "10.0.0.1/24".to_string(),
            persistent_keepalive: Some(25),
        };

        let result = apply_config(config).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("applied successfully"));
    }

    #[tokio::test]
    async fn test_disconnect() {
        let result = disconnect().await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Disconnected"));
    }

    #[test]
    fn test_config_to_wg_quick_format() {
        let config = WireGuardConfig {
            name: "Test Config".to_string(),
            private_key: "test-private-key".to_string(),
            public_key: "test-public-key".to_string(),
            endpoint: "test.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("1.1.1.1".to_string()),
            address: "10.0.0.1/24".to_string(),
            persistent_keepalive: Some(25),
        };

        let wg_format = config_to_wg_quick_format(&config);

        assert!(wg_format.contains("[Interface]"));
        assert!(wg_format.contains("PrivateKey = test-private-key"));
        assert!(wg_format.contains("Address = 10.0.0.1/24"));
        assert!(wg_format.contains("DNS = 1.1.1.1"));
        assert!(wg_format.contains("[Peer]"));
        assert!(wg_format.contains("PublicKey = test-public-key"));
        assert!(wg_format.contains("Endpoint = test.example.com:51820"));
        assert!(wg_format.contains("AllowedIPs = 0.0.0.0/0"));
        assert!(wg_format.contains("PersistentKeepalive = 25"));
    }

    #[test]
    fn test_config_to_wg_quick_format_without_optional_fields() {
        let config = WireGuardConfig {
            name: "Test Config".to_string(),
            private_key: "test-private-key".to_string(),
            public_key: "test-public-key".to_string(),
            endpoint: "test.example.com:51820".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: None,
            address: "10.0.0.1/24".to_string(),
            persistent_keepalive: None,
        };

        let wg_format = config_to_wg_quick_format(&config);

        assert!(wg_format.contains("[Interface]"));
        assert!(wg_format.contains("PrivateKey = test-private-key"));
        assert!(!wg_format.contains("DNS"));
        assert!(wg_format.contains("[Peer]"));
        assert!(!wg_format.contains("PersistentKeepalive"));
    }

    #[test]
    fn test_wireguard_config_serialization() {
        let config = WireGuardConfig {
            name: "Test".to_string(),
            private_key: "key".to_string(),
            public_key: "pub".to_string(),
            endpoint: "endpoint".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: Some("1.1.1.1".to_string()),
            address: "10.0.0.1/24".to_string(),
            persistent_keepalive: Some(25),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: WireGuardConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.private_key, deserialized.private_key);
        assert_eq!(config.public_key, deserialized.public_key);
    }
}

