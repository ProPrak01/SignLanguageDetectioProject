
use fibrustlib;

use std::io;

fn main() {
    println!("Enter the number n->");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error reading");
    let n: usize = user_input.trim().parse().expect("Invalid");

    let ans = fibrustlib::fib_sec(n);
    println!("this is the sequence: {:?}", ans);
}
