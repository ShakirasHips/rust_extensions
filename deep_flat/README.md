# deep_flat

`deep_flat` is a small Rust crate that provides a `DeepFlatten` trait for recursively flattening nested vectors into a single `Vec`.

It also includes a derive macro for quickly implementing `DeepFlatten` on custom types.

## Features

- Flatten deeply nested structures.
- Built-in support for common primitive types.
- Derive macro for custom structs and enums.


## Deriving `DeepFlatten`
```
#[derive(DeepFlatten)]
struct Point {
    x: f32,
    y: f32,
}
```


## Supported primitive types

The crate currently provides `DeepFlatten` implementations for:

- Signed integers: `i8`, `i16`, `i32`, `i64`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`
- Floating-point numbers: `f32`, `f64`
- `bool`
- `char`
- `String`