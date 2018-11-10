
/* String demo:
    As in C/C++ non-constant strings are allocated in stack
    in this case the string is with autmoatic storage.
    Then is droped when is out of scope 
 */
fn main() {
    {  // as in c scopes are created with curly braces
        let mut s = String::from("First part");
        s.push_str("Second part");
        println!("{}",s);
    }
    println!("{}",s); // Error s does not exists
}
