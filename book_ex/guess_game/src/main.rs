use std::io;



fn main() {
    println!("Guess number\nEnter the guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Guess {}", guess); 
}
