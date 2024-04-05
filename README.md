# mail
up and coming mail app for nostr.
uses tauri, sveltekit, and shadcn ui

### Build
Make sure you have rust installed with https://rustup.rs
Make sure you have bunjs installed, and node (for now)
it's also wise to ensure that you have a c compiler installed, we are using sqlcipher bundled so you will need to install a compiler

1. install js dependencies with `bun install` in the root directory
2. run bunx tauri build
3. profit???

no cross-compile (sqlite)

### Dev
Same steps as build section above, except instead of running `bunx tauri build` it's `bunx tauri dev`
