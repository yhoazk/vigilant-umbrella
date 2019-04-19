
static NOT_USED : i32 = 45;

// fn format_me(s: &str, s2: &str)-> String{
//     //let mut k = String::from(s).push_str(s2);
//     //return k;
//     return "d";
// }
/* String demo:
    As in C/C++ non-constant strings are allocated in stack
    in this case the string is with autmoatic storage.
    Then is droped when is out of scope 

    There are 2 main string format placeholders
    {} and {:?}

    {} calls the display method, which is similar to the toString in java

    {:?} is the debug specifier, calls the Debug method which sometimes is not
    present. To add this method add the attribute #[derive(Debug)]
 */
fn main() {
    {  // as in c scopes are created with curly braces
        // let not_used2 = 45;
        let _ not_being_used = 0.2;
        let mut s = String::from("First part");
        s.push_str("Second part");
        // The place holder {} calls the method display, while the 
        // placeholder {:?} calls the method debug trait
        println!("{}",s);
    }
    //println!("{}", format_me("ee", "xx"));
    //println!("{}",s); // Error s does not exists 
}
