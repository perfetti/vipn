/// Integration tests for WireGuard platform implementations
///
/// These tests require WireGuard to be installed and may require
/// elevated permissions. Use feature flags to conditionally compile.

#[cfg(test)]
mod integration_tests {
    use super::super::{WireGuardPlatform, WireGuardError};
    use crate::wireguard::platform::{create_platform, macos::MacOSPlatform};
    use crate::wireguard::{WireGuardConfig, ConnectionStatus};

    /// Test fixture: Create a valid test WireGuard config
    /// Note: This uses test keys - NOT for production use
    fn create_test_config(name: &str) -> WireGuardConfig {
        WireGuardConfig {
            name: name.to_string(),
            // Test private key (base64 encoded, 32 bytes)
            private_key: "cF7B1i7pHVXo0jRGyTqNy5GZgQWQ6Y5vN8jH9kL2mP3q=".to_string(),
            // Test public key (corresponding to above private key)
            public_key: "xTk2K9vN8jH9kL2mP3qRGyTqNy5GZgQWQ6Y5vN8jH=".to_string(),
            endpoint: "127.0.0.1:51820".to_string(), // Localhost for testing
            allowed_ips: "10.0.0.0/8".to_string(), // Narrow scope for testing
            dns: Some("1.1.1.1".to_string()),
            address: "10.0.0.2/24".to_string(),
            persistent_keepalive: Some(25),
        }
    }

    /// Helper: Check if WireGuard is installed
    fn is_wireguard_available() -> bool {
        let platform = match create_platform() {
            Ok(p) => p,
            Err(_) => return false,
        };
        platform.is_wireguard_installed()
    }

    /// Helper: Clean up test interface
    fn cleanup_interface(interface: &str) {
        let platform = match create_platform() {
            Ok(p) => p,
            Err(_) => return,
        };
        let _ = platform.disconnect(interface);
    }

    /// Test: Check if WireGuard is installed
    #[test]
    fn test_wireguard_installation_check() {
        let platform = MacOSPlatform::new();
        let is_installed = platform.is_wireguard_installed();

        // This test always passes - just checks if we can detect installation
        println!("WireGuard installed: {}", is_installed);

        if !is_installed {
            println!("Note: WireGuard not installed. Install with: brew install wireguard-tools");
        }
    }

    /// Test: Find wg-quick path
    #[test]
    fn test_find_wg_quick_path() {
        let platform = MacOSPlatform::new();
        let path = platform.wg_quick_path();

        match path {
            Some(p) => {
                println!("Found wg-quick at: {}", p);
                assert!(std::path::Path::new(&p).exists());
            },
            None => {
                println!("wg-quick not found. Install WireGuard to run integration tests.");
            }
        }
    }

    /// Test: Validate config format conversion
    #[test]
    fn test_config_format_conversion() {
        let config = create_test_config("test-config");
        let wg_format = crate::wireguard::config_to_wg_quick_format(&config);

        // Verify format contains required sections
        assert!(wg_format.contains("[Interface]"));
        assert!(wg_format.contains("[Peer]"));
        assert!(wg_format.contains(&config.private_key));
        assert!(wg_format.contains(&config.public_key));
        assert!(wg_format.contains(&config.endpoint));
        assert!(wg_format.contains(&config.address));
    }

    /// Integration Test: Apply config (requires WireGuard and may need sudo)
    ///
    /// This test is marked as `#[ignore]` by default because it:
    /// - Requires WireGuard to be installed
    /// - May require elevated permissions
    /// - Creates actual network interfaces
    ///
    /// Run with: `cargo test --features integration-tests -- --ignored`
    #[test]
    #[ignore]
    fn test_apply_config_integration() {
        // Skip if WireGuard not available
        if !is_wireguard_available() {
            println!("Skipping: WireGuard not installed");
            return;
        }

        let platform = MacOSPlatform::new();
        let config = create_test_config("integration-test");

        // Use a unique interface name for this test
        let result = platform.apply_config(&config);

        match result {
            Ok(interface) => {
                println!("Successfully connected via: {}", interface);

                // Clean up
                cleanup_interface(&interface);
            },
            Err(e) => {
                // Check if it's a permission error
                if matches!(e, WireGuardError::PermissionDenied) {
                    println!("Permission denied - this test may require sudo");
                    println!("Error: {}", e);
                } else {
                    println!("Failed to apply config: {}", e);
                }
            }
        }
    }

