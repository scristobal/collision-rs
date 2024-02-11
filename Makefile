ts-bindings: wasm
	wasm-bindgen ./target/wasm32-unknown-unknown/release/collision_rs.wasm --out-dir ./dist

wasm:
	cargo build --target=wasm32-unknown-unknown --release