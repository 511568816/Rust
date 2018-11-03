extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guesse a interger");
    let secret_number = rand::thread_rng().gen_range(1,100);
    loop {
        println!("Please input your guess");
        let mut guess_number = String::new();
        io::stdin().read_line(&mut guess_number).expect("Fail to read\n");
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
