# WireGuard Integration Design

## Overview

This document outlines the architecture for integrating real WireGuard functionality into VIPN in a cross-platform, extensible way.

## Design Goals

1. **Cross-Platform**: Work on macOS, Linux, and Windows
2. **Extensible**: Easy to add platform-specific optimizations
3. **Maintainable**: Clear separation of concerns
4. **Secure**: Proper handling of sensitive data (keys)
5. **User-Friendly**: Clear error messages and status feedback

## Architecture

### Approach 1: Platform Abstraction Layer (Recommended)

Create a trait-based architecture that abstracts platform differences:

```
wireguard/
├── platform/
│   ├── mod.rs          # Platform detection and trait definitions
│   ├── macos.rs        # macOS implementation
│   ├── linux.rs        # Linux implementation
│   └── windows.rs      # Windows implementation (future)
├── wg_quick.rs         # wg-quick command wrapper (common implementation)
├── config.rs           # Config management
└── connection.rs       # Connection state management
```

### Implementation Strategy

#### Phase 1: wg-quick Based (Cross-Platform)

**Why wg-quick?**
- Works on all platforms (macOS, Linux, Windows)
- Standard, well-tested tool
- No need for kernel modules or special permissions during development
- Easy to debug (can test manually)

**Implementation:**
1. Write config to temporary file
2. Execute `wg-quick up <config_file>`
3. Parse output for status
4. Track active connections
5. Use `wg-quick down <interface>` to disconnect

**Platform Detection:**
```rust
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;
```

#### Phase 2: Native APIs (Platform-Specific Optimizations)

After Phase 1 is stable, we can add native implementations:

- **macOS**: Use Network Extensions framework (requires entitlements)
- **Linux**: Direct kernel module interface via `/sys/class/net/`
- **Windows**: WireGuard service API

### Key Components

#### 1. Platform Trait

```rust
pub trait WireGuardPlatform {
    fn apply_config(&self, config: &WireGuardConfig) -> Result<String, WireGuardError>;
    fn disconnect(&self, interface: &str) -> Result<String, WireGuardError>;
    fn get_status(&self, interface: Option<&str>) -> Result<ConnectionStatus, WireGuardError>;
    fn list_interfaces(&self) -> Result<Vec<String>, WireGuardError>;
    fn is_wireguard_installed(&self) -> bool;
}
```

#### 2. Config Management

- Store configs in secure temporary files
- Clean up on disconnect
- Validate config format before applying
- Handle DNS resolution (for endpoint)

#### 3. Connection State

- Track active connections (interface name, config name)
- Persist connection state (optional, for app restart)
- Monitor connection health

#### 4. Error Handling

```rust
pub enum WireGuardError {
    NotInstalled,
    PermissionDenied,
    ConfigInvalid(String),
    InterfaceNotFound(String),
    CommandFailed(String),
    NetworkError(String),
    PlatformNotSupported,
}
```

## Implementation Plan

### Step 1: Create Platform Abstraction

1. Create `src-tauri/src/wireguard/platform/mod.rs`
2. Define `WireGuardPlatform` trait
3. Create platform-specific modules

### Step 2: Implement wg-quick Wrapper

1. Create `src-tauri/src/wireguard/wg_quick.rs`
2. Implement config file writing
3. Execute `wg-quick` commands
4. Parse output for status

### Step 3: macOS Implementation

1. Check for `wg-quick` in PATH
2. Handle macOS-specific paths (`/usr/local/bin/wg-quick`)
3. Handle permissions (may need sudo for first run)
4. Generate interface names (utun0, utun1, etc.)

### Step 4: Connection State Management

1. Track active connections
2. Store interface -> config mapping
3. Update status on connect/disconnect

### Step 5: Error Handling & User Feedback

1. Clear error messages
2. Permission handling
3. Status updates

## Dependencies

### Required

```toml
[dependencies]
tokio = { version = "1", features = ["rt", "time", "process", "fs"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tempfile = "3"  # For secure temporary config files
```

### Optional (for future native implementations)

```toml
# macOS Network Extensions (future)
# core-foundation = "0.9"
# security-framework = "0.9"

# Linux (future)
# nix = "0.27"
```

## Security Considerations

1. **Config Files**: Store in secure temp directory, delete after use
2. **Keys**: Never log private keys
3. **Permissions**: Request only necessary permissions
4. **Sandboxing**: Handle macOS app sandbox appropriately

## macOS-Specific Notes

### Installation Check

WireGuard on macOS can be installed via:
- Homebrew: `brew install wireguard-tools`
- Official installer from wireguard.com
- Check: `which wg-quick` or `/usr/local/bin/wg-quick`

### Permissions

- Network configuration requires admin privileges
- First run may prompt for password
- Consider using `osascript` to request elevation if needed

### Interface Naming

- macOS uses `utun0`, `utun1`, etc.
- Can be auto-generated or specified in config

## Linux-Specific Notes

### Installation Check

- Package managers: `apt`, `yum`, `pacman`, etc.
- Check: `which wg-quick` or `/usr/bin/wg-quick`

### Permissions

- Typically requires `sudo` or running as root
- Can use `setcap` to allow non-root execution (advanced)

### Interface Naming

- Usually `wg0`, `wg1`, etc.
- Can be specified in config filename

## Testing Strategy

1. **Unit Tests**: Mock command execution
2. **Integration Tests**: Test with real wg-quick (if available)
3. **Platform Tests**: Test on each target platform
4. **Error Paths**: Test permission denied, missing tools, etc.

## Migration Path

1. Replace mock functions with real implementations
2. Keep same function signatures (maintains API compatibility)
3. Update tests to work with real implementations
4. Add feature flags if needed: `default = ["wireguard-real"]`

## Future Enhancements

1. **Native APIs**: Use platform-specific APIs for better performance
2. **Connection Monitoring**: Ping endpoints, track latency
3. **Auto-Reconnect**: Handle connection drops
4. **Multiple Connections**: Support multiple simultaneous connections
5. **Config Profiles**: Save/load configs for quick switching

## Example Usage

```rust
// Detect platform
let platform = WireGuardPlatform::new()?;

// Check if WireGuard is installed
if !platform.is_wireguard_installed() {
    return Err(WireGuardError::NotInstalled);
}

// Apply config
let result = platform.apply_config(&config).await?;
// Returns: "Connected via utun0"

// Get status
let status = platform.get_status(Some("utun0"))?;

// Disconnect
platform.disconnect("utun0")?;
```

