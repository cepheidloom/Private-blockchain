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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![cmd_get_address_wrapper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
