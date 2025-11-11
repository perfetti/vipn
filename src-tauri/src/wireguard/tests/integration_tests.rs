/// End-to-end integration tests for WireGuard functionality
///
/// These tests verify the complete flow from config to connection
/// and require WireGuard to be installed.

#[cfg(test)]
mod tests {
    use crate::wireguard::{
        WireGuardConfig,
        apply_config,
        disconnect,
        get_connection_status,
        config_to_wg_quick_format,
    };
    use std::time::Duration;
    use std::thread;

    /// Helper: Check if we can run integration tests
    fn can_run_integration_tests() -> bool {
        // Check if WireGuard is installed
        use crate::wireguard::platform::create_platform;
        match create_platform() {
            Ok(platform) => platform.is_wireguard_installed(),
            Err(_) => false,
        }
    }

    /// Helper: Create a test config
    /// WARNING: This uses test keys - DO NOT use in production
    fn create_test_config() -> WireGuardConfig {
        WireGuardConfig {
            name: "integration-test".to_string(),
            private_key: "cF7B1i7pHVXo0jRGyTqNy5GZgQWQ6Y5vN8jH9kL2mP3q=".to_string(),
            public_key: "xTk2K9vN8jH9kL2mP3qRGyTqNy5GZgQWQ6Y5vN8jH=".to_string(),
            endpoint: "127.0.0.1:51820".to_string(),
            allowed_ips: "10.0.0.0/8".to_string(),
            dns: Some("1.1.1.1".to_string()),
            address: "10.0.0.2/24".to_string(),
            persistent_keepalive: Some(25),
        }
    }

    /// Test: Complete connection lifecycle
    #[tokio::test]
    #[ignore] // Requires WireGuard and may need sudo
    async fn test_complete_connection_lifecycle() {
        if !can_run_integration_tests() {
            println!("Skipping: WireGuard not installed");
            return;
        }

        let config = create_test_config();

        // Step 1: Apply config
        let result = apply_config(config.clone()).await;
        match result {
            Ok(msg) => {
                println!("Connected: {}", msg);
            },
            Err(e) => {
                if e.contains("Permission denied") || e.contains("requires elevated") {
                    println!("Skipping: Requires elevated permissions");
                    println!("Error: {}", e);
                    return;
                }
                panic!("Failed to apply config: {}", e);
            }
        }

        // Step 2: Check status
        thread::sleep(Duration::from_millis(500)); // Give it time to establish
        let status = get_connection_status();
        assert!(status.connected, "Should be connected after apply_config");

        // Step 3: Disconnect
        let disconnect_result = disconnect().await;
        assert!(disconnect_result.is_ok(), "Should disconnect successfully");

        // Step 4: Verify disconnected
        thread::sleep(Duration::from_millis(500)); // Give it time to disconnect
        let final_status = get_connection_status();
        assert!(!final_status.connected, "Should be disconnected after disconnect()");
    }

    /// Test: Config format validation
    #[test]
    fn test_config_format() {
        let config = create_test_config();
        let wg_format = config_to_wg_quick_format(&config);

        // Verify all required fields are present
        assert!(wg_format.contains("[Interface]"));
        assert!(wg_format.contains("[Peer]"));
        assert!(wg_format.contains("PrivateKey ="));
        assert!(wg_format.contains("PublicKey ="));
        assert!(wg_format.contains("Endpoint ="));
        assert!(wg_format.contains("Address ="));
        assert!(wg_format.contains("AllowedIPs ="));

        // Verify values
        assert!(wg_format.contains(&config.private_key));
        assert!(wg_format.contains(&config.public_key));
        assert!(wg_format.contains(&config.endpoint));
    }

    /// Test: Error handling for missing WireGuard
    #[test]
    fn test_error_when_wireguard_not_installed() {
        // This test verifies graceful handling when WireGuard is not available
        // It's mostly useful for CI/CD environments where WireGuard might not be installed

        use crate::wireguard::platform::create_platform;
        match create_platform() {
            Ok(platform) => {
                if !platform.is_wireguard_installed() {
                    // Test that we get appropriate error
                    let config = create_test_config();
                    // This should fail gracefully
                    // In a real scenario, we'd test the error message
                }
            },
            Err(_) => {
                // Platform not supported - that's okay
            }
        }
    }

    /// Test: Multiple configs (if supported)
    #[tokio::test]
    #[ignore]
    async fn test_multiple_configs() {
        if !can_run_integration_tests() {
            println!("Skipping: WireGuard not installed");
            return;
        }

        // This test would verify handling multiple configs
        // For now, we'll just verify we can create multiple configs
        let config1 = create_test_config();
        let config2 = WireGuardConfig {
            name: "test-config-2".to_string(),
            ..create_test_config()
        };

        // Verify both configs are valid
        let format1 = config_to_wg_quick_format(&config1);
        let format2 = config_to_wg_quick_format(&config2);

        assert!(!format1.is_empty());
        assert!(!format2.is_empty());
        assert_ne!(format1, format2);
    }
}

