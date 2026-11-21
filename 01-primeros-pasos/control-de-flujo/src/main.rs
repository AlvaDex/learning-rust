fn main() {
    // 1. Condicionales `if` / `else if` / `else`
    println!("--- Temperatura ---");
    let temperatura = 22;

    if temperatura > 30 {
        println!("Hace calor");
    } else if temperatura > 15 {
        println!("Se está bien");
    } else {
        println!("Hace frío");
    }

    // 2. `if` como expresión (ambas ramas deben devolver el mismo tipo)
    println!("\n--- Tipo de entrada (`if` como expresión) ---");
    let edad = 15;
    let entrada = if edad >= 18 { "general" } else { "reducida" };
    println!("Entrada {entrada}");

    // 3. Bucle infinito `loop` con ruptura condicional (`break`)
    println!("\n--- Bucle `loop` ---");
    let mut intentos = 0;

    loop {
        intentos += 1;
        println!("Intento {intentos}");

        if intentos == 3 {
            break;
        }
    }

    // 4. `loop` devolviendo un valor mediante `break <expresión>;`
    println!("\n--- `loop` con valor de retorno ---");
    let mut contador = 0;

    let resultado = loop {
        contador += 1;

        if contador == 10 {
            break contador * 2;
        }
    };
    println!("Resultado: {resultado}");

    // 5. Bucle condicional `while`
    println!("\n--- Cuenta atrás con `while` ---");
    let mut cuenta = 3;

    while cuenta > 0 {
        println!("{cuenta}...");
        cuenta -= 1;
    }
    println!("¡Despegue!");

    // 6. `for` con rango exclusivo (`1..4` -> 1, 2, 3)
    println!("\n--- Rango exclusivo `1..4` ---");
    for i in 1..4 {
        println!("Vuelta {i}");
    }

    // 7. `for` con rango inclusivo (`1..=3`) invertido con `.rev()`
    println!("\n--- Rango inclusivo invertido `(1..=3).rev()` ---");
    for i in (1..=3).rev() {
        println!("{i}...");
    }
    println!("¡Despegue!");

    // 8. Recorrer elementos de un array con `for`
    println!("\n--- Recorriendo array ---");
    let lenguajes = ["Rust", "Go", "Zig"];

    for lenguaje in lenguajes {
        println!("Me gusta {lenguaje}");
    }

    // 9. Etiquetas de bucle (`'etiqueta`) para romper bucles anidados
    println!("\n--- Etiqueta de bucle `'externo` ---");
    'externo: for fila in 1..=3 {
        for columna in 1..=3 {
            if fila * columna == 4 {
                println!("Encontrado: {fila} × {columna} = 4");
                break 'externo;
            }
            println!("{fila} × {columna}");
        }
    }
}