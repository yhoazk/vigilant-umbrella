# vigilant-umbrella
Rust


## Install Rust


## Compile rust progams
The rustc command has the following format: `rustc [options] input`

General option:
* `-o <name>`: By default the executable is created with the same name as the
  input source, the option `-o` lower case o renames the file.
* `-O`: Produces a native code that is optimized for execution speed. Equivalent
  to the option `rustc -C opt-level=2`; The most optimized code is generated with
  `rustc -C opt-level=3`
* ``


## Debug Rust progams


## [Working with Cargo](https://doc.crates.io/guide.html)
Cargo is a package and dependency manager for Rust, similar to `pip` for python.

This tool does the following things for you:
  * Makes a tidy folder structure and some templates for your project
    * `cargo new <name>`
  * Comiles the code by using the command:
    * `cargo build` inside the project
  * runs the project by using:
    * `cargo run`
  * Runs the unit tests with the next command:
    * `cargo test`
  * If the project has dependencies, it will download them and build them with
  the command:
    * `cargo update`


## [Zinc]( http://zinc.rs/)

Zinc is a baremetal stack for running Rust in embedded environments, at this
moment only ARM is supported.


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
-
