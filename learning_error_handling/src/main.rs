use std::fs::File;
use std::io::{self, Read};

enum custom_result<A, B> {
    Okay(A),
    Nope(B),
}

fn declare(number: i32) -> custom_result<String, i32> {
    if number == 0 { return custom_result::Okay("Zero".to_string()) }
    if number == 1 { return custom_result::Nope(number) }
    custom_result::Okay("This looks good".to_string())
}

fn read_username(path: String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;

    return Ok(username);
}

fn main() {
    let username = read_username("./hello.txt".to_string());
    match username {
        Ok(value) => println!("Username: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    let declaration = declare(2);
    match declaration {
        custom_result::Okay(value) => println!("Output: {}", value),
        custom_result::Nope(value) => println!("Nope output: {}", value),
    }
    
}

