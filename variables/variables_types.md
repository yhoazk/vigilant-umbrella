# Variables and types

### Comments

As is `C/C++` single line comments start with `//` and finish with the line. And
multiline comments start with `/*` and end with `*/`.

Rust also has the doc comment `///` like in java `/**`. Useful for large programs
which require official documentation. This comments have to appear before an item
on a separate line to document that item. In these comments is possible to use 
Markdown formatting.


### Global constants

This global constants are declared with the keyword `static`, their names must
be uppercase and underscores can be used to separate words. Their type must be
indicated, for the case of the next staing, which is a reference their lifetime
specifier must also be included. The life time of an object is important in
rust as it says how long the object will live in the program's memory. The Rust
compiler adds code to free that memory after the life of an object ends.
The `'static` lifetime is the longest possible lifetiem. Such object stays
alive throughout the entire application, and is available to all of its code.
```
static MAX_LEVEL: i32 = 100; // Constant integer of 32 bits
static GAME_NAME: &'static str = "Monster Attack"; // Constant string
const PI: f32 = 3.14; // more local without the static specifier.
```

### Printing variables

As in Python, formtatting for printing variables we can use
the `{}` place-holder to expand the value of a variable, for example:
```rust
const PI: f32 = 3.14;
println!("PI: {}", PI);
```

There can be more that one variable, and again this works as
in python. The first variable to be replaced is `{0}` the 
second `{1}` and so on. Example:
```rust
const FIRST: i32 = 5;
const SECND: f32 = 5.44;
println!("First value: {0}, second: {1}, First Again: {0}", FIRST, SECND);
```

The place holder can also be named.
```rus
println!("You have {points} %", points=20);
```
Special ways of formatting can be indicated inside `{}` 
after a colon as follows:

```rust
const MAX: i32 = 100;
const PI: f32 = 3.141592;
println!("MAX is {:x} in hex", MAX); //100
println!("MAX is {:b} in bin", MAX); //1100100
println!("pi is {:e} in floating point notation", PI);
```

Here is a list of the possible types:
* `o` for octal
* `x` for lowercase hex
* `X` for uppercase hex
* `p` for a pointer
* `b` for binary
* `e` for lower exponential notation
* `E` for upper exponential notation
* `?` for debugging purposes

NOTE: The macro `format!` has the same parameters and works in the same way 
that `println!`, but returns a string instead of printing out.


## Values and primitive types

In rust the type `char` is unicode(Unicode UTF8 by default), which take 4 bytes.
There is also `bool` type.

Integers can be represented in all the bases, and they are expressed as in verilog:
```
0x46
0o106
0b1_0000_0000 or 0b100000000 // underscores can be used for clarity.
```

The type can also be added to a number as a suffix:
* `10usize` denotes an unsigned integer of machine word size, which can be any
of the `u8`, `u16`, `u32` or `u64` types.
* `10isize` denotes a signed integer of machine word size, which can be any
of the `u8`, `u16`, `u32` or `u64` types.
* In the preceding cases, for a 64-bit operating system `usize` is in fact `u64`
and `isize` is equivalent to `i64`
* `3.14f32` denotes a single precision float.
* `3.14f64`denotes a double precision float.

By default `i32` and `f64` are the defaults if no suffix is given, the point
is used to differentiate between them.


* Rust uses the same binary operators, but it does NOT have `++` nor `--` 
operators.
* To compare for equality use `==`
* To compare for inequality `!=`
* For functions which return nothing, not even null, Rust implements the  empty
value `()` with size 0.



## Binding variables to values

* Rust uses the `let` keyword as in JS.
* When a variable is not used, the rustc will print a warning leting us know 
that. For prototyping purposes, the warning can be suppresed by prefixing the
variable name with a `_` e.g. `let _ energy = 5;` In general, `_` is used for
variables that we don't need.

As on Go the compiler inferred the type of the variable. For example:
```rust
let tile_str = "Title: 1"; //str inferred
let dead = false; // bool inferred 
let pi = 3.1415f32; // single precision assigned
let pi = 3.1415_f32; // single precision assigned
let empty = (); // assigned the unit type.
```

## Mutable and immutable variables

#### What does it mean to be mutable?
[Long read](https://doc.rust-lang.org/book/first-edition/mutability.html)


Variables are y default **immutable** in Rust, which is very similar to what
funcitonal languages do. In pure functional languages, mutability is not even
allowed.

Example:
```rust
fn main() {
    let x =5;
    x = 10; /*error: re-assignment of immutable variable `x`*/
}
```

If we want a mutable variable because its value can change during code execution,
you have to indicate that explicitly with the keyword `mut`. i.e.:
```rust
let mut fuel = 34;
fuel = 60;
```

## Scope of a variable and shadowing

As in C the scope is limited by the `{}`. We can even create a scope with just
a pair of `{}`.

```rust
fn main() {
    let outer = 42;
    { // Brackets create a new scope, aka code block
        let inner = 3.14;
        println!("Block var: {}", inner);
        let outer = 99; //shadows the first outer var
        println!("Block variable outer: {}", outer);
    }
    println!("outer var: {}", outer);
}
```


## Type checking and conversions

Rust checks the type of the variables at compile time. This means that we
cannot change the type of a variable during its lifetime because of static 
typing.

```rust
fn main()
{
    let foo: i32 = 100;
    foo = "bar"; // Error: mismatched types: expected int found &static str
    /* However is possible to do the following */
    let foo = "bar"; // This will create a new variable the old foo will be
                     // deleted

}
```

### Casting

Rust warns about every change of type with compiler warnings. But if it's
intended Rust provides the operator `as`.

```rust
let pts = 10i32;
let mut other_pts: u32 = 0;
other_pts = pts as u32;
```

The operator `as`:
* truncates floats to only the integer part
* Strings cannot be converted to integers

### Aliasing

This has the same purpose as the `typedef` in C. It assigns a new name to
a known type.

```rust
type uint_16 = u16;

fn main {
    let my_int: uint_16 = 943;
}
```

### Expressions

#### Expressions and statements

What is an expression?:

What is an statement?: