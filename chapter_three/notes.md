# Chapter Three

## Common Programming Concepts

### Variables

Rust favours immutability over mutability. A variable is said to be _mutatable_ if you can change the value assigned to it.

For example in Python, it's possible to do this:

```python
name = "Nick"
name = "Fish"
```

In this example, we have reassigned the value of name without any consequence. Doing something similar in a Rust program will definitely throw an error.

Example in Rust:

```rust
fn main() {
  let x = 5;
  println!("The value of x is: {x}");

  x = 6;
  println!("The value of x is: {x}");
}
```

This function will result in an error:

```text
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

Mutability can introduce bugs into your programs. Thankfully, the Rust compiler serves as a faithful watchdog that checks that things that aren't meant to change actually do not change, unless we explicitly state that it can be changed.

Rust variables can be made mutable by adding the `mut` keyword in front of the variable name.

For example:

```rust
fn main() {
  let mut x = 5;
  println!("The value of x is: {x}");

  x = 6;
  println!("The value of x is: {x}");
}
```

### Constants

Constants are values that are always immutable. Meaning, they never change. Once a constant value is set, `mut` cannot be used to change the value.

Unlike variables that are set using the `let` keyword, constants are defined using `const`

Finally, constants may be set to only a constant expression. Not a value that can only be computed at runtime.

```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *3
```

### Shadowing

When you redeclare a value, the original declaration is said to have been shadowed by the second one. It simply means the compiler will shadowed value within the scope where it is declared and used.

For example:

```rust
let x =5;

let x = x + 1;
```

In the example above, `x` is bound to the value 5. The later, it is redeclared and the new value becomes 6.

When we shadow, we create an entirely new variable. This allows us to not only reuse the variable name, but also change the type of value assigned.

## Data Types

Types are common in almost every pregramming language. We use types to different one value from another, thus, helping us to determine what operations are possible.

Rust is a statically types language, which means that it must know the types of all variables before compile time. There are two subsets of data types in Rust: scalar and compound types.

### Scalar Types

Scalar types in Rust includes: integers, floating point numbers, Booleans, and characters.

## Functions

Functions bodies are made up of a series of statements optionally ending in an expression.

- _Statements_ are instructions that perform some action and do not return a value
- _Expressions_ evaluates to a resultant value
