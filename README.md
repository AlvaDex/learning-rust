# 🦀 Rust Learning (`aprenderust.dev`)

Repositorio personal para documentar, practicar y dar seguimiento al curso de Rust en español de [Aprende Rust](https://aprenderust.dev/), creado por `@midudev` (26 lecciones, 95 ejercicios y 7 exámenes de sección).

---

## 🎯 Objetivo

Comprender la sintaxis, el modelo de memoria (**ownership & borrowing**), el sistema de tipos y la concurrencia en Rust mediante los ejercicios interactivos de la plataforma, la resolución de exámenes y la implementación de prácticas locales con `cargo`.

---

## 📚 Apuntes y Referencia

- [Glosario y errores frecuentes](./notas/glosario-y-errores-frecuentes.md)

---

## 📁 Estructura del Repositorio

```text
learning-rust/
├── 01-primeros-pasos/
│   ├── hola-rust/
│   ├── variables-mutabilidad/
│   └── tipos-de-datos/
│   └── control-de-flujo/
├── notas/
│   └── glosario-y-errores-frecuentes.md
└── README.md
```

---

## 🗺️ Temario y Control de Progreso

### Sección 1: Primeros pasos
- [x] Lección 01: Hola Rust y configuración
- [x] Lección 02: Variables y mutabilidad (`let`, `mut`, `const`)
- [x] Lección 03: Tipos de datos
- [X] Lección 04: Funciones y expresiones
- [X] Lección 05: Control de flujo (`if`, `loop`, `while`, `for`)
- [X] Examen de Primeros pasos

### Sección 2: Ownership y préstamos
- [ ] Lección 06: Ownership
- [ ] Lección 07: Referencias y préstamos
- [ ] Lección 08: Slices y `&str`
- [ ] Examen de Ownership y préstamos

### Sección 3: Structs, enums y patrones
- [ ] Lección 09: Structs
- [ ] Lección 10: Métodos con `impl`
- [ ] Lección 11: Enums y pattern matching
- [ ] Lección 12: `Option`
- [ ] Examen de Structs, enums y patrones

### Sección 4: Colecciones y errores
- [ ] Lección 13: Vectores (`Vec<T>`)
- [ ] Lección 14: Strings en profundidad
- [ ] Lección 15: `HashMap`
- [ ] Lección 16: Manejo de errores (`Result` y `?`)
- [ ] Examen de Colecciones y errores

### Sección 5: Genéricos, traits y módulos
- [ ] Lección 17: Tipos genéricos
- [ ] Lección 18: Traits
- [ ] Lección 19: Lifetimes
- [ ] Lección 20: Módulos, crates y Cargo
- [ ] Examen de Genéricos, traits y módulos

### Sección 6: Rust funcional
- [ ] Lección 21: Closures
- [ ] Lección 22: Iteradores
- [ ] Lección 23: Smart pointers (`Box`, `Rc`, `RefCell`)
- [ ] Examen de Rust funcional

### Sección 7: Concurrencia y ecosistema
- [ ] Lección 24: Concurrencia sin miedo
- [ ] Lección 25: Tests en Rust
- [ ] Lección 26: El ecosistema de Rust
- [ ] Examen de Concurrencia y ecosistema

---

## 🛠️ Comandos Útiles

```bash
# Crear un proyecto sin repositorio Git anidado
cargo new --vcs none nombre-proyecto

# Compilar y ejecutar
cargo run

# Revisar errores rápido sin generar binario final
cargo check

# Subir cambios a GitHub
git add .
git commit -m "feat(01): add practice and update progress"
git push
```