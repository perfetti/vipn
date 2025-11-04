import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type {
  WireGuardConfig,
  ServerConfigItem,
  ServerConfigResponse,
  ConnectionStatus,
  StatusMessageType,
} from "./types";

function App() {
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>({
    connected: false,
    current_config: null,
    interface: null,
  });
  const [currentConfig, setCurrentConfig] = useState<WireGuardConfig | null>(null);
  const [manualConfig, setManualConfig] = useState("");
  const [serverConfigs, setServerConfigs] = useState<ServerConfigItem[]>([]);
  const [showServerList, setShowServerList] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState<string>("");
  const [statusType, setStatusType] = useState<StatusMessageType>("info");

  // Load connection status on mount
  useEffect(() => {
    loadConnectionStatus();
  }, []);

  const loadConnectionStatus = async () => {
    try {
      const status = await invoke<ConnectionStatus>("get_connection_status");
      setConnectionStatus(status);
    } catch (error) {
      console.error("Failed to load connection status:", error);
    }
  };

  const showStatus = (message: string, type: StatusMessageType = "info") => {
    setStatusMessage(message);
    setStatusType(type);
    setTimeout(() => setStatusMessage(""), 5000);
  };

  const handleConnect = async () => {
    if (!currentConfig) {
      showStatus("Please select or enter a config first", "error");
      return;
    }

    setIsLoading(true);
    try {
      const result = await invoke<string>("apply_config", { config: currentConfig });
      setConnectionStatus({ ...connectionStatus, connected: true, current_config: currentConfig.name });
      showStatus(result, "success");
    } catch (error) {
      showStatus(`Failed to connect: ${error}`, "error");
    } finally {
      setIsLoading(false);
      loadConnectionStatus();
    }
  };

  const handleDisconnect = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<string>("disconnect");
      setConnectionStatus({ ...connectionStatus, connected: false, current_config: null });
      showStatus(result, "success");
    } catch (error) {
      showStatus(`Failed to disconnect: ${error}`, "error");
    } finally {
      setIsLoading(false);
      loadConnectionStatus();
    }
  };

  const handleLoadFromServer = async () => {
    setIsLoading(true);
    try {
      const response = await invoke<ServerConfigResponse>("fetch_config_list");
      setServerConfigs(response.configs);
      setShowServerList(true);
      showStatus(`Loaded ${response.configs.length} server configurations`, "success");
    } catch (error) {
      showStatus(`Failed to load server configs: ${error}`, "error");
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectServerConfig = async (configId: string) => {
    setIsLoading(true);
    try {
      const config = await invoke<WireGuardConfig | null>("get_config_by_id", { id: configId });
      if (config) {
        setCurrentConfig(config);
        setShowServerList(false);
        showStatus(`Selected config: ${config.name}`, "success");
      } else {
        showStatus("Config not found", "error");
      }
    } catch (error) {
      showStatus(`Failed to load config: ${error}`, "error");
    } finally {
      setIsLoading(false);
    }
  };

  const handleApplyManualConfig = () => {
    if (!manualConfig.trim()) {
      showStatus("Please enter a config", "error");
      return;
    }

    // Try to parse the config - for now, we'll create a minimal config
    // In a real app, you'd parse the wg-quick format
    try {
      // This is a placeholder - in reality, you'd parse the wg-quick format
      const parsedConfig: WireGuardConfig = {
        name: "Manual Config",
        private_key: "",
        public_key: "",
        endpoint: "",
        allowed_ips: "0.0.0.0/0",
        address: "",
        dns: undefined,
      };

      // Extract values from wg-quick format (basic parsing)
      const lines = manualConfig.split("\n");
      for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith("PrivateKey =")) {
          parsedConfig.private_key = trimmed.split("=")[1]?.trim() || "";
        } else if (trimmed.startsWith("PublicKey =")) {
          parsedConfig.public_key = trimmed.split("=")[1]?.trim() || "";
        } else if (trimmed.startsWith("Endpoint =")) {
          parsedConfig.endpoint = trimmed.split("=")[1]?.trim() || "";
        } else if (trimmed.startsWith("Address =")) {
          parsedConfig.address = trimmed.split("=")[1]?.trim() || "";
        } else if (trimmed.startsWith("DNS =")) {
          parsedConfig.dns = trimmed.split("=")[1]?.trim();
        } else if (trimmed.startsWith("AllowedIPs =")) {
          parsedConfig.allowed_ips = trimmed.split("=")[1]?.trim() || "0.0.0.0/0";
        }
      }

      if (!parsedConfig.private_key || !parsedConfig.public_key || !parsedConfig.endpoint) {
        showStatus("Invalid config format. Please provide a valid WireGuard config.", "error");
        return;
      }

      setCurrentConfig(parsedConfig);
      showStatus("Config loaded successfully", "success");
    } catch (error) {
      showStatus(`Failed to parse config: ${error}`, "error");
    }
  };

  return (
    <main className="vpn-app">
      <div className="app-header">
        <h1>VIPN - WireGuard VPN</h1>
        <div className={`status-indicator ${connectionStatus.connected ? "connected" : "disconnected"}`}>
          <div className="status-dot"></div>
          <span>{connectionStatus.connected ? "Connected" : "Disconnected"}</span>
        </div>
      </div>

      <div className="app-content">
        <div className="connection-panel">
          <div className="connection-controls">
            <button
              className={`connect-button ${connectionStatus.connected ? "disconnect" : "connect"}`}
              onClick={connectionStatus.connected ? handleDisconnect : handleConnect}
              disabled={isLoading}
            >
              {isLoading ? "Loading..." : connectionStatus.connected ? "Disconnect" : "Connect"}
            </button>
          </div>

          {statusMessage && (
            <div className={`status-message ${statusType}`}>
              {statusMessage}
            </div>
          )}

          {connectionStatus.current_config && (
            <div className="current-config-name">
              Current: {connectionStatus.current_config}
            </div>
          )}
        </div>

        <div className="config-section">
          <h2>Configuration</h2>

          <div className="config-options">
            <div className="config-option">
              <h3>Load from Server</h3>
              <button onClick={handleLoadFromServer} disabled={isLoading}>
                {isLoading ? "Loading..." : "Fetch Server Configs"}
              </button>

              {showServerList && serverConfigs.length > 0 && (
                <div className="server-list">
                  <h4>Select a server:</h4>
                  {serverConfigs.map((config) => (
                    <div
                      key={config.id}
                      className="server-item"
                      onClick={() => handleSelectServerConfig(config.id)}
                    >
                      <div className="server-name">{config.name}</div>
                      <div className="server-location">{config.location}</div>
                      <div className="server-endpoint">{config.endpoint}</div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            <div className="config-option">
              <h3>Manual Config</h3>
              <textarea
                value={manualConfig}
                onChange={(e) => setManualConfig(e.target.value)}
                placeholder="Paste WireGuard config here (wg-quick format)..."
                rows={10}
                className="config-textarea"
              />
              <button onClick={handleApplyManualConfig} disabled={isLoading}>
                Apply Config
              </button>
            </div>
          </div>

          {currentConfig && (
            <div className="config-details">
              <h3>Current Config Details</h3>
              <div className="config-info">
                <div className="info-row">
                  <span className="info-label">Name:</span>
                  <span className="info-value">{currentConfig.name}</span>
                </div>
                <div className="info-row">
                  <span className="info-label">Endpoint:</span>
                  <span className="info-value">{currentConfig.endpoint}</span>
                </div>
                <div className="info-row">
                  <span className="info-label">Address:</span>
                  <span className="info-value">{currentConfig.address}</span>
                </div>
                <div className="info-row">
                  <span className="info-label">Allowed IPs:</span>
                  <span className="info-value">{currentConfig.allowed_ips}</span>
                </div>
                {currentConfig.dns && (
                  <div className="info-row">
                    <span className="info-label">DNS:</span>
                    <span className="info-value">{currentConfig.dns}</span>
                  </div>
                )}
                <div className="info-row">
                  <span className="info-label">Public Key:</span>
                  <span className="info-value truncated">{currentConfig.public_key}</span>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
