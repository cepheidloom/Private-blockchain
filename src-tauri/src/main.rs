// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use crate::cli::Cli;
// use crate::errors::Result;
// mod block;
// mod errors;
// mod blockchain;
// mod cli;
// mod transaction;
// mod wallets;
// mod tx;
// mod utxoset;
// mod server;

fn main() /*-> Result<()> */{
    bolk_lib::run()
    // let mut cli = Cli::new()?;
    // cli.run()?;

    // Ok(())
}
