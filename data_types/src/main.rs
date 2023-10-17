use std::io;

fn main() {
    let a = [1, 2, 3, 4];

    println!("Please enter a number");
    let mut index = String::new();


    io::stdin()
        .read_line(&mut index)
        .expect("Something happend");

    let index : usize = index
        .trim()
        .parse()
        .expect("Please enter a number");

    let value = a[index];
    println!("The value is {value}")
}