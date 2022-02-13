# Day 1 

## Build Tools
rustc and cargo  are important tools that every rustaceans must know.

rustc - Compiles the code
cargo - It is build system & package manager 

```

cargo --version

```

## Cargo in Action

```
cargo new hello
cargo build 
cargo run
cargo check
```

## Key concepts 

- By default variables are immutable. It is declared using 'let' keyword.
- Variable can be marked as mutable by
```
let mut guess = String::new(); // Allocates new String
```
- Compiler does type inference to avoid verbose typing of Type when declaring variable. It is same as functional programming langs.
```
let x = 10 ;// This is int
```
- Rust is helping programming by making sure all errors are handled at the time of writing code

```
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // This line is forcing to handle error. Code will not compile!
```
- Associated Function call are made with '::' 
```
String::new
```
Associated functions are defined on particular type. It is also like static functions if coming from java world or class level functions.
- Regular function are called with "."
```
io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
```
- Macro are called using '!'
```
 println!("Lets play guessing game!!!");
 println!("You guess it {}", guess);
```
- cargo.toml

This file is created by caro and contains package details and dependency
```
[package]
name = "hello"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.3"

```
- 

