// Generating the nth fibonnaci number
use std::io;

fn fibonnaci(n: u128) -> u128 {
    if n == 0{
        0
    } else if n == 1 {
        1
    } else {
        fibonnaci(n-1) + fibonnaci(n-2)
    }
}

fn main() {
    println!("Generating Fibonnaci numbers. ");
    println!("Enter a number: ");

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: u64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
    };
    let answer = fibonnaci(num.try_into().unwrap());

    println!("Fibonacci number at position {num} is: {answer}");
}