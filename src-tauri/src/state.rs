use nostr_sdk::prelude::*;
use tokio::sync::Mutex;

#[derive(Default, Debug)]
pub struct NostrState(pub Mutex<Client>);