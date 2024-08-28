use std::{collections::HashMap, env, fs, io::{self, Read}, path::PathBuf};

use colored::Colorize;
use parser::{patterns::Pattern, Parser};

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
    let maths_dir = fs::read_dir(&maths_dir_path).unwrap();
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
    
    let mut patterns: HashMap<String, Pattern> = HashMap::new();
    
    for (name, content) in &maths_files {
        let parser = Parser::new(content.as_str(), None);
        let param_names = parser.get_paramater_names()
            .expect("This maths f");
        let params: Vec<&str> = param_names.iter().map(|s| s.as_str()).collect();
        let new_content = content.lines().skip(1).collect::<Vec<&str>>().join("\n");
        let pattern = Pattern::new(name.as_str(), params, new_content.as_str());
        patterns.insert(name.to_owned(), pattern);
    }
    

    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        panic!("No file path provided - Maths path/to/file.maths");
    }

    let mut arg_count = 1;
    
    let first = &args[arg_count];
    
    if first == "padd" {
        arg_count += 1;
        let file_path = &args[arg_count];
        let content = fs::read_to_string(file_path).expect("Failed to read the file");
        let temp_parser = Parser::new(content.as_str(), None);
        let param_names = temp_parser.get_paramater_names();
        match param_names {
            Ok(_) => {
                let file_name = file_path.split("/").last().unwrap().strip_suffix(".maths").unwrap();
                let dir = format!("{}/{}.maths", &maths_dir_path, file_name);
                fs::write(dir, content).expect("Failed to write the file");
                let message = "Pattern added to local standard library".bright_green();
                println!("{}", message);
                std::process::exit(0);
            }
            Err(_) => {
                panic!("This maths file doesn't contain any context of paramaters TOP OF LINE: #[params, params1]");
            }
        }
        
    }
    
    else if first == "premove" {
        let file_name = format!("{}.maths", args[2]);
        let file_path = format!("{}/{}", &maths_dir_path, file_name);
        fs::remove_file(file_path.trim()).expect("Failed to remove the file");
        let message = format!("Pattern removed from local standard library: {}", file_name).bright_green();
        println!("{}", message);
        std::process::exit(0);
    }
    
    else if first == "std" || first == "standard" {
        let mut builder = String::new();
        let header = "Standard Library:\n".bright_black();
        builder.push_str(&format!("{}\n", header));
        
        for (name, pattern) in parser::patterns::PATTERNS.iter() {
            let name_pattern = format!("[{}: {}]", name, pattern.context_params.join(", ")).bright_black();
            let mut usage = String::new();
            usage.push_str("\nUsage:");
            usage.push_str("\n```\n");
            usage.push_str(&pattern.construct_use_example());
            usage.push_str("\n```\n");
            builder.push_str(&format!("{} {}", name_pattern, usage.bold().bright_purple()));
        }
        
        let spacer = "Local Standard Library: \n".bright_yellow();
        builder.push_str(&format!("{}\n", spacer));
        
        for (name, pattern) in patterns {
            let name_pattern = format!("[{}: {}]", name, pattern.context_params.join(", ")).green();
            let mut usage = String::new();
            usage.push_str("\nUsage:");
            usage.push_str("\n```\n");
            usage.push_str(&pattern.construct_use_example());
            usage.push_str("\n```\n");
            builder.push_str(&format!("{} {}", name_pattern, usage.bold().bright_cyan()));
        }
        
        println!("{}", builder);
        std::process::exit(0);
    } else if first == "std-local" || first == "standard-local" {
        let mut builder = String::new();
        let header = "Local Standard Library:\n".bright_yellow();
        builder.push_str(&format!("{}\n\n", header));
        
        for (name, pattern) in patterns {
            let name_pattern = format!("[{}: {}]", name, pattern.context_params.join(", ")).green();
            let mut usage = String::new();
            usage.push_str("\nUsage:");
            usage.push_str("\n```\n");
            usage.push_str(&pattern.construct_use_example());
            usage.push_str("\n```\n");
            builder.push_str(&format!("{} {}", name_pattern, usage.bold().bright_cyan()));
        }
        
        println!("{}", builder);
        std::process::exit(0);
    }
    
    let content = if first == "interp" {
            // Read multi-line input from stdin
            println!("Enter your Maths Lang code (Press Ctrl+D or Ctrl+Z to end input):");
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            input
        } else {
            fs::read_to_string(first).expect("Failed to read the file")
        };

    let mut parser = parser::Parser::new(content.as_str(), Option::from(maths_files));
    let paramater_names = parser.get_paramater_names();
    
    match paramater_names {
        Ok(param_names) => {
            let param_size = param_names.len() + 2; // add additional 2 for the current dir and file path
            
            if args.len() != param_size {
                panic!("Expected {} argument(s), but got {} ", param_size - 2, args.len() - 2);
            }
            
            for param in param_names {
                arg_count += 1;
                
                
                let arg_value =  args[arg_count].parse::<f64>()
                    .expect("Failed to parse argument");
                
                parser.var_container.numbers.insert(param.to_string(), arg_value);
            }
        }
        Err(_) => {
            parser.parse();
        }
    }
    
    let result_value = parser.var_container.numbers.get("result").unwrap_or(&0.0).to_string();
    
    println!("result={}", result_value);
    
}


#[macro_export]
macro_rules! remove_whitespace {
    ($s:expr) => {
        $s
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
}