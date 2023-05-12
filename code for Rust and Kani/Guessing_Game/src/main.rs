#![allow(non_snake_case)]
use std::io; 
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    //Guess the number
    println!("Guess the number!");
    // now here the secret number printing that could be anything between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1 , 100);
    println!("Secret number {}" , secret_number);
    loop {
    println!("Input your guess!");
    //user has to guess number 
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("fail to load");
  //binding variable guess into expression guess.trim().parse() . trim will elimate any white space at the begining and end 
  // which must be able to compare string to u32, which can only contain numerical data
  // also handling invalid input with thw help of ok and Err enum
  //shadowing guess variable 
    let guess: u32 = match guess.trim().parse(){
         Ok(num) => num,
         Err(_) => continue,
    };
    println!("Your guess: {}" , {guess});
  // comparing secret number and guess number 
   match guess.cmp(&secret_number){

    Ordering::Less => println!("Number is small!"),
    Ordering::Greater =>  println!("Too Big number!"),
    Ordering::Equal=> { 
        println!("you win!");
        break;
   }
}
    }
}



