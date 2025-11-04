/// Windows-specific WireGuard implementation
///
/// This is a placeholder implementation for future Windows support.

use super::{WireGuardError, WireGuardPlatform};
use crate::wireguard::{WireGuardConfig, ConnectionStatus};

pub struct WindowsPlatform;

impl WindowsPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl WireGuardPlatform for WindowsPlatform {
    fn apply_config(&self, _config: &WireGuardConfig) -> Result<String, WireGuardError> {
        Err(WireGuardError::PlatformNotSupported)
    }

    fn disconnect(&self, _interface: &str) -> Result<(), WireGuardError> {
        Err(WireGuardError::PlatformNotSupported)
    }

    fn get_status(&self, _interface: Option<&str>) -> Result<ConnectionStatus, WireGuardError> {
        Err(WireGuardError::PlatformNotSupported)
    }

    fn list_interfaces(&self) -> Result<Vec<String>, WireGuardError> {
        Ok(vec![])
    }

    fn is_wireguard_installed(&self) -> bool {
        false
    }

    fn wg_quick_path(&self) -> Option<String> {
        None
    }
}

