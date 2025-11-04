# Testing Guide for VIPN

## Overview

This project uses two testing frameworks:
- **Frontend**: Vitest + React Testing Library
- **Backend**: Rust's built-in testing framework

## Running Tests

### Frontend Tests

```bash
# Run tests in watch mode
npm run test

# Run tests once
npm run test:run

# Run tests with UI
npm run test:ui

# Run tests with coverage
npm run test:coverage
```

### Backend Tests

```bash
# Run all Rust tests
cd src-tauri
cargo test

# Run tests for a specific module
cargo test wireguard

# Run tests with output
cargo test -- --nocapture
```

## Test Structure

### Frontend Tests

- Location: `src/test/`
- Main test file: `src/test/App.test.tsx`
- Setup file: `src/test/setup.ts`

Tests cover:
- Component rendering
- User interactions
- Tauri command invocations
- Connection status management
- Config loading and selection

### Backend Tests

- Location: `src-tauri/src/wireguard.rs` (test module)
- Tests cover:
  - Config retrieval functions
  - Server config list fetching
  - Connection status
  - Config application
  - wg-quick format conversion
  - Serialization/deserialization

## Writing New Tests

### Frontend Test Example

```typescript
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import App from "../App";

describe("MyComponent", () => {
  it("renders correctly", () => {
    render(<App />);
    expect(screen.getByText("VIPN")).toBeInTheDocument();
  });
});
```

### Backend Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert_eq!(result, expected_value);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = my_async_function().await;
        assert!(result.is_ok());
    }
}
```

## Test Coverage

Coverage reports are generated in the `coverage/` directory when running `npm run test:coverage`.

## CI/CD Integration

Tests should be run before committing:
- Frontend: `npm run test:run`
- Backend: `cargo test`

