use std::io;
use rand::Rng;
use std::cmp::Ordering;
// Ordering is an enum with 3 variants
// the cmp method compares the input and returns
// one of the variants to be matched


fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess number\nEnter the guess");
    loop {

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // shadow the variable guess from str to int
        let guess : i32 = guess.trim().parse().expect("Error parsing input");
        println!("Guess: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Great"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}
