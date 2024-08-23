use std::collections::HashMap;

use types::VariableContainer;

use crate::expressions::evaluate_expression;
pub mod types;

pub struct Parser<'a> {
    pub contents: &'a str,
    pub patterns: HashMap<String, String>, // <name> <maths_content>
    pub var_container: VariableContainer,
}


impl<'a> Parser<'a> {
    pub fn new(contents: &'a str, patterns: Option<HashMap<String, String>>) -> Self {
        
        Self {
            contents,
            patterns: patterns.unwrap_or(HashMap::new()),
            var_container: VariableContainer::new(),
        }
    }
    
    pub fn get_paramater_names(&self) -> Result<Vec<String>, String> {
        let lines = self.contents.lines().collect::<Vec<&str>>();
        let first = lines.get(0).unwrap_or(&"");
        
        if first.starts_with("#[") {
            let params_string = first.strip_prefix("#[").unwrap_or("").strip_suffix("]").unwrap_or("");
            let params = params_string.split(",").collect::<Vec<&str>>();
            
            
            let new_params = params
                .iter()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            

            Ok(new_params)
            
        } else {
            Err("This maths doesn't contain any context of paramaters".to_string())
        }
        
    }
    
    fn get_pattern_paramaters_names(&self, pattern_name: &str) -> Vec<String> {
        let lines = self.patterns.get(pattern_name)
            .expect(&format!("Pattern {} doesn't exist", pattern_name))
            .lines()
            .collect::<Vec<&str>>();
        let first = lines.first().unwrap();
        
        if first.starts_with("#[") {
            let params_string = first.strip_prefix("#[").unwrap_or("").strip_suffix("]").unwrap_or("");
            let params = params_string.split(",").collect::<Vec<&str>>();
            
            
            let new_params = params
                .iter()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            

            new_params
            
        } else {
            panic!("This maths doesn't contain any context of paramaters");
        }
        
    }
        
    

    fn is_let_statement(&self, line: &str, tokens: Vec<&str>) -> bool {
        let first_token = tokens.get(0).unwrap_or(&"");

        if !first_token.starts_with("let") {
            return false;
        }

        let variable_name = tokens.get(1).unwrap_or(&"");

        if variable_name.is_empty() {
            panic!(
                "Let statement must be followed by a variable name, see {}",
                line
            );
        }

        true
    }

    fn is_variable(&self, token: &str) -> bool {
        self.var_container.get_number(token).is_some()
    }
    
    fn is_pattern(&self, token: &str) -> bool {
        self.patterns.get(token).is_some()
    }

    fn evaluate_condition(&self, condition_tokens: &[&str], num_vars: &HashMap<String, f64>) -> bool {
        let final_tokens = condition_tokens
            .into_iter()
            .map(|token| {
                if self.is_variable(token.to_owned()) {
                    format!("{}", *num_vars.get(token.to_owned()).unwrap())
                } else {
                    token.to_string()
                }
            })
            .collect::<Vec<String>>();

        println!("Condition tokens: {:?}", final_tokens);

        let expression = final_tokens.join(" ");
        let result = evaluate_expression(&expression);
        
        if result == 1.0 {
            true
        } else if result == 0.0 {
            false
        } else {
            panic!("Unexpected result from evaluate_expression: {}", result)
        }
    }

