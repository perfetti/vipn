// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod wireguard;

use wireguard::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Get a mock WireGuard config
#[tauri::command]
fn get_mock_config() -> WireGuardConfig {
    wireguard::get_config()
}

/// Fetch list of available configs from server (mocked)
#[tauri::command]
async fn fetch_config_list() -> Result<ServerConfigResponse, String> {
    Ok(wireguard::fetch_config_list_from_server().await)
}

/// Get a specific config by ID from server (mocked)
#[tauri::command]
async fn get_config_by_id(id: String) -> Result<Option<WireGuardConfig>, String> {
    Ok(wireguard::get_config_by_id(&id))
}

/// Get current connection status
#[tauri::command]
fn get_connection_status() -> ConnectionStatus {
    wireguard::get_connection_status()
}

/// Apply a WireGuard config
#[tauri::command]
async fn apply_config(config: WireGuardConfig) -> Result<String, String> {
    wireguard::apply_config(config).await
}

/// Disconnect from VPN
#[tauri::command]
async fn disconnect() -> Result<String, String> {
    wireguard::disconnect().await
}

/// Convert config to wg-quick format
#[tauri::command]
fn config_to_wg_quick_format(config: WireGuardConfig) -> String {
    wireguard::config_to_wg_quick_format(&config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_mock_config,
            fetch_config_list,
            get_config_by_id,
            get_connection_status,
            apply_config,
            disconnect,
            config_to_wg_quick_format
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
