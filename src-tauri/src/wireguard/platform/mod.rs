/// Platform-specific WireGuard implementations
///
/// This module provides a trait-based abstraction for WireGuard operations
/// that can be implemented differently on each platform.

use crate::wireguard::{WireGuardConfig, ConnectionStatus};

/// Errors that can occur during WireGuard operations
#[derive(Debug, Clone)]
pub enum WireGuardError {
    NotInstalled,
    PermissionDenied,
    ConfigInvalid(String),
    InterfaceNotFound(String),
    CommandFailed(String),
    NetworkError(String),
    PlatformNotSupported,
}

impl std::fmt::Display for WireGuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WireGuardError::NotInstalled => write!(f, "WireGuard is not installed. Please install WireGuard tools."),
            WireGuardError::PermissionDenied => write!(f, "Permission denied. This operation requires elevated privileges."),
            WireGuardError::ConfigInvalid(msg) => write!(f, "Invalid configuration: {}", msg),
            WireGuardError::InterfaceNotFound(iface) => write!(f, "Interface not found: {}", iface),
            WireGuardError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            WireGuardError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            WireGuardError::PlatformNotSupported => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for WireGuardError {}

/// Trait for platform-specific WireGuard operations
pub trait WireGuardPlatform: Send + Sync {
    /// Apply a WireGuard configuration
    /// Returns the interface name on success
    fn apply_config(&self, config: &WireGuardConfig) -> Result<String, WireGuardError>;

    /// Disconnect from a WireGuard interface
    fn disconnect(&self, interface: &str) -> Result<(), WireGuardError>;

    /// Get the current connection status
    fn get_status(&self, interface: Option<&str>) -> Result<ConnectionStatus, WireGuardError>;

    /// List all active WireGuard interfaces
    fn list_interfaces(&self) -> Result<Vec<String>, WireGuardError>;

    /// Check if WireGuard is installed on this system
    fn is_wireguard_installed(&self) -> bool;

    /// Get the path to wg-quick command
    fn wg_quick_path(&self) -> Option<String>;
}

/// Create a platform-specific WireGuard implementation
pub fn create_platform() -> Result<Box<dyn WireGuardPlatform>, WireGuardError> {
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(crate::wireguard::platform::macos::MacOSPlatform::new()))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(crate::wireguard::platform::linux::LinuxPlatform::new()))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(crate::wireguard::platform::windows::WindowsPlatform::new()))
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err(WireGuardError::PlatformNotSupported)
    }
}

// Platform-specific implementations
#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

