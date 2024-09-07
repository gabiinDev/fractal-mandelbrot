# Comandos
## Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Instalar wasm-pack
cargo install wasm-pack

## Compilar codigo y generar lib
wasm-pack build --target web