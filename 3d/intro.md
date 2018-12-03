# Intro 
rust in 3 days

The type system detecs concurrent accesss to data and requires sync
Rusts detects when unsinc access is safely possible
protection from data races
Static dispatch: Calls for v-table are static
(FFI) Foreing function interface, calling functions from a differente language

Rust is an expression-base lang. Each expressionhas a val which can be assigned or returned



---

Links:
https://rustup.rs
```
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
rustup install beta
```

`rustup doc` gives the documetation of the current version of rust
`rustup components list`: 

intall rustup component add rustfmt-preview clippy-preview  
- Rust Friends
-
```
cargo new hello-world --bin

cd hello-world
cat >src/main.rs
fn main(){
    println!("Hello world");
}
cargo build
cargo run
```

Intellij supports Rust with type definition


## Mutabilits

Is a prop of a var and binding, not the bound data.

Types:
* signed and unsigned: 8,16,32,64,128 sizes
* Arch dependent numbers: isize, usize

Array are always fiexed:

let arr: [i32; 4] = [1,2,3,4];
let arr: [i32] = &[1,2,3,4];

slice: a pointer and a length associated with it
cannot be extended or reduced, they are like views of an array.
If it')S needed to modify we need a vector or a new slice

```
&mut[i32] <- mutable slice but not the contents
```
The unit typeÖ:

```
fn main() -> (){
    42; // The semicolon makes this function to ignore the 42, if removed the fnc returns 42
}
```


## Structs

Definition:


The compiler can layout the memory as he wants, annotations need to be set if we need to rely on the 
memory layout.
```
struct Point { // Structs are upper case as guideline
    x: i32, // needs t obe spelled always
    y: i32, // always add the coma in the last statement
}
```

Allocation:

```
struct Point{
    x: i32,
    y: i32,
}

// rust allocations are by default in the stack
// rust does not have a heap by design

fn main(){
    lent p = Point {x:1, y:4};
    //access the syntax
    // Locks stdout, formatting, unlock stdout
    println!("{}", p.x);
}
```

Tuples:

```
fn main() {
    let p = (1,2); // the fields can be accessed by number
    p.0;
    p.1;
    println!("{}", p.1);
}
```

Tuple structs:

Enums:

```
enum Direction {
    Right,
    Left,
    Up,
    Down
}

fn main(){
    let direction = Direction::Left;
}
```

The different choices are called variants. This variants can have a value.

```
// Equality is not defined here

enum Movement{
    Right(i32),
    Left(i32),
}


```

Enums with strucutred variants


```
enum actions {
    stickaround,
    Moveto {x:i32, y : i32},
}

let main(){
    let action = Actions::moveto{x:0, y:0};
}
```

`null` does not exists in rust, refernces cannot be `null`. Always point in memory and legal and pointing to something

Nothing inthe function can be inferred, everything has to be dlecared.
The return is no semicolon, is recomended to not use return.

Special enums `Result`

```
enum Result<T,E>{
ok(T),
err(E),
}

fn main(){
    let file = std::fs::File::open("Idont exists");
    // we cannot get the result with out the match
    match maybe_file{
        ok(fike) =>{println("fiel openend");},
        err(e) => {println!("{:?}", e)}
    }
}
```


`#[derive(DEbug)]` 

The variables are "consumed", then the owneship is _moved_ to the first function.
The movement makes the variable owned with other variable.
This errors are guaranteed to be catched in compile time

The vars can be cloned in any type by enabling the clone `#[derive(Debug, Clone)]`

```
struct Dot{
    x:i32,
    y:i32,
}

let dot = Dot {x:1, y:2};
pacman(dot.clone());

fn pacman(dot: Dot){
    println!("Eating {:?}", dot);
}
```




Copy is a memcopy, this means that the contents of the memory are the same, the clone call
are can be implemented as an interface, for example for a file descriptor or socket.

Copy is meant for data that can be quicly copied in memory usnig memcpy and are allowed to
be copied eg not file pointers

There's no innmovable types, objects can be pinned but just until the next releases.
Copy works only on plain data.

When a file is dropped is get automatically closed.



Simple borrowing (&) works like C
mutable borrowing are exlusive. Pointer will never be aliased, thi means that only 1 can get it at a time.
The borrower cannot be used multiple times, only once per instance be used multiple times, only once per instance

Its not possible to create a second mutable ppointer when its already borrowed


# box

Box intergers are in the heap in java
The same is in rust, with the Box type, which is guaranteed to be in the heap.
The ownnershp of the data passed to the box is retained

# Strings

slices borrow, vectors own.

# Iterators
Rsut iters:
- Lazy: like yield
- pervasive
- potetially inf

Three types of iterators:
- mutable: `iter_mut`
- borrowing: `iter()`
- owning: `iter<vec<i32>>`


iternators implement the function `filter()`

The for loop takes ownership of the iterator.

```
fn main(){
    let events = (0..10_000).filter(|x| x % 2 == 0);
    for item in events{
        println!("{}", item)
    }
}
```

Rust bound checks. If an array or vector is accessed out of boundaries the compiler will painc.
default enondign in the OS. win utf-16. macos broken utf-8. linux? in sockets.

panics are proper way of handling errors, they close all the used resources.


To add testsing the macro `#[test]` and the test can be run with `cargo test`

get a slice after the length of the string but inside the capacity

Mozilla `rr` tool for re-run error.
