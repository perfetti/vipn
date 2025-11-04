/**
 * Shared TypeScript types for VIPN WireGuard VPN application
 */

export interface WireGuardConfig {
  name: string;
  private_key: string;
  public_key: string;
  endpoint: string;
  allowed_ips: string;
  dns?: string;
  address: string;
  persistent_keepalive?: number;
}

export interface ServerConfigItem {
  id: string;
  name: string;
  location: string;
  endpoint: string;
}

export interface ServerConfigResponse {
  configs: ServerConfigItem[];
}

export interface ConnectionStatus {
  connected: boolean;
  current_config: string | null;
  interface: string | null;
}

export type StatusMessageType = "success" | "error" | "info";

