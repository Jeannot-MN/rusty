# Rusty

## Chapter 1: Primitive Data Types
- **Integers**: 
Rust provide both signed (`i8`, `i16`, `i32`, `i64`, `i128`) and unsigned intergers(`u8`, `u16`, `u32`, `u64`, `u128`)


- **Floating-point numbers**: 
Decimal numbers(`f32`, `f64`)

- **Booleans**: 
`true` or `false`(`bool`)

- **Characters**: A single Unicode character (`char`), e.g. `'a'`

The are the 4 primitive data types in Rust.

---

## Chapter 2: Compound Data Types
- **Arrays**: 
Fixed size collection of elements of the same type(homogeneous).
`let numbers : [i32; 5] = [1,2,3,4,5]`: Notice that the size is specified on declation, 5 in this case.

- **Tuples**: 
Fixed size collection of heterogeneous elements.
`let human : (String, i32, bool) = ("Alice", 25, false)`

- **Slices**: 

- **Strings (Slice String)**:

---

## Appendix

### Command

- `rustc`: The Rust compiler used to compile Rust source code into an executable program.
- `cargo new`: Create a new cargo project.
- `cargo run`: Compile and Run a cargo project.