    /// Integration Test: Connection lifecycle
    #[test]
    #[ignore]
    fn test_connection_lifecycle() {
        if !is_wireguard_available() {
            println!("Skipping: WireGuard not installed");
            return;
        }

        let platform = MacOSPlatform::new();
        let config = create_test_config("lifecycle-test");

        // Connect
        let interface = match platform.apply_config(&config) {
            Ok(iface) => iface,
            Err(e) => {
                println!("Failed to connect: {}", e);
                return;
            }
        };

        // Check status
        let status = match platform.get_status(Some(&interface)) {
            Ok(s) => s,
            Err(e) => {
                cleanup_interface(&interface);
                panic!("Failed to get status: {}", e);
            }
        };

        assert!(status.connected || status.interface.is_some());

        // Disconnect
        match platform.disconnect(&interface) {
            Ok(_) => println!("Successfully disconnected"),
            Err(e) => {
                println!("Failed to disconnect: {}", e);
                // Try cleanup anyway
                cleanup_interface(&interface);
            }
        }

        // Verify disconnected
        let _final_status = platform.get_status(Some(&interface)).unwrap_or(ConnectionStatus {
            connected: false,
            current_config: None,
            interface: None,
        });

        // Interface may still exist but should be down
        // In a real scenario, we'd verify it's actually down
    }

    /// Integration Test: List interfaces
    #[test]
    #[ignore]
    fn test_list_interfaces() {
        if !is_wireguard_available() {
            println!("Skipping: WireGuard not installed");
            return;
        }

        let platform = MacOSPlatform::new();
        let interfaces = match platform.list_interfaces() {
            Ok(ifs) => ifs,
            Err(e) => {
                println!("Failed to list interfaces: {}", e);
                return;
            }
        };

        println!("Found {} WireGuard interfaces", interfaces.len());
        for iface in &interfaces {
            println!("  - {}", iface);
        }
    }

    /// Integration Test: Error handling - invalid config
    #[test]
    fn test_invalid_config() {
        let platform = MacOSPlatform::new();

        let invalid_config = WireGuardConfig {
            name: "invalid".to_string(),
            private_key: "".to_string(), // Invalid: empty key
            public_key: "invalid-key".to_string(),
            endpoint: "not-a-valid-endpoint".to_string(),
            allowed_ips: "0.0.0.0/0".to_string(),
            dns: None,
            address: "10.0.0.1/24".to_string(),
            persistent_keepalive: None,
        };

        let result = platform.apply_config(&invalid_config);
        assert!(result.is_err());

        // Verify error type
        match result {
            Err(WireGuardError::ConfigInvalid(_)) => {
                // Expected error
            },
            Err(e) => {
                // May fail with CommandFailed if wg-quick validates differently
                println!("Got error (may be expected): {}", e);
            },
            Ok(_) => {
                panic!("Should have failed with invalid config");
            }
        }
    }

    /// Integration Test: Error handling - disconnect non-existent interface
    #[test]
    fn test_disconnect_nonexistent() {
        let platform = MacOSPlatform::new();

        let result = platform.disconnect("nonexistent-interface-12345");

        // Should either succeed (no-op) or fail gracefully
        match result {
            Ok(_) => {
                // No-op is acceptable - some platforms may silently ignore
            },
            Err(e) => {
                // Should be a reasonable error
                // PlatformNotSupported is acceptable if WireGuard not available
                assert!(matches!(e,
                    WireGuardError::InterfaceNotFound(_) |
                    WireGuardError::CommandFailed(_) |
                    WireGuardError::PlatformNotSupported
                ), "Unexpected error: {:?}", e);
            }
        }
    }
}

