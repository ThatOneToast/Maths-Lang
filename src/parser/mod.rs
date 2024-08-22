use std::collections::HashMap;

use types::VariableContainer;

use crate::expressions::evaluate_expression;
pub mod types;

pub struct Parser<'a> {
    pub contents: &'a str,
    pub var_container: VariableContainer,
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

    fn evaluate_condition(&self, condition_tokens: &[&str], num_vars: &HashMap<String, f64>) -> bool {
        let final_tokens = condition_tokens.into_iter()
            .map(|token| {
                if self.is_variable(token.to_owned()) {
                    format!("{}", *num_vars.get(token.to_owned()).unwrap())
                } else {
                    token.to_string()
                }
            }
        ).collect::<Vec<String>>();
            
        
        println!("Condition tokens: {:?}", final_tokens);
        
        let expression = final_tokens.join(" ");
        let result = evaluate_expression(&expression);
        result > 0.0 // Assuming your evaluate_expression returns a number, treat > 0 as true
    }

    fn parse_line(&mut self, line: &str) {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if self.is_let_statement(line, tokens.clone()) {
            let variable_name = tokens[1];

            let mut remaining_tokens = tokens[2..]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            for (_, token) in remaining_tokens.iter_mut().enumerate() {
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

            if line.starts_with("if") || line.starts_with("???") {
                let condition_tokens = &tokens[1..tokens.len() - 1];
                let condition_met = self.evaluate_condition(condition_tokens, &self.var_container.numbers);
                println!("Condition met: {}", condition_met);

                let mut true_block_lines = Vec::new();
                let mut false_block_lines = Vec::new();
                let mut in_false_block = false;

                while let Some(block_line) = lines_iter.next() {
                    println!("Block line: {}", block_line);
                    let block_line_trimmed = block_line.trim();
                    if block_line_trimmed == "}" {
                        if in_false_block {
                            break;
                        } else {
                            continue;
                        }
                    }
                    if block_line_trimmed.starts_with("else") 
                        || block_line_trimmed.starts_with("!!!") 
                        || block_line_trimmed.starts_with("} else {")
                    {
                        in_false_block = true;
                        
                        if let Some(next_line) = lines_iter.next() {
                            println!("Next line: {}", next_line);
                            if next_line.trim() == "{" {
                                continue;
                            } else if next_line.trim() == "}" {
                                break;
                            } 
                        }
                    }
                
                    if in_false_block {
                        false_block_lines.push(block_line);
                    } else {
                        true_block_lines.push(block_line);
                    }
                }


                println!("True block: {:?}", true_block_lines.join(" ").trim());
                println!("False block: {:?}", false_block_lines.join(" ").trim());

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
            } else {
                self.parse_line(line);
            }
        }

        let result_value = self.var_container.get_number("result").unwrap();
        println!("Result: {}", result_value);
    }
}
