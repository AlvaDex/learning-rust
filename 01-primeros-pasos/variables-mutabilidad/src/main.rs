// Las constantes siempre requieren tipo explícito y se escriben en MAYÚSCULAS
const SEGUNDOS_POR_HORA: u32 = 60 * 60;
const MAX_VIDAS: u32 = 3;

fn main() {
    // 1. Inmutabilidad por defecto e inferencia/anotación de tipo
    let edad: i32 = 7; // Rust infiere tipos, pero podemos anotarlos explícitamente
    println!("Edad inicial: {edad}");

    // 2. Variables mutables con `mut` (mismo tipo, distinto valor)
    println!("\n--- Mutabilidad con `mut` ---");
    let mut puntos = 10;
    println!("Puntos iniciales: {puntos}");

    puntos = 20;
    println!("Tras reasignar:   {puntos}");

    puntos += 5;
    println!("Tras sumar += 5:  {puntos}");

    // 3. Shadowing: redeclarar con `let` en el mismo ámbito
    println!("\n--- Shadowing (mismo tipo) ---");
    let x = 5;
    let x = x + 1; // 6
    let x = x * 2; // 12
    println!("Valor final de x: {x}");

    // Shadowing permitiendo cambiar el tipo de la variable
    println!("\n--- Shadowing (cambio de tipo) ---");
    let espacios = "    ";        // &str
    let espacios = espacios.len(); // usize
    println!("Cantidad de espacios: {espacios}");

    // 4. Uso de constantes globales
    println!("\n--- Constantes ---");
    println!("Una hora tiene {SEGUNDOS_POR_HORA} segundos");
    println!("Empiezas con {MAX_VIDAS} vidas");

    // 5. Alcance (scope) con bloques `{}`
    println!("\n--- Ámbito de bloque (`{}`) ---");
    let x = 5;
    {
        let x = x * 10; // Sombrea `x` solo dentro de este bloque
        let y = 1;      // `y` deja de existir al cerrar la llave
        println!("Dentro del bloque -> x: {x}, y: {y}");
    }
    println!("Fuera del bloque  -> x: {x}");
}