#[macro_use] extern crate text_io;


fn main() {

    println!("Give me a number");
    let i: i32 = read!();
    println!("Your number was {}", i);
    
    print!("Now enter some text!\n");
    // reads until a \n is encountered

    let line: String = read!("{}\r\n");
    println!("{}", line);
}