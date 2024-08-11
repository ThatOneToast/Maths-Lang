
mod parser;



fn main() {
    let test_input_file_path = "/Users/toast/Documents/Maths/Maths/src/test.maths";
    let input = std::fs::read_to_string(test_input_file_path).expect("Failed to read file");
    


    let mut sequence = parser::parser::parse_expression_file(&input);
    println!("Result: {:?}", parser::parser::calculate_sequence(&mut sequence));
}


fn mini_interpreter(input: &str) {
    let mut variables = parser::parser::parse_expression_file(input);
    println!("Result: {:?}", parser::parser::calculate_sequence(&mut variables));
}   