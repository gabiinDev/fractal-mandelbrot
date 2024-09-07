function mandelbrot(x, y, maxIterations) {
  let real = x;
  let imag = y;
  for (let i = 0; i < maxIterations; i++) {
    const tempReal = real * real - imag * imag + x;
    imag = 2 * real * imag + y;
    real = tempReal;
    if (real * real + imag * imag > 4) {
      return i;
    }
  }
  return maxIterations;
}

export function drawMandelbrot(canvas, maxIterations) {
  const ctx = canvas.getContext("2d");
  const width = canvas.width;
  const height = canvas.height;

  for (let px = 0; px < width; px++) {
    for (let py = 0; py < height; py++) {
      const x = (px / width) * 4 - 2;
      const y = (py / height) * 4 - 2;
      const iteration = mandelbrot(x, y, maxIterations);
      let color;
      if (iteration === maxIterations) {
        color = "#000000"; // Negro para puntos dentro del conjunto
      } else {
        // Usar una paleta de colores más suave
        const hue = 200 + (iteration * 60) / maxIterations; // Rango de azul a violeta
        const saturation = 50 + (iteration * 50) / maxIterations; // Aumentar saturación gradualmente
        const lightness = 30 + (iteration * 25) / maxIterations; // Aumentar brillo gradualmente
        color = `hsl(${hue}, ${saturation}%, ${lightness}%)`;
      }
      ctx.fillStyle = color;
      ctx.fillRect(px, py, 1, 1);
    }
  }
}
