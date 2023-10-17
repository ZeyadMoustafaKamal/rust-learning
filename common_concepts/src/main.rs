use std::io;

fn main() {
    let mut input = String::new();
    let mut result = 0;
    println!("Please enter the number");
    io::stdin()
        .read_line(&mut input)
        .expect("An error happen");
    let input: u32 = input.trim()
        .parse()
        .expect("Please enter a valid number");
    
    for number in 0..input {
        result += number;
    }
    println!("The result is {result}");
}