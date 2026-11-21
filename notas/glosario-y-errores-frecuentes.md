# 📖 Glosario y Errores Frecuentes de Rust

Notas de referencia rápida basadas en las prácticas de [Aprende Rust](https://aprenderust.dev/).

---

## 📝 Glosario de Conceptos

### Salida y consola
- **`println!` / `print!` / `eprintln!`:** Macros para imprimir texto. `println!` agrega salto de línea al final, `print!` no agrega salto de línea y `eprintln!` escribe en la salida de errores (`stderr`) en lugar de la salida estándar (`stdout`).

### Variables, mutabilidad y constantes
- **`let` (inmutabilidad por defecto):** Las variables en Rust son inmutables a menos que se indique lo contrario.
- **`mut` (`let mut`):** Permite modificar el valor de una variable manteniendo siempre su mismo tipo. Úsalo solo cuando sea necesario.
- **`const`:** Valor fijo que no cambia; exige anotación explícita de tipo, puede evaluarse en tiempo de compilación y se escribe en `SCREAMING_SNAKE_CASE`.
- **Shadowing (sombreado):** Volver a declarar una variable con `let` en el mismo ámbito (o en un bloque interno `{}`), pudiendo incluso cambiar su tipo (por ejemplo, de `&str` a `usize`) sin usar `mut`.
- **Ámbito de bloque (`{}`):** Las variables creadas dentro de `{}` solo viven hasta que cierra la llave.
- **Separador numérico (`_`):** Permite escribir literales numéricos legibles como `1_000_000` o `1_500` sin alterar su valor.

### Tipos escalares
- **Enteros:** Con signo (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) y sin signo (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`). Por defecto Rust infiere `i32`. La división entera trunca decimales (`7 / 2 == 3`).
- **Métodos anti-desbordamiento (*overflow*):** `wrapping_add()` da la vuelta al rango permitido; `saturating_add()` se detiene en el valor mínimo o máximo del tipo.
- **Flotantes:** `f32` y `f64` (por defecto `f64`).
- **Booleanos (`bool`):** Valores `true` o `false`, combinables con `&&`, `||` y `!`.
- **Caracteres (`char`):** Entre comillas simples (`'a'`, `'ñ'`, `'Ω'`, `'🦀'`); ocupan 4 bytes y representan un valor escalar Unicode.

### Tipos compuestos y conversión
- **Tuplas (`(T1, T2, ...)`):** Agrupan valores de distintos tipos con tamaño fijo. Se acceden por índice (`tupla.0`) o mediante desestructuración (`let (a, b) = tupla;`).
- **Arrays (`[T; N]`):** Colección de tamaño fijo con elementos del mismo tipo en el *stack*. Se imprimen en formato depuración con `{:?}`.
- **Conversión explícita (`as`):** Rust no convierte tipos numéricos implícitamente; `as` fuerza la conversión (`total as f64`), pudiendo truncar decimales (`3.99 as i32 == 3`) o desbordar bits en tipos estrechos (`300_i32 as u8 == 44`).

### Funciones y expresiones
- **Expresiones vs. sentencias:** Una **expresión** evalúa a un valor y **no** lleva punto y coma (`;`) al final (`x + 1`). Agregar `;` convierte una expresión en una **sentencia**, cuyo valor es `()` (unidad) y puede provocar un error de tipos (`E0308`) si esperabas devolver un valor.
- **Retorno de funciones:** La última expresión del cuerpo de una función se devuelve implícitamente sin necesidad de escribir `return`. Usa `return` para salidas anticipadas (por ejemplo, devolver `0` si el divisor es `0`).
- **Tipo unidad `()`:** Es el valor por defecto que retornan las funciones sin anotación `-> Tipo` (o con `-> ()`); representa la ausencia de un valor útil.
- **Parámetros y retornos tipados:** En Rust la firma de una función siempre exige declarar el tipo de cada parámetro (`nombre: &str, puntos: u32`) y, si retorna algo distinto de `()`, el tipo tras `->`.

---

## ⚠️ Errores Frecuentes del Compilador

### Cannot assign twice to immutable variable (`E0384`)
- **Causa:** Intentar reasignar una variable declarada con `let` sin `mut`.
- **Solución:** Cambia `let x = ...` a `let mut x = ...` si el valor debe cambiar, o usa *shadowing* (`let x = ...`) si vas a crear un nuevo enlace.

### Mismatched types (`E0308`)
- **Causa:** Asignar o comparar valores de tipos incompatibles (por ejemplo, asignar un string a una variable inferida como `i32`, o sumar `i32` con `u32` sin convertir).
- **Solución:** Verifica la anotación de tipo o convierte explícitamente con `as` cuando corresponda.

### Integer overflow / panic o resultado inesperado en `as`
- **Causa:** Superar el rango de un tipo entero (`u8` va de `0` a `255`) o forzar `as` hacia un tipo más pequeño.
- **Solución:** Usa `wrapping_add` / `saturating_add` según el comportamiento deseado, o elige un tipo con rango suficiente (`u32`, `u64`, `i32`).