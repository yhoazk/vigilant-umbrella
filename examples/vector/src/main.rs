fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8,9,10].into_iter();
    // Print the vector w/o loops
    println!("Numbers: {:?}", numbers);
    // Filter the numbers to get only the even ones
    let even = numbers.filter(|e| *e % 2 == 0);
    println!("Even Filter: {:?}", even);
    // The avobe print will print the operation as the filter is a lazy one 
    // which only applies the operations when asked
    let even_numbrs = even.clone().collect::<Vec<_>>();
    println!("{:?}", even_numbrs);
    // Now to apply an operation to every member of a vector
    // use the map operation
    let sqrd_even = even_numbrs.clone().into_iter().map(|x| x*x);
    println!("Before apply the mapping -> {:?}", sqrd_even);
    let sqrd_numbrs = sqrd_even.collect::<Vec<_>>();
    println!("After apply the mapping -> {:?}", sqrd_numbrs);
}
