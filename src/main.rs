use std::io::{self, Write};
use std::string;

fn main() {
    let mut buffer = String::new();

    println!("Lispy version 1.0.1");
    println!("Press ctrl+c to exit!");

    loop {
        print!("lispy> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();
        
        println!("No you: {}", buffer.trim_end());
    }
}
