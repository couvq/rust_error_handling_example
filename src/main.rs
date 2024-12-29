use std::fs;

fn main() {
    println!("In file hello.txt");

    let contents = fs::read_to_string("hello.txt")
        .unwrap_or_else(|_| String::from("Failed to read file"));

    println!("With text:\n{contents}");
}