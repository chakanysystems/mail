// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;

use std::fs::create_dir_all;
use std::path::PathBuf;
use tauri::{Manager, State};
use nostr_sdk::prelude::*;
use nostr_sqlcipher::{SQLCipherDatabase};
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
        let dir_buf: PathBuf = handle.path_resolver().app_data_dir().expect("This shouldn't be none");
        let cloned_dir_buf = dir_buf.clone();
        create_dir_all(&cloned_dir_buf).expect("Could not create app data dir");
        tauri::async_runtime::spawn(async move {
          let mut dir_buf = cloned_dir_buf;
          if cfg!(debug_assertions) {
            dir_buf.push("dev.db"); // use a different db for dev testing
          } else {
            dir_buf.push("stable.db");
          }
          let data_dir = dir_buf.to_str().unwrap();
          let database = SQLCipherDatabase::open(data_dir, "PASSWORD".to_string()).await.unwrap();
          let nostr_state = handle.state::<NostrState>();
          *nostr_state.0.lock().await = ClientBuilder::default().signer(&my_keys).database(database).build();
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

#[derive(Debug)]
struct Contact {
    name: String,
    their_public_key: String,
    their_shared_key: String,
    our_shared_key: String,
}

fn get_contacts() -> Vec<Contact> {
    let mut contacts: Vec<Contact> = Vec::new();
    contacts.push(Contact {
        name: "Jack Chakany".to_string(),
        their_public_key: "".to_string(),
        their_shared_key: "".to_string(),
        our_shared_key: "".to_string()
    });


    return contacts
}

#[tauri::command]
async fn publish_messsage(nostr: State<'_, NostrState>, to: Vec<String>, cc: Vec<String>, bcc: Vec<String>, subject: String, content: String, attachments: Vec<String>) -> Result<String, String> {
    let client = nostr.0.lock().await;

    for recipient in to.iter() {
        // get contacts list
    }

    for recipient in cc.iter() {

    }

    for recipient in bcc.iter() {

    }
    Ok("F".into())
}
