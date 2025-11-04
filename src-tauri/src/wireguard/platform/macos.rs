/// macOS-specific WireGuard implementation
use super::{WireGuardError, WireGuardPlatform};
use crate::wireguard::{WireGuardConfig, ConnectionStatus};
use std::process::Command;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use std::io::Write;
use std::fs;
use crate::wireguard::config_to_wg_quick_format;

pub struct MacOSPlatform;

impl MacOSPlatform {
    pub fn new() -> Self {
        Self
    }

    /// Find wg-quick command on macOS
    fn find_wg_quick(&self) -> Option<String> {
        // Common locations on macOS
        let paths = vec![
            "/usr/local/bin/wg-quick",
            "/opt/homebrew/bin/wg-quick",
            "/usr/bin/wg-quick",
        ];

        // Check if wg-quick is in PATH
        if let Ok(output) = Command::new("which").arg("wg-quick").output() {
            if output.status.success() {
                if let Ok(path) = String::from_utf8(output.stdout) {
                    return Some(path.trim().to_string());
                }
            }
        }

        // Check common paths
        for path in paths {
            if PathBuf::from(&path).exists() {
                return Some(path);
            }
        }

        None
    }

    /// Execute wg-quick command
    fn execute_wg_quick(&self, args: &[&str]) -> Result<String, WireGuardError> {
        let wg_quick = self.find_wg_quick()
            .ok_or(WireGuardError::NotInstalled)?;

        let output = Command::new(&wg_quick)
            .args(args)
            .output()
            .map_err(|e| WireGuardError::CommandFailed(format!("Failed to execute wg-quick: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(WireGuardError::CommandFailed(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Generate a unique interface name
    fn generate_interface_name(&self) -> String {
        // macOS typically uses utun interfaces
        // We'll use a simple naming scheme: wg0, wg1, etc.
        // In a real implementation, we'd check existing interfaces
        "wg0".to_string()
    }

    /// Write config to temporary file
    fn write_config_file(&self, config: &WireGuardConfig, wg_format: &str) -> Result<PathBuf, WireGuardError> {
        // Create a temporary file with .conf extension
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| WireGuardError::ConfigInvalid(format!("Failed to create temp file: {}", e)))?;

        // Write config content
        temp_file.write_all(wg_format.as_bytes())
            .map_err(|e| WireGuardError::ConfigInvalid(format!("Failed to write config: {}", e)))?;

        let path = temp_file.path().to_path_buf();
        temp_file.keep()
            .map_err(|e| WireGuardError::ConfigInvalid(format!("Failed to persist temp file: {}", e)))?;

        Ok(path)
    }
}

impl WireGuardPlatform for MacOSPlatform {
    fn apply_config(&self, config: &WireGuardConfig) -> Result<String, WireGuardError> {
        // Convert config to wg-quick format
        let wg_format = config_to_wg_quick_format(config);

        // Write to temporary file
        let config_path = self.write_config_file(config, &wg_format)?;

        // Generate interface name (simplified - in reality, parse from config or auto-detect)
        let interface = self.generate_interface_name();

        // Execute: wg-quick up <config_file>
        self.execute_wg_quick(&["up", config_path.to_str().unwrap()])?;

        Ok(interface)
    }

    fn disconnect(&self, interface: &str) -> Result<(), WireGuardError> {
        // Find the config file for this interface
        // For now, we'll use a simple approach: wg-quick down <interface>
        // In a real implementation, we'd track which config file was used

        // Try to find config file in common locations
        let config_paths = vec![
            format!("/etc/wireguard/{}.conf", interface),
            format!("{}/.config/wireguard/{}.conf", std::env::var("HOME").unwrap_or_default(), interface),
        ];

        for config_path in config_paths {
            if PathBuf::from(&config_path).exists() {
                self.execute_wg_quick(&["down", &config_path])?;
                return Ok(());
            }
        }

        // If no config file found, try direct interface down
        // Note: This might not work on all systems
        self.execute_wg_quick(&["down", interface])?;
        Ok(())
    }

    fn get_status(&self, interface: Option<&str>) -> Result<ConnectionStatus, WireGuardError> {
        // Use `wg show` to check interface status
        let wg_path = self.find_wg_quick()
            .ok_or(WireGuardError::NotInstalled)?
            .replace("wg-quick", "wg");

        let interfaces = if let Some(iface) = interface {
            vec![iface.to_string()]
        } else {
            self.list_interfaces()?
        };

        if interfaces.is_empty() {
            return Ok(ConnectionStatus {
                connected: false,
                current_config: None,
                interface: None,
            });
        }

        // Check if interface exists and has peers
        let output = Command::new(&wg_path)
            .args(&["show", &interfaces[0], "dump"])
            .output()
            .map_err(|e| WireGuardError::CommandFailed(format!("Failed to check status: {}", e)))?;

        let is_connected = output.status.success() && !output.stdout.is_empty();

        Ok(ConnectionStatus {
            connected: is_connected,
            current_config: if is_connected { Some(interfaces[0].clone()) } else { None },
            interface: if is_connected { Some(interfaces[0].clone()) } else { None },
        })
    }

    fn list_interfaces(&self) -> Result<Vec<String>, WireGuardError> {
        let wg_path = self.find_wg_quick()
            .ok_or(WireGuardError::NotInstalled)?
            .replace("wg-quick", "wg");

        let output = Command::new(&wg_path)
            .arg("show")
            .output()
            .map_err(|e| WireGuardError::CommandFailed(format!("Failed to list interfaces: {}", e)))?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let interfaces: Vec<String> = stdout
            .lines()
            .filter_map(|line| {
                // wg show outputs interface names, one per line
                let name = line.trim();
                if !name.is_empty() {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(interfaces)
    }

    fn is_wireguard_installed(&self) -> bool {
        self.find_wg_quick().is_some()
    }

    fn wg_quick_path(&self) -> Option<String> {
        self.find_wg_quick()
    }
}

