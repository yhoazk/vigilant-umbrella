
// Gobal constants
//  They don't change during the life of the program
//  type has to be indicated 
static MAX_VAL: i32 = 20;
// as in C, & indicates a reference
static BIN_NAME: &str = "Loops";

/// Mark down comments have 3 /s
fn main(){
    // for <var> in <init>..<endrange>
    // The ranges have to have the same tipe
    println!("{}", BIN_NAME);
    for x in 1i32..MAX_VAL{
        print!("{}::",x);
        if x == 15 {
           break; // 99; // Break may take an expression argument
                     // which is the final result of the loop 
        }
    }
    // Range are [x,y)
    // an underscore can be used if we dont want the variable 
    for y in 0u16..10{
        print!("{}",y);
    }
    let mut n = 0;

    // while <exp> block
    while n < 5 {
        print!("n: {}", n);
        n+=1;
    }

    // For infinite loops use loop
    let mut k = 5;
    loop{
        k +=1;
        if k == 42{

            continue;
        }
        if k == 56 {
            print!("endd");
            break;
        }
    }
    // When loops are nested the break statement
    // normally breaks the closest loop, but in rust 
    // is possible to specify the loop that we want to 
    // break
    // named print arguments 
    let first: &str = "first";
    let sec: &str = "Sec";
    println!("Named Arguments Second: {sec} first {first}, again sec{sec}", first=first, sec=sec);
    'outer: loop{
        println!("Outer lup");
        'inner: loop{
            println!("Innnerloop");
            break 'outer;
            println!("Neva"); // this pops a warinig as we will never reach this satement
        }
    }
}