// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use blockchain::Blockchain;
use crate::errors::Result;
use log::{info,error};
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
#[tauri::command]
fn cmd_reindex_tauri() -> i32 {
    let refined = crate::cli::cmd_reindex().unwrap();
    refined
}

#[tauri::command]
fn cmd_create_blockchain_tauri(address: &str) {
    crate::cli::cmd_create_blockchain(address).unwrap();
}

#[tauri::command]
fn cmd_send_tauri(from: &str, to: &str, amount: i32, mineNow: bool) {
    match crate::cli::cmd_send(from, to, amount, mineNow) {
        Ok(_) => info!("Transaction sent successfully"),
        Err(e) => error!("Failed to send transaction: {}", e)
    }
}

#[tauri::command]
fn cmd_start_miner_tauri(port: &str, address: &str) {
    match crate::cli::cmd_start_miner(port, address) {
        Ok(_) => println!("Miner started successfully"),
        Err(e) => eprintln!("Error: {}",e)
    }
}

#[tauri::command]
fn cmd_print_chain_tauri() -> String {
    match format_blockchain() {
        Ok(chain_str) => chain_str,
        Err(e) => format!("Error printing chain: {}", e)
    }
}

fn format_blockchain() -> Result<String> {
    let bc = Blockchain::new()?;
    let mut output = String::from("Blockchain:\n");
    
    for (i, block) in bc.iter().enumerate() {
        output.push_str(&format!("\nBlock {}:\n", i));
        output.push_str(&format!("  Timestamp: {}\n", block.timestamp));
        output.push_str(&format!("  Hash: {}\n", hex::encode(&block.hash)));
        output.push_str(&format!("  Previous Hash: {}\n", hex::encode(&block.prev_block_hash)));
        
        output.push_str("  Transactions:\n");
        for (j, tx) in block.transactions.iter().enumerate() {
            output.push_str(&format!("    Transaction {}:\n", j));
            output.push_str(&format!("      ID: {}\n", hex::encode(&tx.id)));
            
            output.push_str("      Inputs:\n");
            for vin in &tx.vin {
                output.push_str(&format!("        TxID: {}\n", hex::encode(&vin.txid)));
                output.push_str(&format!("        Vout: {}\n", vin.vout));
                output.push_str(&format!("        Signature: {}\n", base64::encode(&vin.signature)));
            }
            
            output.push_str("      Outputs:\n");
            for vout in &tx.vout {
                output.push_str(&format!("        Value: {}\n", vout.value));
                output.push_str(&format!("        PubKeyHash: {}\n", hex::encode(&vout.pub_key_hash)));
            }
        }
    }
    
    Ok(output)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd_get_address_wrapper,
            cmd_create_wallet_tauri,
            cmd_get_balance_tauri,
            cmd_reindex_tauri,
            cmd_create_blockchain_tauri,
            cmd_send_tauri,
            cmd_start_miner_tauri,
            cmd_print_chain_tauri
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
