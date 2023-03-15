extern crate rand;

use std::char::from_u32;

use rand::Rng;
use std::env;
fn main() {
    let password_length = get_password_length_from_args();
    let mut rng = rand::thread_rng();
    let mut result = String::new();

    for _ in 0..password_length {
        let number = rng.gen_range(48..122);
        let ch = from_u32(number).unwrap();
        result.push(ch);
    }

    println!("{}",result)
}


fn get_password_length_from_args() -> u8{
    let args: Vec<String> = env::args().collect();
    let mut password_length: u8 = 12;
    
    for arg in args {
        if !arg.starts_with("length=") { continue;}
        
        let split: Vec<&str> = arg.split("=").collect();
        let length_string = split.get(1).unwrap();
        match length_string.parse::<u8>() {
            Ok(number) => password_length = number,
            Err(e) => println!("Invalid argument error: {}",e),
        }
        
    }
    return password_length;
}