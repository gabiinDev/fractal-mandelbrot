use wasm_bindgen::prelude::*;

/// Calcula el número de iteraciones para un punto en el conjunto de Mandelbrot
#[wasm_bindgen]
pub fn mandelbrot(x: f64, y: f64, max_iterations: usize) -> usize {
    let mut real = x;
    let mut imag = y;
    for i in 0..max_iterations {
        let temp_real = real * real - imag * imag + x;
        imag = 2.0 * real * imag + y;
        real = temp_real;
        if real * real + imag * imag > 4.0 {
            return i;
        }
    }
    max_iterations
}

/// Dibuja el conjunto de Mandelbrot en un buffer de datos
#[wasm_bindgen]
pub fn draw_mandelbrot(width: usize, height: usize, max_iterations: usize) -> Box<[u8]> {
    let mut buffer = vec![0; width * height * 4]; // RGBA buffer

    for px in 0..width {
        for py in 0..height {
            let x = (px as f64 / width as f64) * 4.0 - 2.0;
            let y = (py as f64 / height as f64) * 4.0 - 2.0;
            let iteration = mandelbrot(x, y, max_iterations);

            let color = if iteration == max_iterations {
                (0, 0, 0, 255) // Negro para puntos dentro del conjunto
            } else {
                let hue = 200 + (iteration * 60) / max_iterations;
                let saturation = 50 + (iteration * 50) / max_iterations;
                let lightness = 30 + (iteration * 25) / max_iterations;
                hsl_to_rgba(hue, saturation, lightness)
            };

            let index = (py * width + px) * 4;
            buffer[index] = color.0;
            buffer[index + 1] = color.1;
            buffer[index + 2] = color.2;
            buffer[index + 3] = color.3;
        }
    }

    buffer.into_boxed_slice()
}

fn hsl_to_rgba(hue: usize, saturation: usize, lightness: usize) -> (u8, u8, u8, u8) {
    // Conversión simple de HSL a RGB
    // Implementación básica para fines demostrativos
    // Puedes usar una biblioteca para una conversión más precisa
    (hue as u8, saturation as u8, lightness as u8, 255)
}
