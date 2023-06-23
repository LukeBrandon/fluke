# Fluke - Frontend


- Fluke's frontend is written using [Yew - Rust / Wasm client web app framework](https://github.com/yewstack/yew)

- [Trunk for bundling](https://github.com/thedodd/trunk)

- [wasm-bindgen book/guide](https://rustwasm.github.io/wasm-bindgen/) covers most topics

- [Tailwind CSS for Styling](https://tailwindcss.com/)

<br>

# Installation / Setup

## Rust and Trunk

This assumes a typical rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

# Run and Build

```bash
trunk serve # with hosting
trunk watch # without hosting 
trunk build --release
```

Unless overwritten, the output will be located in the `dist` directory.

<br>

# Notes

### On WSL2 you may need to add some env variables to connect with ssl:

- `export OPENSSL_INCLUDE_DIR=/usr/include/openssl`
- `export OPENSSL_LIB_DIR=/usr/lib`

<br>

### Caution with using too many icons with `wasm-bindgen`, there have been issues with [refusal to load binaries that are too large](https://github.com/rustwasm/wasm-pack/issues/981)

<br>

### For tailwind syntax highlighting and other VSCode features: 

- Open VS Code Settings (Ctrl/Cmd P + > Settings)

- Type Tailwind

- Look for Tailwind CSS: Include Languages

- Press add item

-  Add rust in key and html in value