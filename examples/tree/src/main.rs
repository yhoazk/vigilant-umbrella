use std::io::{stdin, stdout};
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let mut x = 0;
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    //x = i32::from_str(&input).unwrap();
    println!("{}", input);
    x = input.parse::<i32>().unwrap();

    loop {
        println!("Hello, world!");
        if(x > 0){
            x = x-1; 
        }
        else {
            break;
        }
       // unimplemented!(); This macro ends the program with an error message
    }
}
