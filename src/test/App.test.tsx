import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "../App";
import { invoke } from "@tauri-apps/api/core";
import type {
  WireGuardConfig,
  ConnectionStatus,
  ServerConfigResponse,
} from "../types";

// Mock the Tauri invoke function
vi.mock("@tauri-apps/api/core");

describe("App", () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // Default mock for get_connection_status
    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === "get_connection_status") {
        return Promise.resolve<ConnectionStatus>({
          connected: false,
          current_config: null,
          interface: null,
        });
      }
      return Promise.resolve(null);
    });
  });

  it("renders the app title and status", async () => {
    render(<App />);

    expect(screen.getByText("VIPN - WireGuard VPN")).toBeInTheDocument();
    expect(screen.getByText("Disconnected")).toBeInTheDocument();
  });

  it("displays connection status correctly", async () => {
    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === "get_connection_status") {
        return Promise.resolve<ConnectionStatus>({
          connected: true,
          current_config: "Test Config",
          interface: "utun0",
        });
      }
      return Promise.resolve(null);
    });

    render(<App />);

    await waitFor(() => {
      expect(screen.getByText("Connected")).toBeInTheDocument();
    });
  });

  it("shows connect button when disconnected", async () => {
    render(<App />);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: /connect/i })).toBeInTheDocument();
    });
  });

  it("fetches server configs when button is clicked", async () => {
    const user = userEvent.setup();

    const mockConfigs = {
      configs: [
        {
          id: "us-east-1",
          name: "US East Server",
          location: "New York, USA",
          endpoint: "us-east.vpn.example.com:51820",
        },
      ],
    };

    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === "get_connection_status") {
        return Promise.resolve<ConnectionStatus>({
          connected: false,
          current_config: null,
          interface: null,
        });
      }
      if (cmd === "fetch_config_list") {
        return Promise.resolve<ServerConfigResponse>(mockConfigs);
      }
      return Promise.resolve(null);
    });

    render(<App />);

    const fetchButton = screen.getByRole("button", { name: /fetch server configs/i });
    await user.click(fetchButton);

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("fetch_config_list");
      expect(screen.getByText("US East Server")).toBeInTheDocument();
    });
  });

  it("allows selecting a server config from the list", async () => {
    const user = userEvent.setup();

    const mockConfigs: ServerConfigResponse = {
      configs: [
        {
          id: "us-east-1",
          name: "US East Server",
          location: "New York, USA",
          endpoint: "us-east.vpn.example.com:51820",
        },
      ],
    };

    const mockConfig: WireGuardConfig = {
      name: "US East Server",
      private_key: "test-key",
      public_key: "test-pub-key",
      endpoint: "us-east.vpn.example.com:51820",
      allowed_ips: "0.0.0.0/0",
      address: "10.0.0.3/24",
      dns: "1.1.1.1",
    };

    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === "get_connection_status") {
        return Promise.resolve<ConnectionStatus>({
          connected: false,
          current_config: null,
          interface: null,
        });
      }
      if (cmd === "fetch_config_list") {
        return Promise.resolve<ServerConfigResponse>(mockConfigs);
      }
      if (cmd === "get_config_by_id") {
        return Promise.resolve<WireGuardConfig | null>(mockConfig);
      }
      return Promise.resolve(null);
    });

    render(<App />);

    // Fetch configs
    const fetchButton = screen.getByRole("button", { name: /fetch server configs/i });
    await user.click(fetchButton);

    await waitFor(() => {
      expect(screen.getByText("US East Server")).toBeInTheDocument();
    });

    // Select a config
    const serverItem = screen.getByText("US East Server").closest(".server-item");
    if (serverItem) {
      await user.click(serverItem);
    }

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("get_config_by_id", { id: "us-east-1" });
      expect(screen.getByText("US East Server")).toBeInTheDocument();
    });
  });

  it("handles manual config input", async () => {
    const user = userEvent.setup();

    render(<App />);

    const textarea = screen.getByPlaceholderText(/paste wireguard config/i);
    const configText = `[Interface]
PrivateKey = test-private-key
Address = 10.0.0.2/24

[Peer]
PublicKey = test-public-key
Endpoint = vpn.example.com:51820
AllowedIPs = 0.0.0.0/0`;

    await user.type(textarea, configText);

    const applyButton = screen.getByRole("button", { name: /apply config/i });
    await user.click(applyButton);

    await waitFor(() => {
      expect(screen.getByText("vpn.example.com:51820")).toBeInTheDocument();
    });
  });

  it("connects when connect button is clicked with a config", async () => {
    const user = userEvent.setup();

    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === "get_connection_status") {
        return Promise.resolve<ConnectionStatus>({
          connected: false,
          current_config: null,
          interface: null,
        });
      }
      if (cmd === "apply_config") {
        return Promise.resolve("Config applied successfully");
      }
      return Promise.resolve(null);
    });

    render(<App />);

    // First, set a config via manual input
    const textarea = screen.getByPlaceholderText(/paste wireguard config/i);
    const configText = `[Interface]
PrivateKey = test-private-key
Address = 10.0.0.2/24

[Peer]
PublicKey = test-public-key
Endpoint = vpn.example.com:51820
AllowedIPs = 0.0.0.0/0`;

    await user.type(textarea, configText);
    const applyButton = screen.getByRole("button", { name: /apply config/i });
    await user.click(applyButton);

    await waitFor(() => {
      expect(screen.getByText("10.0.0.2/24")).toBeInTheDocument();
    });

    // Now connect
    const connectButton = screen.getByRole("button", { name: /connect/i });
    await user.click(connectButton);

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("apply_config", expect.any(Object));
    });
  });
});

