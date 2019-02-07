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
If it'))S needed to modify we need a vector or a new slice

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



Ownership and borrowing of `self` 

Rust diff w normal oo
Values can replace themselvs
Values have methods that consi,e 

IMplementations acn occur multiple times

Type `self`: The self is a special type in Rust, it always referenes the type to which the implementation refers
_No optional parameters_


Traints are between templates and interfaces

Trait bounds are for specify the data it uses, both of will interact with that type or in the case that needs to
know the size for example in containers

The rule for implementing traist

USCF: Unified function call.

Associated parameters: 


Rthreads:

`FnOnce` Moves the studf inside the thread, the types need to have no references. And the types need specific properties.
For move the contents of the referenced 

```
#![deny(missing_docs)] // The lint requires to add documentation
#![deny(unused_must_use)] // The level is raised from warn to err
use std::thread;

#[derive(Debug, Clone)]
struct Point{
    x: i32,
    y: i32
}

fn main(){
    let p = Point{x:1, y:2};
    for i in 1..3{
        let clone = p.clone(); // we need to cloine the contents of the point p as the thead takes owneship
        thread::spawn(move || {
            println!("{:?}", clone)
        })
    }
}
```

Linux handles are specific to a thread, cannot be re-used, the same applies to rust


- - - 
-
Conversions:

`From` and `into()`
```
let string: String = "foo".into(); // allocates foo is a string slice in the data section of the program
let string = String.from("foo");
```


`AsRef<T>` REference to reference conversion.

Box allocates on the heap, and is able to call inner methods by using the method `DeRef`.
Important deref:
- String -> &str
- VEc -> &[T]

`DeRef` is necessary for container types.


imports:

use the `use` command to import, glob imports are allowed but discoregend with the `*` commands
The only use case that is needded is for modules that implement the prelude.

```
use std::io::prelude::*;
```

Structured imports
use std::{}

rename import

use std::fs as fs_sys;

Imports can happen in specific scopes

In Rust everything is private by default, public are marked with `pub`.
Public types and fnc that can be reached throuh a public modeule path are exported
pub(crate)
pub(export) // forget the rule and make public to the external
pub(super)

Crates: Rust libraries





### Dynamic Dispatch

Look up in vtables for the address of the next function.
Java only makes dynamic dispatch

```rust
enum Operation{
    Get,
    Set(String),
    Count
} // cost is to check the discriminant

fn exec(op: Operation){
match op{
    Operation::Get =>
}
}
```

Alternative form:
Bounded: We know all the alternatives 

/// The borrowing needs to be specified in the impl
enun Operation

impl Operation {
match &self {
  &operation::Get =>
  &Operation::
}



```
// Debug is a trait
//  This generates only one function that takes a ptr to data and a vtable
// Needs to have the dyn and will select the function to call in runtime.
// It only accepts one trait, if we need 2 params we need to create a meta trait

fn print(debuggabke: &dyn Debug){
fn print(debuggabke: Box<dyn Debug>){
fn print(debuggabke: Rc<dyn Debug>){
  println!("{:?}", debuggabke)
}


// This version generates a function for every instance
fn print_static<D: Debug>(debuggabke: D){
  println!("{:?}", debuggabke)
}
```

For traits that are to be called
They are not allowes t ouse uppercase self
and the should not use generic functions themselfes

TODO: std:any:Any
Any is defined for any type, it TRIES the type and give us the result of the
downcast. 


### Life times

- `'v`: 
- `'a`
- `'v`


lifetime describe thetime that vals remain in mem. Thy describe- they cannot force or chane anything-
There is no change in compile time. The life time can be associate with the lifetime of a differnt
time given as a trait. Lifetime describe the minimal.

##### Multiple lifetimes in one signature

the compiler assumes taht everything is tied together, that every reference given hs the same life time
of the outgoing references.

```
fn main(){

let input = String::form("sdsdasd sdasd")
let splits: vec<&str> = input.split(2, ' ').collect(); // Create a view of the vector

drop(input) // drop the vector
println!("{:?}", splits); // try to print the reference but the vec is already drop
}
```


`'static`: Lifetime that lives longer that the others. Not necesariliy for ever. eg static string, globals
when is added `+ 'static` means that we are fully responsible and owner of this data that is being passedm




TAgs for FFI:

```
#[repr(C)]
#[repr(transparent)]
#[no_mangle]
```

The modifier `extern "C"` is neded to modify the layout of a function call
EXPLAIN WHY LEAK IS SAFE



