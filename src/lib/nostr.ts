import { writable } from 'svelte/store';
import NDK from '@nostr-dev-kit/ndk';
import NDKCacheAdapterDexie from "@nostr-dev-kit/ndk-cache-dexie";

const dexieAdapter = new NDKCacheAdapterDexie({ dbName: 'chaker-web-db' });
const ndkBase = new NDK({ cacheAdapter: dexieAdapter, explicitRelayUrls: ['wss://nos.lol', 'wss://relay.damus.io'] });
export const ndk = writable(ndkBase)
export const pubkey = writable("");
