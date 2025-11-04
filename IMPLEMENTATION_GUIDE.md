# WireGuard Integration Implementation Guide

## Quick Start

This guide shows you how to integrate the real WireGuard functionality into VIPN.

## Architecture Overview

We've created a **platform abstraction layer** that:
- Uses a trait (`WireGuardPlatform`) to abstract platform differences
- Implements `wg-quick` based solution for cross-platform compatibility
- Can be extended with native APIs later

## Current Implementation Status

✅ **Completed:**
- Platform abstraction trait and module structure
- macOS implementation skeleton
- Linux and Windows placeholder implementations
- Error handling types

⏳ **Next Steps:**
1. Integrate platform abstraction into existing wireguard.rs
2. Replace mock functions with real implementations
3. Add connection state tracking
4. Test on macOS

## Integration Steps

### Step 1: Update wireguard.rs to use platform abstraction

Replace the mock `apply_config` and `disconnect` functions with real implementations:

```rust
// In wireguard.rs, add at module level:
use platform::{create_platform, WireGuardPlatform};

// Add connection state tracking
static mut ACTIVE_CONNECTION: Option<(String, String)> = None; // (interface, config_name)

// Replace apply_config function:
pub async fn apply_config(config: WireGuardConfig) -> Result<String, String> {
    let platform = create_platform()
        .map_err(|e| e.to_string())?;

    if !platform.is_wireguard_installed() {
        return Err("WireGuard is not installed. Please install WireGuard tools.".to_string());
    }

    let interface = platform.apply_config(&config)
        .map_err(|e| e.to_string())?;

    // Track active connection
    unsafe {
        ACTIVE_CONNECTION = Some((interface.clone(), config.name.clone()));
    }

    Ok(format!("Connected via {}", interface))
}

// Replace disconnect function:
pub async fn disconnect() -> Result<String, String> {
    let platform = create_platform()
        .map_err(|e| e.to_string())?;

    // Get active interface
    let interface = unsafe {
        ACTIVE_CONNECTION.as_ref()
            .map(|(iface, _)| iface.clone())
    };

    if let Some(iface) = interface {
        platform.disconnect(&iface)
            .map_err(|e| e.to_string())?;

        unsafe {
            ACTIVE_CONNECTION = None;
        }

        Ok("Disconnected successfully".to_string())
    } else {
        Err("No active connection".to_string())
    }
}

// Replace get_connection_status:
pub fn get_connection_status() -> ConnectionStatus {
    let platform = match create_platform() {
        Ok(p) => p,
        Err(_) => return ConnectionStatus {
            connected: false,
            current_config: None,
            interface: None,
        },
    };

    let interface = unsafe {
        ACTIVE_CONNECTION.as_ref()
            .map(|(iface, _)| iface.as_str())
    };

    match platform.get_status(interface) {
        Ok(status) => status,
        Err(_) => ConnectionStatus {
            connected: false,
            current_config: None,
            interface: None,
        },
    }
}
```

### Step 2: Handle Permissions

On macOS, `wg-quick` may require sudo. You have a few options:

**Option A: Request sudo when needed**
```rust
// In macos.rs, check if command failed with permission error
// Then prompt user for password using osascript
```

**Option B: Use setuid wrapper (advanced)**
- Create a helper binary with setuid
- Requires code signing and proper permissions

**Option C: Guide user to run with sudo (simplest for MVP)**
- Show clear error message
- Guide user to install WireGuard properly

### Step 3: Test on macOS

1. **Install WireGuard:**
   ```bash
   brew install wireguard-tools
   ```

2. **Test manually first:**
   ```bash
   wg-quick up /path/to/config.conf
   wg show
   wg-quick down wg0
   ```

3. **Test in app:**
   - Run `npm run tauri dev`
   - Try connecting with a real WireGuard config
   - Check console for errors

### Step 4: Improve Error Messages

Update error handling to provide user-friendly messages:

```rust
impl From<WireGuardError> for String {
    fn from(err: WireGuardError) -> Self {
        match err {
            WireGuardError::NotInstalled => {
                "WireGuard is not installed. Please install WireGuard tools:\n\n\
                macOS: brew install wireguard-tools\n\
                Linux: See your distribution's package manager"
            },
            WireGuardError::PermissionDenied => {
                "Permission denied. WireGuard requires elevated privileges.\n\
                Please ensure you have the necessary permissions."
            },
            // ... etc
        }
    }
}
```

## Known Issues & Solutions

### Issue 1: Config File Management
**Problem:** wg-quick needs config file to stay for disconnect

**Solution:** Store config file path with connection state:
```rust
struct ActiveConnection {
    interface: String,
    config_name: String,
    config_path: PathBuf,
}
```

### Issue 2: Interface Name Detection
**Problem:** Hardcoded "wg0" won't work for multiple connections

**Solution:** Parse interface name from wg-quick output or use config file name

### Issue 3: Permission Requirements
**Problem:** Need sudo on first run

**Solution:**
- Check permissions first
- Guide user to setup properly
- Consider using helper daemon (future)

## Testing Strategy

1. **Unit Tests:** Mock command execution
2. **Integration Tests:** Test with real wg-quick (if available)
3. **Manual Testing:** Test on real macOS system

## Migration Checklist

- [ ] Replace mock `apply_config` with real implementation
- [ ] Replace mock `disconnect` with real implementation
- [ ] Replace mock `get_connection_status` with real implementation
- [ ] Add connection state tracking
- [ ] Handle config file lifecycle
- [ ] Improve error messages
- [ ] Test on macOS
- [ ] Update tests
- [ ] Document macOS-specific requirements

## Next Platform: Linux

Once macOS is working, Linux implementation should be straightforward:
- Similar wg-quick approach
- Different paths (`/usr/bin/wg-quick`)
- Different permission model (may need sudo)
- Different interface naming (wg0, wg1, etc.)

## Future Enhancements

1. **Native APIs:** Use platform-specific APIs for better performance
2. **Connection Monitoring:** Track latency, packet loss
3. **Auto-Reconnect:** Handle connection drops automatically
4. **Multiple Connections:** Support multiple simultaneous tunnels
5. **Config Persistence:** Save/load configs for quick switching

