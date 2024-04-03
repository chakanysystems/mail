// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;

use tauri::{Manager, State};
use nostr_sdk::prelude::*;
use crate::state::NostrState;

#[tokio::main]
async fn main() -> Result<()> {
  // Generate new random keys
  let my_keys = Keys::generate();

  // Show bech32 public key
  let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
  println!("Bech32 PubKey: {}", bech32_pubkey);

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_pubkey])
    .manage(NostrState(Default::default()))
      .setup(|app| {
        let handle = app.handle();
        tauri::async_runtime::spawn(async move {
           let nostr_state = handle.state::<NostrState>();
           *nostr_state.0.lock().await = Client::new(&my_keys);
        });
        Ok(())
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
    Ok(())
}

#[tauri::command]
async fn get_pubkey(nostr: State<'_, NostrState>) -> Result<PublicKey, String> {
  let client = nostr.0.lock().await;
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