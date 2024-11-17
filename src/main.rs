use std::{io::{self}, num::ParseIntError};

fn main() {
    println!("Welcome to Encrypto! Encrypt your txt files to be stored on any platform you want");
    menu();
}

fn menu() {
    println!("Menu:");
    println!("1) Encrypt File");
    println!("2) Decrypt File");
    println!("Any other number to exit");
    match read_numerical_input() {
        Ok(val) => {
            match val {
                1 => encrypt(),
                2 => decrypt(),
                _ => println!("Exiting..."),
            }
        }
        Err(e) => {
            println!("Error parsing integer from the input: {}, Exiting...",e);
        }
    }
}

fn read_numerical_input() -> Result<i64, ParseIntError> {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    value.trim().parse::<i64>() //return either a i64 or ParseInt error as a result
}

fn read_file() {

}

fn write_file() {

}

fn encrypt() {
    println!("encrypt!");
}

fn decrypt() {
    println!("Decrypt!");
}
