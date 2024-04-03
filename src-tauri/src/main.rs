// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc};
use tokio::sync::Mutex;
use tauri::State;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  // Generate new random keys
  let my_keys = Keys::generate();

  // Show bech32 public key
  let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
  println!("Bech32 PubKey: {}", bech32_pubkey);

  // Create new client
  let client = Client::new(&my_keys);

  client.add_relay("wss://relay.damus.io").await?;

  client.connect().await;

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_pubkey])
    .manage(Arc::new(Mutex::new(client)))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
    Ok(())
}

#[tauri::command]
async fn get_pubkey(state: State<'_, Arc<Mutex<Client>>>) -> Result<PublicKey, String> {
  let client = state.lock().await;
  let signer = match client.signer().await {
    Ok(sig) => sig,
    Err(_e) => return Err("could not get signer".into()),
  };
  let pubkey = match signer.public_key().await {
    Ok(key) => key,
    Err(_e) => return Err("No Public Key".into()),
  };

  Ok(pubkey)
}