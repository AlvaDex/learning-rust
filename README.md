# 🦀 Rust Learning (`aprenderust.dev`)

Repositorio personal para documentar, practicar y dar seguimiento al curso de Rust en español de [Aprende Rust](https://aprenderust.dev/), creado por `@midudev`.

---

## 🎯 Objetivo

Comprender la sintaxis, el modelo de memoria (**ownership & borrowing**), el sistema de tipos y la concurrencia en Rust mediante los 95 ejercicios interactivos de la plataforma, la resolución de sus 7 exámenes y la implementación de prácticas locales con `cargo`.

---

## 📂 Estructura del Repositorio

```text
rust-learning/
├── 01-primeros-pasos/
│   ├── hola-rust/
│   └── ejercicios/
├── 02-ownership-y-prestamos/
├── 03-structs-enums-y-patrones/
├── 04-colecciones-y-errores/
├── 05-genericos-traits-y-modulos/
├── 06-rust-funcional/
├── 07-concurrencia-y-ecosistema/
├── notas/
│   └── glosario-y-errores-frecuentes.md
└── README.md
```

---

## 🗺️ Temario y Control de Progreso

### Seccion 1: Primeros pasos
- [X] **Leccion 01:** Hola Rust y configuracion
- [X] **Leccion 02:** Variables y mutabilidad (`let`, `mut`, `const`)
- [ ] **Leccion 03:** Tipos de datos escalares y compuestos
- [ ] **Leccion 04:** Funciones, sentencias y expresiones
- [ ] **Leccion 05:** Control de flujo (`if`, `loop`, `while`, `for`)
- [ ] **Examen 01:** Evaluacion de Primeros Pasos

### Seccion 2: Ownership y Préstamos (El corazón de Rust)
- [ ] **Leccion 06:** Comprensión de Ownership (Reglas de propiedad y *Move*)
- [ ] **Leccion 07:** Referencias y Préstamos (`&` y `&mut`)
- [ ] **Leccion 08:** El tipo *Slice* (`&str` y vectores)
- [ ] **Examen 02:** Evaluacion de Ownership y Prestamos

### Seccion 3: Structs, Enums y Pattern Matching
- [ ] **Leccion 09:** Structs (Estructuras de datos personalizadas)
- [ ] **Leccion 10:** Métodos e implementación con bloques `impl`
- [ ] **Leccion 11:** Enums (Enumeraciones)
- [ ] **Leccion 12:** Control de flujo con `match` e `if let`
- [ ] **Leccion 13:** El enum `Option` y manejo de nulos
- [ ] **Examen 03:** Evaluacion de Structs y Enums

### Seccion 4: Colecciones y Manejo de Errores
- [ ] **Leccion 14:** Vectores (`Vec<T>`)
- [ ] **Leccion 15:** Cadenas de texto en profundidad (`String` vs `&str`)
- [ ] **Leccion 16:** Tablas Hash (`HashMap<K, V>`)
- [ ] **Leccion 17:** Manejo de errores recuperables con `Result<T, E>` y operador `?`
- [ ] **Leccion 18:** Errores irrecuperables con `panic!`
- [ ] **Examen 04:** Evaluacion de Colecciones y Errores

### Seccion 5: Genéricos, Traits y Módulos
- [ ] **Leccion 19:** Tipos de datos genéricos
- [ ] **Leccion 20:** *Traits* (Comportamiento compartido / Interfaces)
- [ ] **Leccion 21:** Introducción a *Lifetimes* (Tiempos de vida)
- [ ] **Leccion 22:** Sistema de paquetes, Crates y Módulos con Cargo
- [ ] **Examen 05:** Evaluacion de Genéricos y Traits

### Seccion 6: Rust Funcional y Smart Pointers
- [ ] **Leccion 23:** Closures (Funciones anónimas)
- [ ] **Leccion 24:** Iteradores y adaptadores
- [ ] **Leccion 25:** Punteros inteligentes (`Box<T>`, `Rc<T>`, `RefCell<T>`)
- [ ] **Examen 06:** Evaluacion de Rust Funcional

### Seccion 7: Concurrencia y Ecosistema
- [ ] **Leccion 26:** Concurrencia sin miedo (Threads, `mpsc`, `Arc`, `Mutex`)
- [ ] **Examen 07:** Examen Final del Creador y Certificacion

---

## ⏰ Metodología de Estudio Diario

Para maximizar la retención, cada día de práctica se divide en 4 bloques:

1. **Lectura y Playground (20 min):**
   - Leer la lección en `aprenderust.dev`.
   - Probar los ejemplos de código directamente en el navegador.
2. **Resolución de Ejercicios (25 min):**
   - Completar los ejercicios interactivos de la plataforma.
   - Analizar detalladamente cada mensaje de error generado por el compilador (`rustc`).
3. **Práctica Local con Cargo (30 min):**
   - Crear o modificar un proyecto en la carpeta del módulo correspondiente usando `cargo new`.
   - Modificar tipos, referencias o estructuras para experimentar fuera de los ejercicios predefinidos.
4. **Cierre y Documentación (15 min):**
   - Marcar con `[x]` las lecciones completadas en este `README.md`.
   - Registrar notas rápidas o conceptos difíciles en `notas/glosario-y-errores-frecuentes.md`.
   - Realizar `commit` y `push` a GitHub.

---

## 🛠️ Comandos Útiles

### Cargo (Gestor de paquetes de Rust)

```bash
# Crear un nuevo proyecto binario ejecutable
cargo new nombre_proyecto

# Compilar y ejecutar en modo desarrollo
cargo run

# Solo verificar si el código compila (mas rápido que run)
cargo check

# Compilar para producción (optimizado)
cargo build --release

# Ejecutar tests automáticos
cargo test
```

### Git (Flujo de trabajo diario)

```bash
# Guardar cambios locales
git add .
git commit -m "docs(rust): leccion XX completada y ejercicios resueltos"

# Sincronizar con GitHub
git push origin main
```

---

## 📝 Notas y Aprendizajes Clave

- **Impresión (`print!`, `println!`, `eprintln!`):** `print!` imprime texto sin salto de línea final, `println!` añade un salto de línea al final y `eprintln!` envía la salida al flujo de errores (`stderr`) en lugar de la salida estándar (`stdout`).
- **Variables e inmutabilidad:** Por defecto en Rust las variables son **inmutables** (`let`). Para modificar su valor se usa `mut` (`let mut`), manteniendo siempre el mismo tipo.
- **Constantes (`const`):** Representan valores fijos que no cambian (como límites o configuraciones). A diferencia de `let`, siempre exigen anotar su tipo explícitamente, pueden evaluarse en tiempo de compilación y se escriben en `SCREAMING_SNAKE_CASE` (mayúsculas con guiones bajos).
- **Shadowing (sombreado):** Permite declarar una nueva variable con el mismo nombre usando `let`, pudiendo incluso cambiar su tipo (por ejemplo, de `&str` a `usize`), sin necesidad de usar `mut`.
- **Ámbito de bloque (`{}`):** Las variables declaradas dentro de un bloque `{}` solo existen dentro de ese bloque y se destruyen al salir de él.
- **Legibilidad numérica:** Se puede usar `_` como separador visual en literales numéricos (`1_000_000` equivale a `1000000`) sin afectar el valor.
