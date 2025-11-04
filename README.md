# VIPN - WireGuard VPN Tauri App

A Tauri application for managing WireGuard VPN connections on macOS.

## Features

- Mock WireGuard configuration loading
- Connect/Disconnect VPN controls
- Status monitoring
- Modern React + TypeScript frontend

## Development

### Prerequisites

- Node.js and npm
- Rust and Cargo
- Tauri CLI: `npm install -g @tauri-apps/cli` or `cargo install tauri-cli`

### Setup

1. Install dependencies:
```bash
npm install
```

2. Run the development server:
```bash
npm run tauri dev
```

### Building

To build the app:
```bash
npm run tauri build
```

## Current Status

- ✅ Tauri app structure set up
- ✅ Mock `getConfig` function implemented
- ✅ Basic UI for viewing configuration and controlling VPN
- ⏳ Real WireGuard integration (TODO)

## Notes

The `getConfig` function is currently mocked and returns a fake configuration. The actual WireGuard integration will be implemented next.

