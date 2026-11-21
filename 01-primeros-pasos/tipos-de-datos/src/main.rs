fn main() {
    // 1. Inferencia y anotación explícita de tipos escalares
    let entero = 42;         // i32 por defecto
    let decimal = 3.14;      // f64 por defecto
    let activo = true;       // bool
    let puntos: u64 = 1_500; // Anotación explícita

    println!("Básicos -> entero: {entero}, decimal: {decimal}, activo: {activo}, puntos: {puntos}");

    // 2. Límites de los tipos enteros
    println!("\n--- Límites de enteros ---");
    println!("i8:  {} a {}", i8::MIN, i8::MAX);
    println!("u8:  {} a {}", u8::MIN, u8::MAX);
    println!("i32: {} a {}", i32::MIN, i32::MAX);
    println!("u64: {} a {}", u64::MIN, u64::MAX);

    // 3. Control de desbordamiento (overflow)
    println!("\n--- Overflow controlado ---");
    let volumen: u8 = 250;
    println!("wrapping_add(10):   {}", volumen.wrapping_add(10));   // 4 (da la vuelta)
    println!("saturating_add(10): {}", volumen.saturating_add(10)); // 255 (tope máximo)

    // 4. Flotantes y operaciones aritméticas
    println!("\n--- Flotantes y división ---");
    let precio = 19.99; // f64
    let ancho: f32 = 2.5;
    println!("Precio doble: {}, ancho: {}", precio * 2.0, ancho);
    println!("7 / 2 (enteros):   {}", 7 / 2);     // 3 (trunca decimales)
    println!("7.0 / 2.0 (float): {}", 7.0 / 2.0); // 3.5
    println!("7 % 2 (módulo):    {}", 7 % 2);     // 1 (resto)

    // 5. Booleanos y operadores lógicos
    println!("\n--- Booleanos ---");
    let edad = 20;
    let es_mayor = edad >= 18;
    let tiene_entrada = false;
    println!("¿Puede entrar? {}", es_mayor && tiene_entrada);
    println!("¿Es menor?     {}", !es_mayor);

    // 6. Caracteres Unicode (4 bytes, comillas simples)
    println!("\n--- Caracteres ---");
    let letra = 'ñ';
    let omega = 'Ω';
    let cangrejo = '🦀';
    println!("{letra} {omega} {cangrejo}");

    // 7. Tuplas (tipos distintos, tamaño fijo)
    println!("\n--- Tuplas ---");
    let jugador: (&str, u32, f64) = ("Ferris", 3, 98.5);
    println!("Acceso por índice -> Nombre: {}, Nivel: {}", jugador.0, jugador.1);

    let (nombre, nivel, puntuacion) = jugador; // Desestructuración
    println!("Desestructurado   -> {nombre} está en el nivel {nivel} con {puntuacion} puntos");

    // 8. Arrays (mismo tipo, tamaño fijo)
    println!("\n--- Arrays ---");
    let dias: [&str; 3] = ["lunes", "martes", "miércoles"];
    let ceros = [0; 5]; // Crea [0, 0, 0, 0, 0]
    println!("Primer día: {}", dias[0]);
    println!("Total días: {}", dias.len());
    println!("Array de ceros: {:?}", ceros);

    // 9. Conversión explícita con `as` (casting)
    println!("\n--- Casting con `as` ---");
    let total = 7;
    let partes = 2;
    let media = total as f64 / partes as f64;
    println!("Media exacta: {media}");        // 3.5
    println!("3.99 as i32:  {}", 3.99 as i32); // 3 (trunca)
    println!("'A' as u32:   {}", 'A' as u32);  // 65 (código Unicode)

    let grande: i32 = 300;
    println!("300 as u8:    {}", grande as u8); // 44 (desbordamiento al truncar bits)
}