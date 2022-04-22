# Camera Mouse

Remaster of small demo from many years ago, which tracks movement in the camera's video output and uses the right-most point of movement as position inside the video frame. This could be translated to the screen position to control the mouse pointer.

## Usage

### Run locally

Install GTK3 development `libgtk-3-dev`/`gtk3-devel` and `clang` packages first, they are required for the build.

Run `cargo run --release`.

### Deyploy and run in web

Unfortunately, it does not work, because spawning threads fails :( But if it would work:

You need `wasm-pack`:
```bash
cargo install wasm-pack
```

Then build with:
```bash
wasm-pack build --release --target web -- --no-default-features --features "wasm"
```

Run a webserver in the project directory or deploy the relevant files to run it in web. The relevant files are:
```gitignore
index.*
/pkg
```

IMPORTANT: The following headers must be sent in HTTP responses for threading support in WASM:
```HTTP
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Opener-Policy: same-origin
```

Example using [`http`](https://lib.rs/crates/https)
```bash
http -H "Cross-Origin-Embedder-Policy: require-corp" -H "Cross-Origin-Opener-Policy: same-origin"
```
