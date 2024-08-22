
mod parser;
mod expressions;
mod tests;


use std::io::{self, Read};

fn main() {
    println!("Welcome to Maths!");
    println!("When you are finished with your maths script, Unix: Ctrl+D, (Shitty) Windows: Ctrl+Z");
    
    loop {
        
        let mut input = String::new();
    
        print!("Maths> ");
        io::stdin().read_to_string(&mut input)
            .expect("Failed to read input");
        
        let mut parser = parser::Parser::new(input.as_str());
        parser.parse();
    }
}
