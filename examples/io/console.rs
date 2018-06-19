//use std::fmt;

fn main(){
   // This is a common way to import only the names we will
   // use in the program. It's also possible to import all the 
   // pkg and then use name resolution or simply use a * to
   // throw all the names in this name scope.
   // use::io::*;
    use std::io::{stdin, stdout};
   // String formatting:
    print!("Multiline\
    String in soruce code\
    but not in output");
    print!("Also
    acceptable in this
    way");
    print!("{}:{}\n", "Sample", "string");
    print!("{} - {}, {1}\n", "First", "second");
    print!("{}:{}::{1}", "Sample", "string");
    println!("");
    println!("");
    // print numbers
    println!("Int Number: {}", 12);
    println!("Float number: {}", 3.1425928);
    println!("Hex 0x{0} to decimal number {1}", format!("{:x}", 0x10), 0x10);
    println!("Bin 0b{0} to decimal number {1}", format!("{:b}", 0b10), 0b10);
    // Getting user input:
        // Create a mutable object, by default the variables are non
        // mutable
    let mut s = String::new();
    print!("Give some input bitch:"); // this string is not flushed
    // Flushing the stdout, as in C/C++ the stdout is
    // flished only with new line or with a full buffer.
    // WTF this does not work??
    //stdout().flush();
    stdin().read_line(&mut s).unwrap();
    // What is the unwrap?
        /*

        */
    println!("Input string {}", s);
   // let mut line = String::new();
   // println!("{:?}", std::io::stdin().read_line(&mut line));
   // println!("[{}]", line)
}