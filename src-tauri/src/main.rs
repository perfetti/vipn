// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod wireguard;

use wireguard::{get_config, WireGuardConfig};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_wireguard_config,
            connect_vpn,
            disconnect_vpn,
            get_vpn_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_wireguard_config() -> Result<WireGuardConfig, String> {
    get_config().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn connect_vpn(config: WireGuardConfig) -> Result<String, String> {
    // TODO: Implement actual WireGuard connection
    println!("Connecting with config: {:?}", config);
    Ok("Connected".to_string())
}

#[tauri::command]
async fn disconnect_vpn() -> Result<String, String> {
    // TODO: Implement actual WireGuard disconnection
    println!("Disconnecting VPN");
    Ok("Disconnected".to_string())
}

#[tauri::command]
async fn get_vpn_status() -> Result<String, String> {
    // TODO: Implement actual status check
    Ok("disconnected".to_string())
}

