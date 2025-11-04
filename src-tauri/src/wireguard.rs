use serde::{Deserialize, Serialize};

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

