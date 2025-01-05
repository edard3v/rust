# Tipos Complejos en Rust

## Strings

Rust tiene dos tipos principales de strings:

- `String`: Tipo dinámico, alojado en el heap, mutable y de tamaño variable
- `&str`: Slice de string inmutable, referencia a una secuencia UTF-8

## Tuplas

Colección de valores de diferentes tipos:

```rust
let tup: (i32, f64, bool) = (42, 3.14, true);
```

## Arrays

Colección de elementos del mismo tipo con longitud fija:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

## Vectores

Similar a arrays pero de tamaño dinámico:

```rust
let mut vec: Vec<i32> = Vec::new();
vec.push(1);
```

## Estructuras (Structs)

Tipo personalizado que agrupa datos relacionados:

```rust
struct Usuario {
    nombre: String,
    edad: u32,
    activo: bool
}
```

## Enumeraciones (Enums)

Tipo que puede ser uno de varios valores posibles:

```rust
enum Resultado {
    Exito(String),
    Error(u32)
}
```
