



fn main(){
    // for <var> in <init>..<endrange>
    for x in 1u8..20{
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
    'outer: loop{
        println!("Outer lup");
        'inner: loop{
            println!("Innnerloop");
            break 'outer;
            println!("Neva"); // this pops a warinig as we will never reach this satement
        }
    }
}