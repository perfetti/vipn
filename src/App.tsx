import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface WireGuardConfig {
  interface: string;
  private_key: string;
  address: string;
  dns: string;
  public_key: string;
  endpoint: string;
  allowed_ips: string;
}

function App() {
  const [config, setConfig] = useState<WireGuardConfig | null>(null);
  const [status, setStatus] = useState<string>("disconnected");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadConfig();
    checkStatus();
    // Poll status every 2 seconds
    const interval = setInterval(checkStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  const loadConfig = async () => {
    try {
      setLoading(true);
      setError(null);
      const configData = await invoke<WireGuardConfig>("get_wireguard_config");
      setConfig(configData);
    } catch (err) {
      setError(err as string);
      console.error("Failed to load config:", err);
    } finally {
      setLoading(false);
    }
  };

  const checkStatus = async () => {
    try {
      const currentStatus = await invoke<string>("get_vpn_status");
      setStatus(currentStatus);
    } catch (err) {
      console.error("Failed to check status:", err);
    }
  };

  const handleConnect = async () => {
    if (!config) {
      setError("No configuration available");
      return;
    }
    try {
      setLoading(true);
      setError(null);
      await invoke("connect_vpn", { config });
      await checkStatus();
    } catch (err) {
      setError(err as string);
      console.error("Failed to connect:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleDisconnect = async () => {
    try {
      setLoading(true);
      setError(null);
      await invoke("disconnect_vpn");
      await checkStatus();
    } catch (err) {
      setError(err as string);
      console.error("Failed to disconnect:", err);
    } finally {
      setLoading(false);
    }
  };

  const getStatusColor = () => {
    switch (status) {
      case "connected":
        return "bg-green-500";
      case "connecting":
        return "bg-yellow-500";
      case "disconnected":
        return "bg-gray-500";
      default:
        return "bg-gray-500";
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8">VIPN - WireGuard VPN</h1>

        {/* Status Card */}
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold text-gray-800">Status</h2>
            <div className="flex items-center gap-2">
              <div className={`w-3 h-3 rounded-full ${getStatusColor()}`}></div>
              <span className="text-sm font-medium text-gray-600 capitalize">
                {status}
              </span>
            </div>
          </div>

          <div className="flex gap-4">
            <button
              onClick={handleConnect}
              disabled={loading || status === "connected"}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              {loading ? "Loading..." : "Connect"}
            </button>
            <button
              onClick={handleDisconnect}
              disabled={loading || status === "disconnected"}
              className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              Disconnect
            </button>
            <button
              onClick={loadConfig}
              disabled={loading}
              className="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              Refresh Config
            </button>
          </div>
        </div>

        {/* Error Display */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
            <p className="text-red-800 text-sm">{error}</p>
          </div>
        )}

        {/* Configuration Card */}
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold text-gray-800 mb-4">WireGuard Configuration</h2>

          {loading && !config ? (
            <p className="text-gray-600">Loading configuration...</p>
          ) : config ? (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Interface
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm">
                  {config.interface}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Address
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm">
                  {config.address}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  DNS
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm">
                  {config.dns}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Endpoint
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm">
                  {config.endpoint}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Allowed IPs
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm">
                  {config.allowed_ips}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Public Key
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm break-all">
                  {config.public_key}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Private Key
                </label>
                <div className="bg-gray-50 p-3 rounded-md font-mono text-sm break-all">
                  {config.private_key.substring(0, 20)}...
                </div>
              </div>
            </div>
          ) : (
            <p className="text-gray-600">No configuration available</p>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;

