


fn format_me(s: &str, s2: &str)-> String{
    return String::from(s).push_str(s2);
}
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
    println!("{}", format_me("ee", "xx"));
    //println!("{}",s); // Error s does not exists 
}
