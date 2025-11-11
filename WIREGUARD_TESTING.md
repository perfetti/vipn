# WireGuard Integration Testing Strategy

## Overview

Testing real WireGuard functionality presents unique challenges:
- Requires WireGuard to be installed
- May need elevated permissions
- Creates actual network interfaces
- Must clean up after tests
- Need to avoid conflicts with existing connections

## Testing Strategy

### 1. Unit Tests (Mocked Commands)
**Purpose:** Test logic without requiring WireGuard installation

**Approach:** Mock command execution and verify correct commands are called

### 2. Integration Tests (Real WireGuard)
**Purpose:** Test actual WireGuard functionality end-to-end

**Approach:** Use real WireGuard with test fixtures, proper cleanup

### 3. Feature Flags
**Purpose:** Allow tests to run with or without WireGuard

**Approach:** Use Cargo features to conditionally compile integration tests

## Implementation Plan

### Test Structure

```
src-tauri/src/wireguard/
├── mod.rs
├── platform/
│   ├── mod.rs
│   ├── macos.rs
│   └── tests.rs              # Integration tests
├── tests/
│   ├── integration_tests.rs  # Full integration tests
│   └── test_fixtures.rs       # Test configs and helpers
```

### Test Categories

1. **Platform Detection Tests**
   - Check if WireGuard is installed
   - Find wg-quick path
   - Handle missing installation gracefully

2. **Config Validation Tests**
   - Valid configs
   - Invalid configs (missing keys, bad format)
   - Edge cases (empty DNS, optional fields)

3. **Connection Lifecycle Tests**
   - Connect
   - Check status
   - Disconnect
   - Cleanup

4. **Error Handling Tests**
   - Permission denied
   - Invalid config
   - Interface not found
   - Network errors

## Test Implementation

### Feature Flags

Add to `Cargo.toml`:

```toml
[features]
default = []
integration-tests = []  # Enable integration tests

[dev-dependencies]
# Existing test dependencies
```

### Test Fixtures

Create test WireGuard configs that:
- Use test keys (not real)
- Point to test endpoints (localhost or test servers)
- Are safe to use repeatedly

### Test Isolation

- Each test should use unique interface names
- Clean up interfaces after each test
- Use test-specific config files
- Avoid conflicts with existing connections

## Example Test Code

See `INTEGRATION_TEST_EXAMPLES.md` for detailed examples.

## CI/CD Considerations

### GitHub Actions / CI Setup

1. **Check if WireGuard is available**
   - macOS runners: Install via Homebrew
   - Linux runners: Install via package manager
   - Skip tests if not available (warn but don't fail)

2. **Permission Handling**
   - Some tests may require sudo
   - Use `sudo -n` to check if available
   - Mark tests that need sudo appropriately

3. **Test Isolation**
   - Run tests in separate processes
   - Use unique test interfaces
   - Cleanup in teardown

## Running Tests

```bash
# Unit tests only (no WireGuard required)
cargo test

# Integration tests (requires WireGuard)
cargo test --features integration-tests

# Specific test
cargo test --features integration-tests test_apply_config

# Skip integration tests
cargo test --lib  # Only library tests
```

## Best Practices

1. **Always Clean Up**
   - Use Drop trait for automatic cleanup
   - Test cleanup in finally blocks
   - Verify interfaces are removed

2. **Use Test Fixtures**
   - Predefined test configs
   - Reusable test helpers
   - Consistent test data

3. **Handle Missing Dependencies**
   - Skip tests if WireGuard not installed
   - Provide clear error messages
   - Don't fail build if optional

4. **Test in Isolation**
   - Each test should be independent
   - Use unique identifiers
   - Avoid shared state

5. **Test Error Paths**
   - Invalid configs
   - Missing permissions
   - Network failures
   - Already connected states

