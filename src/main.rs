use std::{env, fs};



mod parser;
mod expressions;
mod tests;


fn main() {

    let args: Vec<String> = env::args().collect();

    let mut arg_count = 1;
    let file_path = &args[arg_count];
    let content = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    let mut parser = parser::Parser::new(content.as_str());
    
    let paramater_names = parser.get_paramater_names();
    
    for param in paramater_names {
        arg_count += 1;
        
        
        let arg_value =  args[arg_count].parse::<f64>()
            .expect("Failed to parse argument");
        
        parser.var_container.numbers.insert(param.to_string(), arg_value);
    }
        
    
    parser.parse();
}