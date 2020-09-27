use std::str;

fn draw_bug() {
let sparkle = vec![0xe0, 0xac, 0xb3];
println!("{}", str::from_utf8(&sparkle).unwrap());

}


fn draw_frame(h: u16, l:u16) {
let sparkle = vec![240,159,146,159];
println!("{}", str::from_utf8(&sparkle).unwrap());

}



fn format_test(){
  println!("Positional args {1} {0}", "First", "second");
}

// arrows start at e28690
// blocks start at e28c88
fn main() {
    println!("Hello, world!");
    draw_frame(4,4);
    draw_bug();
    format_test();
    let mut long_char = vec![0, 0, 0xcb, 0xa5];
    for n in 0..10 { 
        let c = str::from_utf8(&long_char).unwrap();
        println!("{}", c);
         long_char[3] = long_char[3] + 1;
 }
}
