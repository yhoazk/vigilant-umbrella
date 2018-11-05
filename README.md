# vigilant-umbrella
Rust

## Install Rust

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
