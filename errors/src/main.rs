use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username is {}", username),
        Err(e) => println!("Error: {}", e),
    }
}
