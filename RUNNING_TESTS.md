# Running WireGuard Integration Tests

## Quick Start

### Unit Tests (No WireGuard Required)

```bash
# Run all unit tests
cargo test

# Run specific test
cargo test test_config_format_conversion

# Run with output
cargo test -- --nocapture
```

### Integration Tests (Requires WireGuard)

Integration tests are marked with `#[ignore]` and require:
1. WireGuard to be installed
2. May require elevated permissions (sudo)

```bash
# Run ignored integration tests
cargo test -- --ignored

# Run with features (if using feature flags)
cargo test --features integration-tests -- --ignored

# Run specific integration test
cargo test -- --ignored test_apply_config_integration
```

## Prerequisites

### macOS

```bash
# Install WireGuard
brew install wireguard-tools

# Verify installation
which wg-quick
wg-quick --version
```

### Linux

```bash
# Ubuntu/Debian
sudo apt-get install wireguard-tools

# Fedora
sudo dnf install wireguard-tools

# Arch
sudo pacman -S wireguard-tools

# Verify installation
which wg-quick
```

## Test Organization

### Unit Tests
- Location: `src-tauri/src/wireguard/platform/tests.rs`
- Purpose: Test logic without requiring WireGuard
- Always run: `cargo test`

### Integration Tests
- Location: `src-tauri/src/wireguard/tests/integration_tests.rs`
- Purpose: Test real WireGuard functionality
- Requires: WireGuard installed
- Run with: `cargo test -- --ignored`

## Test Categories

### 1. Installation Detection
```bash
cargo test test_wireguard_installation_check
cargo test test_find_wg_quick_path
```

### 2. Config Validation
```bash
cargo test test_config_format_conversion
cargo test test_config_format
```

### 3. Error Handling
```bash
cargo test test_invalid_config
cargo test test_disconnect_nonexistent
```

### 4. Integration (Requires WireGuard)
```bash
cargo test -- --ignored test_apply_config_integration
cargo test -- --ignored test_connection_lifecycle
cargo test -- --ignored test_complete_connection_lifecycle
```

## Handling Permission Errors

If tests fail with permission errors:

```bash
# macOS: May need to run with sudo for some operations
# Note: This is not recommended for automated testing
sudo cargo test -- --ignored

# Better: Configure WireGuard to work without sudo
# See WireGuard documentation for your platform
```

## CI/CD Considerations

### GitHub Actions Example

```yaml
- name: Install WireGuard (macOS)
  if: runner.os == 'macOS'
  run: |
    brew install wireguard-tools
    which wg-quick

- name: Install WireGuard (Linux)
  if: runner.os == 'Linux'
  run: |
    sudo apt-get update
    sudo apt-get install -y wireguard-tools

- name: Run unit tests
  run: cargo test

- name: Run integration tests
  run: cargo test -- --ignored
  continue-on-error: true  # Don't fail build if WireGuard not available
```

## Troubleshooting

### Test Fails: "WireGuard not installed"
- Install WireGuard tools for your platform
- Verify with `which wg-quick`

### Test Fails: "Permission denied"
- Some operations may require sudo
- For development, consider running with sudo
- For CI, configure proper permissions or skip these tests

### Test Leaves Interfaces Behind
- Tests should clean up, but if they crash:
  ```bash
  # List interfaces
  wg show

  # Remove interface
  sudo wg-quick down wg0
  ```

### Test Conflicts with Existing Connections
- Use unique test interface names
- Run tests in isolated environment
- Clean up before running tests

## Best Practices

1. **Always Clean Up**
   - Tests should remove interfaces they create
   - Use `Drop` trait or `finally` blocks

2. **Use Test Fixtures**
   - Reusable test configs
   - Consistent test data

3. **Skip When Appropriate**
   - Check if WireGuard is available
   - Skip gracefully if not installed
   - Don't fail build unnecessarily

4. **Test in Isolation**
   - Each test should be independent
   - Use unique identifiers
   - Avoid shared state

