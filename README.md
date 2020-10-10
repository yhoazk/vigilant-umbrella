# vigilant-umbrella
Rust

## Install Rust

Use the **rustup** tool chain:

```sh
curl https://sh.rustup.rs -sSf | sh
```

By default installs the stable version of the rust compiler, its package
manager and the language std library documentation locally. The installation
is placed by default under `~/.cargo` also updates the `PATH` variable.

To update the rust tool chain:
```sh
rustup update
rustup self update
```

## Install tools

```sh
cargo install racer && cargo install rustfmt && cargo install rustsym
```

### `racer`



### `rustfmt`

### `rustsym`


## Compile rust progams
The `rustc` command has the following format: `rustc [options] input`

General option:
* `-o <name>`: By default the executable is created with the same name as the
  input source, the option `-o` lower case o renames the file.
* `-O`: Produces a native code that is optimized for execution speed. Equivalent
  to the option `rustc -C opt-level=2`; The most optimized code is generated with
  `rustc -C opt-level=3`
* `-g`: Adds the debug information to the executable. `rust-gdb` needs to be installed to be able to use that informaiton.


## Debug Rust progams


## [Working with Cargo](https://doc.crates.io/guide.html)
Cargo is a package and dependency manager for Rust, similar to `pip` for python.

This tool does the following things for you:
  * Makes a tidy folder structure and some templates for your project
    * `cargo new <name>`
    * If we are to build an executable pass to cargo the option `--bin`
      * `cargo new <name> --bin`
    * Or the flag `--lib` if its a library
  * Comiles the code by using the command:
    * `cargo build` inside the project
  * runs the project by using:
    * `cargo run`
  * Runs the unit tests with the next command:
    * `cargo test`
  * If the project has dependencies, it will download them and build them with
  the command:
    * `cargo update`

### cargo files

#### `Cargo.toml` a.k.a. manifest
Is the configuration file or manifest of the project, contains all the metadata
that cargo needs to compile the project. This file follows the [TOML](https://github.com/toml-lang/toml) format

This files is editable, so other sections can be added. For example the name of
the binary:
```
[[bin]]
name = "<binary>"
```


## [Zinc]( http://zinc.rs/)

Zinc is a baremetal stack for running Rust in embedded environments, at this
moment only ARM is supported.

## The `let` statement

#### Syntax

```rust
let [mut] x[:t] = e;
```

* `t` specifies the type; often can be inferred if missing.
* `x` is immutable unless `mut` is present
* if `t` is present then `e:t` is required
* `x`:`t` is assummed in rest of scope, immutability is enforced.
* Assiging to a variable only allowed if `mut` is present.

##### Conditional initialization

```rust
let num = if cont {5} else {5}
```

## Ownership  

All programs have to manage the resources they require. Some use garbage
collectors which constantly looks for no longer used memory, this at run time;
others the programmer must explicitly allocate and free the memory.
Rust manages memory with a system of ownership which are checked at compile
time. None of the ownership rules affect the runtime of the program as the GC
does.

### The heap and the stack

Rust as a systems programming language whether a value is stored on the stack or
in heap has a greater impact on how the program behaves. Here are some notes on
the aspects relevant to stacks and heaps.

  * Stacks are faster as the data is known at compile time and has known size
  * The TOS (Top of stack) is always known
  * Data with a size unknown at compile time or a size that might change can be
  stored in the heap instead.
  * The heap is less organized
  * The OS has to find a space large enough for the requested data and return
  a ptr to it
  * Pushing values onto the stack is not considered as allocating, because the 
  pointer is already known, fixed size
  * Accessing data in the heap is slower than accessing data on the stack
  because a pointer has to be followed

### Ownership rules

* Each value has a variable that's called its owner
* There can be only one owner at a time
* When the owner goes out of scope, the value will be dropped

When the owner of a memory chunk goes out of scope, rusts calls a special
function called `drop`. Rust calls `drop` automatically at the closing curly
bracket.

> This is similar to the RAII pattern in C++

## Notes

- Rust does not perform garbage collection
- The compiler generates code that frees the resources at the __right__ time.
- Developed by Mozzila Research in 2006
- Rust has the built-in functionality necessary for concurrent execution.
- Imperative, structured and object oriented language.
- For concurrency uses the actor-model from Erlang.
- Its variables are immutable by default
- implements
  - Generics
  - Higher-order functions and closures
  - *Hygienic* macro system
  - Zero cost abstraction
- Rust does not have classes.
- Rust is not whitespace sensitive.
- Rust also uses `;` to indicate end of statement.


### Prelude
https://doc.rust-lang.org/std/prelude/index.html

