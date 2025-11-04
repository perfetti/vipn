# VIPN - WireGuard VPN Manager

A modern, cross-platform WireGuard VPN client built with Tauri, React, and TypeScript. VIPN provides an intuitive interface for managing WireGuard connections on macOS (with plans to expand to other platforms).

## Features

- 🔒 **WireGuard VPN Management** - Connect and disconnect from WireGuard VPNs
- 📋 **Multiple Config Sources** - Load configurations from a server or paste manually
- 🎯 **Server Selection** - Browse and select from available VPN servers
- 📝 **Manual Config Input** - Paste and apply WireGuard configs directly
- 🔄 **Connection Status** - Real-time connection status indicator
- 💬 **Status Messages** - Clear feedback for all operations
- 🎨 **Modern UI** - Clean, responsive interface with dark mode support

## Current Status

The application is currently in **development** with mock WireGuard functionality. All core features are implemented and tested:

- ✅ Complete UI with connection management
- ✅ Server config fetching and selection
- ✅ Manual config parsing and application
- ✅ Comprehensive test suite (frontend & backend)
- ⏳ Real WireGuard integration (next step)

## Tech Stack

- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Rust + Tauri 2
- **Testing**: Vitest + React Testing Library (frontend), Rust built-in tests (backend)
- **Styling**: CSS with modern features

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites) dependencies
  - For macOS: Xcode Command Line Tools

## Getting Started

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd VIPN
```

2. Install frontend dependencies:
```bash
npm install
```

3. The Rust dependencies will be installed automatically when you build the app.

### Development

Run the app in development mode:
```bash
npm run tauri dev
```

This will:
- Start the Vite dev server for the React frontend
- Build and run the Tauri application
- Enable hot-reload for both frontend and backend changes

### Building

Build the production application:
```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## Testing

### Frontend Tests

```bash
# Run tests in watch mode
npm run test

# Run tests once
npm run test:run

# Run with UI
npm run test:ui

# Generate coverage report
npm run test:coverage
```

### Backend Tests

```bash
cd src-tauri
cargo test

# Run with output
cargo test -- --nocapture
```

See [README_TESTING.md](./README_TESTING.md) for detailed testing documentation.

## Project Structure

```
VIPN/
├── src/                    # Frontend React application
│   ├── App.tsx            # Main application component
│   ├── types/              # Shared TypeScript types
│   │   └── index.ts
│   └── test/               # Frontend tests
│       ├── setup.ts
│       └── App.test.tsx
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Main entry point
│   │   └── wireguard.rs    # WireGuard module (mock currently)
│   └── Cargo.toml
├── public/                  # Static assets
├── package.json
├── vite.config.ts
└── README.md
```

## Architecture

### Frontend (React/TypeScript)

- **App.tsx**: Main application component handling all UI interactions
- **types/index.ts**: Shared TypeScript interfaces for type safety
- Uses Tauri's `invoke` API to communicate with the Rust backend

### Backend (Rust)

- **lib.rs**: Tauri application entry point with command handlers
- **wireguard.rs**: WireGuard operations module (currently mocked)
  - Config management
  - Server config fetching
  - Connection status
  - wg-quick format conversion

## Development Roadmap

- [ ] Integrate real WireGuard functionality (macOS)
- [ ] Add persistent config storage
- [ ] Implement real server API integration
- [ ] Add connection history/logs
- [ ] Expand to Linux and Windows

## Contributing

This project is in active development. Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

[Add your license here]

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
  - [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
  - [TypeScript](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-typescript-next)

## Support

For issues, questions, or contributions, please open an issue on the repository.

---

**Note**: This application currently uses mock WireGuard functionality. Real WireGuard integration is the next development priority.
