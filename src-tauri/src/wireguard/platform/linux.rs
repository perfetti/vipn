/// Linux-specific WireGuard implementation
///
/// This is a placeholder implementation. The structure mirrors macOS
/// but uses Linux-specific paths and commands.

use super::{WireGuardError, WireGuardPlatform};
use crate::wireguard::{WireGuardConfig, ConnectionStatus};
use std::process::Command;

pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }

    fn find_wg_quick(&self) -> Option<String> {
        // Common locations on Linux
        let paths = vec![
            "/usr/bin/wg-quick",
            "/usr/local/bin/wg-quick",
        ];

        // Check PATH
        if let Ok(output) = Command::new("which").arg("wg-quick").output() {
            if output.status.success() {
                if let Ok(path) = String::from_utf8(output.stdout) {
                    return Some(path.trim().to_string());
                }
            }
        }

        // Check common paths
        for path in paths {
            if std::path::PathBuf::from(&path).exists() {
                return Some(path);
            }
        }

        None
    }
}

impl WireGuardPlatform for LinuxPlatform {
    fn apply_config(&self, _config: &WireGuardConfig) -> Result<String, WireGuardError> {
        // TODO: Implement Linux-specific logic
        Err(WireGuardError::PlatformNotSupported)
    }

    fn disconnect(&self, _interface: &str) -> Result<(), WireGuardError> {
        // TODO: Implement Linux-specific logic
        Err(WireGuardError::PlatformNotSupported)
    }

    fn get_status(&self, _interface: Option<&str>) -> Result<ConnectionStatus, WireGuardError> {
        // TODO: Implement Linux-specific logic
        Err(WireGuardError::PlatformNotSupported)
    }

    fn list_interfaces(&self) -> Result<Vec<String>, WireGuardError> {
        // TODO: Implement Linux-specific logic
        Ok(vec![])
    }

    fn is_wireguard_installed(&self) -> bool {
        self.find_wg_quick().is_some()
    }

    fn wg_quick_path(&self) -> Option<String> {
        self.find_wg_quick()
    }
}

