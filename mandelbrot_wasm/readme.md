# Comandos
## Instalar Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Instalar wasm-pack
```sh
cargo install wasm-pack
```

## Compilar codigo y generar lib
```sh
wasm-pack build --target web
```