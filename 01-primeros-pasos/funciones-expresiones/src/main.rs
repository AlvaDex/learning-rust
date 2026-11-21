// --- Funciones auxiliares ---

fn saludar() {
    println!("Hola");
}

// Retorno explícito de la tupla vacía `()` (tipo unidad / unit)
fn saludar_explicito() -> () {
    println!("Hola (retorno explícito)");
}

fn mostrar_puntos(nombre: &str, puntos: u32) {
    println!("{nombre} tiene {puntos} puntos");
}

// Sin `;` en la última línea: es una expresión que retorna el valor
fn doble(n: i32) -> i32 {
    n * 2
}

// `return` sirve para salida anticipada (early return)
fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 0;
    }
    a / b
}

// Retorno múltiple mediante una tupla `(area, perimetro)`
fn rectangulo(ancho: u32, alto: u32) -> (u32, u32) {
    let area = ancho * alto;
    let perimetro = 2 * (ancho + alto);
    (area, perimetro)
}

fn main() {
    // 1. Funciones sin retorno (retornan `()`)
    println!("--- Saludos ---");
    saludar();
    saludar_explicito();

    // 2. Funciones con parámetros
    println!("\n--- Puntos ---");
    mostrar_puntos("Ferris", 120);
    mostrar_puntos("Corro", 95);

    // 3. Funciones con valor de retorno
    println!("\n--- Doble ---");
    let resultado = doble(21);
    println!("El doble de 21 es {resultado}");

    // 4. Bloques como expresiones: la última línea sin `;` es el valor del bloque
    println!("\n--- Expresiones en bloques ---");
    let y = {
        let x = 3;
        x + 1 // Sin `;` -> evalúa a 4
    };
    println!("y vale {y}");

    // 5. Retorno anticipado con `return`
    println!("\n--- División segura ---");
    println!("10 / 2 = {}", dividir(10, 2));
    println!("10 / 0 = {}", dividir(10, 0));

    // 6. Retorno de tupla y desestructuración
    println!("\n--- Rectángulo ---");
    let (area, perimetro) = rectangulo(3, 4);
    println!("Área: {area}, perímetro: {perimetro}");
}