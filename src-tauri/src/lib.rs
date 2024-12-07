// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::errors::Result;
mod block;

mod errors;
mod blockchain;
mod cli;
mod transaction;
mod wallets;
mod tx;
mod utxoset;
mod server;


#[tauri::command]   
fn cmd_get_address_wrapper() -> Vec<String>{
    let address = crate::cli::cmd_list_address().unwrap();
    address
}

#[tauri::command]
fn cmd_create_wallet_tauri() -> String {
    let new_address = crate::cli::cmd_create_wallet().unwrap();
    new_address
}

#[tauri::command]

fn cmd_get_balance_tauri(address: &str) -> i32 {
    let balance = crate::cli::cmd_get_balance(address).unwrap();
    balance
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd_get_address_wrapper,
            cmd_create_wallet_tauri,
            cmd_get_balance_tauri
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
