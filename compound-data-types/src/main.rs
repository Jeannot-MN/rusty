fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Numbers: {:?}", numbers);

    println!("The first number is {}.", numbers[0]);


    let human: (&str, i32) = ("Alice", 30);
    let complex_tuple : (&str, (&str, [i32; 5])) = ("Bob", ("Alice", numbers)); 

    println!("Human: {:?}", human);
    println!("Complex Tuple: {:?}", complex_tuple);
}
