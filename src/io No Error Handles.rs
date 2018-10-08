use std::io::{self, BufRead, Write};

fn main() 
{
    let stdin = io::stdin();
    print!("Enter your name : "); 
    io::stdout().flush();
    let mut name = String::new();
    stdin.lock().read_line(&mut name);
    println!("Hello {} !", name.trim());
}