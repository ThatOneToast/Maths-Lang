use std::{collections::HashMap, iter::Map};

use crate::expressions::evaluate_expression;



pub struct VariableContainer {
    pub numbers: HashMap<String, f64>,
    
}

impl VariableContainer {
    pub fn new() -> Self {
        let mut nums = HashMap::new();
        nums.insert("restult".to_string(), 0.0);
    
        Self {
            numbers: nums,
        }
        
        
    }
    
    pub fn get_number(&self, key: &str) -> Option<&f64> {
        self.numbers.get(key)
    }
}

pub struct Parser<'a> {
    pub contents: &'a str,
    pub var_container: VariableContainer
}

impl<'a> Parser<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self {
            contents,
            var_container: VariableContainer::new(),
        }
    }
    
    fn is_let_statement(&self, line: &str, tokens: Vec<&str>) -> bool {
        let first_token = tokens.get(0).unwrap_or(&"");
        
        if !first_token.starts_with("let") {
            return false;
        }
        
        let variable_name = tokens.get(1).unwrap_or(&"");
        
        if variable_name.is_empty() {
            panic!("Let statement must be followed by a variable name, see {}", line);
        }
        
        true
    }
    
    fn is_variable(&self, token: &str) -> bool {
        self.var_container.numbers.contains_key(token)
    }
        
    
    pub fn parse(&mut self) {
        let mut lines_iter = self.contents.lines().into_iter().peekable();
        
        
        while let Some(line) = lines_iter.next() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            
            if self.is_let_statement(line, tokens.to_owned()) {
                let variable_name = &tokens.get(1).unwrap_or(&"");
                
                let mut remaining_tokens = tokens.get(2..).unwrap_or(&[]).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
                let mut rem_token_index = 0;
                
                if remaining_tokens.is_empty() {
                    panic!("Invalid let statement, see {}", line);
                }
                
                
                for token in remaining_tokens.to_owned() {
                    rem_token_index += 1;
                    println!("Token: {}", token);
                    
                    if self.is_variable(token.as_str()) {
                        let value = self.var_container.numbers.get(&token)
                            .expect(&format!("Undefined variable: {}", token));
                        
                        remaining_tokens[rem_token_index-1] = format!("{}", value);
                        println!("Remaining Tokens: {:?}", remaining_tokens);
                        
                    }
                }
                
                let expression = remaining_tokens.join(" ");
                let eval_exprs = evaluate_expression(expression.as_str());
                
                self.var_container.numbers.insert(variable_name.to_string(), eval_exprs);
                
            }
            
            else if line.starts_with(";") {
                // the next word can be like ;name ; name is the var name
                let var_name = line.strip_prefix(";").unwrap_or("");
                    
                let var_value = self.var_container.numbers.get(var_name)
                    .expect(&format!("Undefined variable: {} Line: {}", var_name, line));
                
                println!("{}: {}", var_name, var_value);
                break
                    
            }
            
            
        
        }
            
        
    }
    

    
    
}

