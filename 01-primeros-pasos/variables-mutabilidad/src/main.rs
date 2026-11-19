// Las constantes siempre requieren tipo explícito y se escriben en MAYÚSCULAS
const SEGUNDOS_POR_HORA: u32 = 60 * 60;
const MAX_VIDAS: u32 = 3;

fn main() {
    // Rust puede inferir el tipo, aunque también se puede declarar explícitamente
    let edad: i32 = 7;
    println!("Edad: {edad}");

    // `mut` permite cambiar el valor (no el tipo); úsalo solo cuando haga falta
    let mut puntos = 10;
    println!("Puntos: {puntos}");

    puntos = 20;
    println!("Puntos: {puntos}");

    puntos += 5;
    println!("Puntos: {puntos}");

    // Shadowing: redeclarar con `let` crea una nueva variable en el mismo ámbito
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x vale {x}");

    // Shadowing también permite cambiar el tipo del valor
    let espacios = "    ";
    let espacios = espacios.len();
    println!("Hay {espacios} espacios");

    // Uso de constantes
    println!("Una hora tiene {SEGUNDOS_POR_HORA} segundos");
    println!("Empiezas con {MAX_VIDAS} vidas");

    // Alcance (scope): `y` y el `x` interno solo viven dentro del bloque `{}`
    let x = 5;
    {
        let x = x * 10;
        let y = 1;
        println!("Dentro: x = {x}, y = {y}");
    }
    println!("Fuera: x = {x}");
}