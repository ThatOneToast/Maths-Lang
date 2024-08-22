

mod parser;
mod expressions;
mod tests;


// use std::io::{self, Read};

// fn main() {
//     println!("Welcome to Maths!");
//     println!("When you are finished with your maths script, Unix: Ctrl+D, (Shitty) Windows: Ctrl+Z");
    
//     loop {
        
//         let mut input = String::new();
    
//         print!("Maths> ");
//         io::stdin().read_to_string(&mut input)
//             .expect("Failed to read input");
        
//         let mut parser = parser::Parser::new(input.as_str());
//         parser.parse();
//     }
// }


fn main() {
    const TEST_FILE: &str = r#"
    
            
if 40 > 30 {
    let result = 5
    let result = 50 + result
} else {
            let result = 100
}
    
    ;result

        "#;
    
    println!("Welcome to Maths!");
    let mut parser = parser::Parser::new(TEST_FILE.trim());
    parser.parse();
}