# Builds wasm and javascript into /pkg for use in browser
cargo build --release --lib --target wasm32-unknown-unknown --no-default-features --features wasm-bindings
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/labelize.wasm