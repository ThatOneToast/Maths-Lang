use std::{collections::HashMap, env, fs, path::PathBuf};



mod parser;
mod expressions;
mod tests;


fn main() {
    
    
    let mut home_dir_path = env::var("HOME").unwrap();
    if !home_dir_path.ends_with("/") {
        home_dir_path = format!("{}/", home_dir_path);
    }
    let maths_dir_path = format!("{}.maths/", home_dir_path);
    if !PathBuf::from(maths_dir_path.as_str()).exists() {
        fs::create_dir(maths_dir_path.as_str()).unwrap();
    }
    let maths_dir = fs::read_dir(maths_dir_path).unwrap();
    
    let mut maths_files: HashMap<String, String> = HashMap::new();
    
    
    for maths_file in maths_dir {
        let maths_file = maths_file.unwrap();
        let maths_file_path = maths_file.path();
        
        if maths_file_path.is_file() {
            if maths_file_path.extension().unwrap_or_default() != "maths" {
                continue;
            }
            let file_name = maths_file_path.file_name()
                .unwrap().to_str()
                .unwrap().to_string()
                .strip_suffix(".maths")
                .unwrap().to_string();
            let content = fs::read_to_string(maths_file_path).unwrap();
            maths_files.insert(file_name, content);
        }
    }
    
    println!("Maths files: {:?}", maths_files);

    let args: Vec<String> = env::args().collect();

    let mut arg_count = 1;
    let file_path = &args[arg_count];
    let content = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    let mut parser = parser::Parser::new(content.as_str(), Option::from(maths_files));
    
    if args.len() > 2 {
        let paramater_names = parser.get_paramater_names().unwrap();
        
        for param in paramater_names {
            arg_count += 1;
            
            
            let arg_value =  args[arg_count].parse::<f64>()
                .expect("Failed to parse argument");
            
            parser.var_container.numbers.insert(param.to_string(), arg_value);
        }
    }
        
    
    parser.parse();
}