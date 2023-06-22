# Fluke - Frontend


- Fluke's frontend is written using [Yew - Rust / Wasm client web app framework](https://github.com/yewstack/yew)

- [Trunk for bundling](https://github.com/thedodd/trunk)

- [wasm-bindgen book/guide](https://rustwasm.github.io/wasm-bindgen/) covers most topics

- [Tailwind CSS for Styling](https://tailwindcss.com/)

<br>
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

## Tailwind

Tailwind doesn't offer clear documentation for bundling with trunk, but we can make it work with the Tailwind CLI. There 
are two methods, one with `node.js`, and one with a standalone executable. 


### Node.js

```bash
npm install -D tailwindcss

npx tailwindcss init
```

You should already have the tailwind.config.js and tailwind.css files from cloning the repo. 

Now, just build the template css files: 

```bash
npx tailwindcss -i ./src/input.css -o ./dist/output.css --watch
```

### Standalone Executable 

Find the [latest release](https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.3.2) for your operating system. 

This example is for macOS, but replace accordingly:

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x tailwindcss-macos-arm64
mv tailwindcss-macos-arm64 tailwindcss

# Start a watcher
./tailwindcss -i input.css -o output.css --watch

# Compile and minify your CSS for production
./tailwindcss -i input.css -o output.css --minify
```

<br>

# Run and Build

```bash
trunk serve # with hosting
trunk watch # without hosting 
trunk build --release
```

Unless overwritten, the output will be located in the `dist` directory.

<br>

# Notes

On WSL2 you may need to add some env variables to connect with ssl:

- `export OPENSSL_INCLUDE_DIR=/usr/include/openssl`
- `export OPENSSL_LIB_DIR=/usr/lib`

Caution with using too many icons with `wasm-bindgen`, there have been issues
with [refusal to load binaries that are too large](https://github.com/rustwasm/wasm-pack/issues/981)