    fn parse_line(&mut self, line: &str) {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if self.is_let_statement(line, tokens.clone()) {
            let variable_name = tokens[1];
            
            println!("Tokenss {:?}", tokens);
            
            let mut remaining_tokens = tokens[2..]
                .iter()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            for (_, token) in remaining_tokens.iter_mut().enumerate() {
                
                if token.starts_with("@") {
                    let (pattern_name, mut passed_params) = token.strip_prefix("@").unwrap().split_once("(").unwrap();
                    println!("Pattern name: {}", pattern_name);
                    passed_params = passed_params.strip_suffix(")").unwrap();
                    println!("Passed params: {}", passed_params);
                    
                    let pre_passed_params_list = passed_params.split(",").collect::<Vec<&str>>();
                    let mut passed_params_list: Vec<String> = Vec::new();
                    
                    for pparam in &pre_passed_params_list {
                        if self.is_variable(&pparam) {
                            let value = self.var_container.get_number(&pparam).unwrap();
                            passed_params_list.push(value.to_string());
                        } else {
                            passed_params_list.push(pparam.to_string());
                        }
                    }
                    
                    let pattern_content = self.patterns.get(pattern_name)
                        .expect(&format!("Pattern {} doesn't exist", pattern_name));
                    let pattern_params = self.get_pattern_paramaters_names(pattern_name);
                    
                    if pattern_params.len() != passed_params_list.len() {
                        panic!("The number of passed params doesn't match the number of params in the pattern");
                    }
                    
                    let mut pattern_parser = Parser::new(pattern_content, Option::from(self.patterns.clone()));
                    
                    
                    let mut cursor: usize = 0;
                    for param in pattern_params {
                        let param_value_int = passed_params_list.get(cursor).unwrap().parse::<u8>().unwrap();
                        pattern_parser.var_container.numbers.insert(param.clone(), param_value_int as f64);
                        cursor += 1;
                        
                    }
                    
                    pattern_parser.parse();
                    let pattern_result = pattern_parser.var_container.get_number("result").unwrap();
                    *token = pattern_result.to_string();
                        
                    
                }
                
                if self.is_variable(token) {
                    *token = self.var_container.get_number(token).unwrap().to_string();
                } 
            }

            let expression = remaining_tokens.join(" ");
            let eval_expr = evaluate_expression(&expression);
            self.var_container
                .numbers
                .insert(variable_name.to_string(), eval_expr);
        } else if line.starts_with(";") {
            let var_name = line.strip_prefix(";").unwrap_or("");

            let var_value = self
                .var_container
                .numbers
                .get(var_name)
                .expect(&format!("Undefined variable: {} Line: {}", var_name, line));

            println!("{}: {}", var_name, var_value);
        } 
    }
    


    pub fn parse(&mut self) {
        let mut lines_iter = self.contents.lines().peekable();

        while let Some(line) = lines_iter.next() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            
            if line.starts_with("#") {
                continue;
            }

            if line.starts_with("if") || line.starts_with("???") {
                let condition_tokens = &tokens[1..tokens.len() - 1];
                let condition_met = self.evaluate_condition(condition_tokens, &self.var_container.numbers);

                let mut true_block_lines = Vec::new();
                let mut false_block_lines = Vec::new();
                let mut in_false_block = false;

                while let Some(block_line) = lines_iter.next() {
                    let block_line_trimmed = block_line.trim();
                    if block_line_trimmed == "}" {
                        if in_false_block {
                            break;
                        } else {
                            continue;
                        }
                    } 
                    
                    
                    if block_line_trimmed.starts_with("else {")
                        || block_line_trimmed.starts_with("!!!")
                        || block_line_trimmed.starts_with("} else {")
                    {
                        in_false_block = true;
                        

                        if let Some(next_line) = lines_iter.next() {
                            if next_line.trim() == "{" {
                                continue;
                            } else if next_line.trim() == "}" {
                                break;
                            }
                            
                        }
                    }

                    if in_false_block {
                        
                        if block_line.trim() == "} else {" {
                            continue;
                        }
                        
                        false_block_lines.push(block_line);
                    } else {
                        true_block_lines.push(block_line);
                    }
                }
                
                if condition_met {
                    if true_block_lines.is_empty() {
                        panic!("Empty true block");
                    }
                    for block_line in true_block_lines {
                        self.parse_line(block_line);
                    }
                } else {
                    if false_block_lines.is_empty() {
                        panic!("Empty false block");
                    }
                    for block_line in false_block_lines {
                        self.parse_line(block_line);
                    }
                }
            } 
            
            else {
                self.parse_line(line);
            }
        }

        let result_value = self.var_container.get_number("result").unwrap_or(&0.0);
        println!("Result: {}", result_value);
    }
}
