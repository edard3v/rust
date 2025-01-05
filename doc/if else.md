# Control de Flujo: if else en Rust

## Sintaxis Básica

```rust
let numero = 5;

if numero < 10 {
    println!("Número pequeño");
} else {
    println!("Número grande");
}
```

## if else if

```rust
let temperatura = 25;

if temperatura < 0 {
    println!("Congelado");
} else if temperatura < 20 {
    println!("Fresco");
} else {
    println!("Cálido");
}
```

## If en una Línea

Rust permite usar if como expresión:

```rust
let condicion = true;
let numero = if condicion { 5 } else { 6 };
```

## Condiciones Múltiples

Usando operadores lógicos:

```rust
if edad >= 18 && tiene_licencia {
    println!("Puede conducir");
}

if es_admin || es_moderador {
    println!("Tiene permisos");
}
```

## Importante

- Las condiciones deben ser booleanas explícitas
- No se necesitan paréntesis alrededor de la condición
- Las llaves {} son obligatorias
