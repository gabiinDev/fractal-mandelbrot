# Mandelbrot

Este proyecto es una simple demostración, a modo de aprendizaje, en el cual se creó una función que permita recrear la visualización del conjunto de Mandelbrot. Utilizando tanto JavaScript puro como una implementación en WebAssembly (Wasm) con Rust.

## Estructura del Proyecto

- `index.html`: Contiene la implementación en JavaScript del conjunto de Mandelbrot.
- `index2.html`: Utiliza la librería WebAssembly escrita en Rust para renderizar el conjunto de Mandelbrot.
- `mandelbrot_wasm/`: Carpeta que contiene la implementación en Rust y el código necesario para compilar la librería a WebAssembly.
- `mandelbrot_wasm/README.md`: Instrucciones para instalar y compilar la librería Rust.

## Cómo Ejecutar

1. **Instalar dependencias necesarias para WebAssembly:**

Es necesario tener Rust y `wasm-pack` instalados. Se pueden instalar con los siguientes comandos:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

2. **Compilar librería WebAssembly:**

Navegar a la carpeta mandelbrot_wasm y compilar la librería:

```sh
  cd mandelbrot_wasm
  wasm-pack build --target web
```

## Información Adicional
Para obtener información sobre cómo instalar y compilar la librería Rust, consultar el archivo README.md en la carpeta mandelbrot_wasm:

[mandelbrot_wasm/README.md](mandelbrot_wasm/)

