#[macro_use] extern crate text_io;


fn main() {

  
    let mut primes: Vec<u32> = vec![0, 1, 2];   //list of primes
    
    println!("Give me a max number...");
    let i: i32 = read!();  //read in user defined max number

    for _c in 0..i+1 {  //iterate from 0 to chosen number(i)
        for d in 2..primes.len() {
            if d as u32 % &primes[d] < 1 {
                primes.push(d as u32);
            };
        }   
        
        
    }    
    
    
    for _c in 0..primes.len() {
        println!("{}", primes[_c])
    }

    
    //println!("There are {} prime numbers between 1 and {}", primes.len(), i);
    
    
}