windows:
	cargo build --release --target x86_64-pc-windows-msvc --features default
linux:
	cargo build --release --target x86_64-unknown-linux-gnu --features default
macX86:
	cargo build --release --target x86_64-apple-darwin --features default
macARM64:
	cargo build --release --target aarch64-apple-darwin --features default
wasm:
	wasm-pack build --target web -- --features wasm
