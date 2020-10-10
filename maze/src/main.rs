use std::str;
use std::fmt;

fn draw_bug() {
let sparkle = vec![0xe0, 0xac, 0xb3];
print!("{}", str::from_utf8(&sparkle).unwrap());

}


fn draw_frame(_h: u16, _l:u16) {
let sparkle = vec![240,159,146,159];
println!("{}", str::from_utf8(&sparkle).unwrap());

}

fn printerr() {
  eprint!("Error");
  eprintln!("new line error");
}

/* Aligns $number to the right with a total of $width chars */
fn print_options_aling(){
  println!("{number:>width$}", number=1, width=4);
}

/* align $number to the right with total of $width chars filling with -'s */
fn print_options_align_l(){
  println!("{number:->width$}", number=2, width=4);
}

fn print_byname(){
  println!("Some {arg0} other {arg1} third {arg2}", arg0="primer", arg1="Second", arg2="dritte");
}

fn decimal_places(){
  println!("Pi {pi:.2}", pi=3.141592);
  println!("Pi {pi:.n$}", pi=3.141592, n=3);
}

fn format_test(){
  println!("Positional args {1} {0}", "First", "second");
  
}

fn draw_startofbox(){
  let mut chars = vec![0xe2, 0x94, 0x80];
  for _ in 0..20 {
    print!("{} ", str::from_utf8(&chars).unwrap());
    chars[2] = chars[2] + 1;
  }
}
// arrows start at e28690
// blocks start at e28c88
// 
#[derive(Debug)]
struct Debuggable(i32);
fn print_debuggable(){
  // Default formatter does not apply println!("Debuggable {}", Debuggable(3));
  println!("Debuggable {:?}", Debuggable(3));
}
/* This will print the data of the struct with name */
#[derive(Debug)]
struct Person<'a>{
  name: &'a str,
  age: u8
}
fn pretty_print(){
  let name = "lena";
  let age = 44;
  let lena = Person {name, age};
  println!("{:#?}", lena);
}


/* Implement a user defined printer of a type */
#[derive(Debug)]
struct Cmplx(i32, i32);
impl fmt::Display for Cmplx {
  fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, j{})", self.0, self.1) // remember to remove semicolon
  }
}

#[derive(Debug)]
struct Point3D{
  x: f64,
  y: f64,
  z: f64
}

impl fmt::Display for Point3D {
  fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
  }
}


// Difference between struct () and struct {}?
fn test_cmplx(a:i32, b:i32){
  let cmp = Cmplx (a, b);
  println!("cmplx {}", cmp);
}

fn test_point3d(x: f64, y : f64, z: f64) {
  println!("A point {}", Point3D{x:x,z:y,y:z});
}

/* drawing box characters start at e29480 */
// https://en.wikipedia.org/wiki/Box-drawing_character
fn main() {
    println!("Hello, world!\u{250c}");
    println!("\u{250c}\u{2500}\u{2510}");
    print!("\u{2502}");
    draw_bug();
    println!("\u{2502}");
    println!("\u{2514}\u{2500}\u{2518}");
    printerr();
    draw_frame(4,4);
    draw_bug();
    format_test();
    draw_startofbox();
    print_byname();
    print_options_aling();
    print_options_align_l();
    decimal_places();
    print_debuggable();
    pretty_print();
    test_cmplx(3,  32);
    test_point3d(3.2, 4f64, 64f64);
 }
