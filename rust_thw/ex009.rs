// http://c.learncodethehardway.org/book/ex12.html

// use allows us to refer to methods without using their fully qualified names
// std::os contains the args() function. Not loger valid
use std::env;

fn main() {
    let argv = env::args(); // Is this a copy? a reference? 
    let argc = argv.len();

    //  == checks for equality
    if argc == 1 { println!("No args"); return;}
    if argc == 2 {
        println!("You only have one argument. You suck.");
        // && is the logical and operator
    } else if argc > 1 || argc < 4  {
        println!("Here are your arguments:");

        for arg in argv {
            print!("{} ", arg);
        }
        println!("");
    } else {
        println!("You have too many arguments. You suck.");
    }
}

// Questions and Exercises:
// 1.) Remove the else at the end so it won't catch the edge case.
// 2.) Change the && to a || so you get an "or" instead of "and" test and see how that works.
// 3.) You were briefly introduced to &&, which does an "and" comparison, so go google
//     the different "boolean operators"
// 4.) Go back to the two previous exercises, and use if-statements to make the loops
//     exit early. You'll need the break statement to do that. Go read about it.
// 5.) Is the first test really saying the right thing? To you the "first argument"
//     isn't the same first argument a user entered. Fix it.