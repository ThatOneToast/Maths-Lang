use std::{collections::HashMap, iter::Peekable};

use results::Results;
use types::VariableContainer;

use crate::{expressions::evaluate_expression, remove_whitespace};
mod helpers;
pub mod patterns;
pub mod results;
pub mod types;

pub struct Parser<'a> {
    pub contents: &'a str,
    pub patterns: HashMap<String, String>, // <name> <maths_content>
    pub var_container: VariableContainer,
}

impl<'a> Parser<'a> {
    pub fn new(contents: &'a str, ppatterns: Option<HashMap<String, String>>) -> Self {
        let patterns = patterns::PATTERNS.clone();
        let constructed_patterns = patterns
            .iter()
            .map(|(_, pattern)| pattern.construct())
            .collect::<HashMap<String, String>>();

        let mut final_patterns = ppatterns.unwrap_or(HashMap::new()).clone();
        final_patterns.extend(constructed_patterns);

        Self {
            contents,
            patterns: final_patterns,
            var_container: VariableContainer::new(),
        }
    }

    pub fn get_paramater_names(&self) -> Result<Vec<String>, String> {
        let lines = self.contents.lines().collect::<Vec<&str>>();
        let first = lines.get(0).unwrap_or(&"");

        if first.starts_with("#[") {
            let params_string = first
                .strip_prefix("#[")
                .unwrap_or("")
                .strip_suffix("]")
                .unwrap_or("");
            let params = params_string.split(",").collect::<Vec<&str>>();

            let new_params = params
                .iter()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            Ok(new_params)
        } else {
            Err("This maths doesn't contain any context of paramaters TOP OF LINE: #[params, params1]".to_string())
        }
    }

    fn get_pattern_paramaters_names(&self, pattern_name: &str) -> Vec<String> {
        let lines = self
            .patterns
            .get(pattern_name)
            .expect(&format!("Pattern {} doesn't exist", pattern_name))
            .lines()
            .collect::<Vec<&str>>();
        let first = lines.first().unwrap();

        if first.starts_with("#[") {
            let params_string = first
                .strip_prefix("#[")
                .unwrap_or("")
                .strip_suffix("]")
                .unwrap_or("");
            let params = params_string.split(",").collect::<Vec<&str>>();

            let new_params = params
                .iter()
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            new_params
        } else {
            panic!("This maths doesn't contain any context of paramaters TOP OF LINE: #[params, params1]");
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
                "Let statement must be followed by a variable name, see {} \n {}",
                line, "let example = 50"
            );
        }

        true
    }

    fn is_variable(&self, token: &str) -> bool {
        self.var_container.get_number(token).is_some()
    }

    fn is_if_statement(&self, line: &str) -> bool {
        line.starts_with("if") || line.starts_with("???")
    }

    fn evaluate_condition(
        &self,
        condition_tokens: &[&str],
        num_vars: &HashMap<String, f64>,
    ) -> bool {
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

    fn parse_line(
        &mut self,
        line: &str,
        iterator: Option<&mut Peekable<std::str::Lines<'_>>>,
    ) -> Result<Results, String> {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if self.is_let_statement(line, tokens.clone()) {
            let variable_name = tokens[1];

            // collect all remaining tokens if its a space skip
            let mut remaining_tokens = tokens[2..]
                .iter()
                .map(|s| remove_whitespace!(s).to_string())
                .collect::<Vec<String>>();

            for (_, token) in remaining_tokens.iter_mut().enumerate() {
                if token.starts_with("@") {
                    let tokenn = remove_whitespace!(token).to_string();

                    let (pattern_name, mut passed_params) =
                        tokenn.strip_prefix("@").unwrap().split_once("(").unwrap();

                    passed_params = passed_params.strip_suffix(")").unwrap();

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

                    let pattern_content = self
                        .patterns
                        .get(pattern_name)
                        .expect(&format!("Pattern {} doesn't exist", pattern_name));
                    let pattern_params = self.get_pattern_paramaters_names(pattern_name);

                    if pattern_params.len() != passed_params_list.len() {
                        panic!("The number of passed params doesn't match the number of params in the pattern");
                    }

                    let mut pattern_parser =
                        Parser::new(pattern_content, Option::from(self.patterns.clone()));
                    let mut cursor: usize = 0;
                    for param in pattern_params {
                        let param_value_int = passed_params_list
                            .get(cursor)
                            .unwrap()
                            .parse::<f64>()
                            .unwrap();
                        pattern_parser
                            .var_container
                            .numbers
                            .insert(param.clone(), param_value_int);
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
        } else if self.is_if_statement(line) {
            let result = self.process_if_statement(line, iterator.unwrap()).unwrap();
            if result == Results::BREAK {
                return Ok(result);
            }
        } else if line.starts_with(";") {
            let var_name = line.strip_prefix(";").unwrap_or("");

            let var_value = self
                .var_container
                .numbers
                .get(var_name)
                .expect(&format!("Undefined variable: {} Line: {}", var_name, line));

            println!("{}: {}", var_name, var_value);
        } else if line.starts_with("break") || line.starts_with("BREAK") {
            return Ok(Results::BREAK);
        } else if line.starts_with("continue") || line.starts_with("CONTINUE") {
            return Ok(Results::CONTINUE);
        }

        Ok(Results::OK)
    }

    fn process_if_statement(
        &mut self,
        line: &str,
        lines_iter: &mut Peekable<std::str::Lines<'_>>,
    ) -> Result<Results, String> {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let condition_tokens = &tokens[1..tokens.len() - 1];
        let condition_met = self.evaluate_condition(condition_tokens, &self.var_container.numbers);

        let mut true_block_lines: Vec<&str> = Vec::new();
        let mut false_block_lines: Vec<&str> = Vec::new();
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
                    } else if next_line.trim() == "} else {" {
                        continue;
                    } else if next_line.trim() == "}" {
                        break;
                    }
                }
            }

            if in_false_block {
                false_block_lines.push(block_line.trim());
            } else {
                true_block_lines.push(block_line.trim());
            }
        }

        if condition_met {
            if true_block_lines.is_empty() {
                panic!("Empty true block");
            }
            for block_line in true_block_lines {
                let result = self.parse_line(block_line, None).unwrap();
                if result == Results::BREAK || result == Results::CONTINUE {
                    return Ok(result);
                }
            }
        } else {
            if false_block_lines.is_empty() {
                panic!("Empty false block");
            }
            for block_line in false_block_lines {
                let result = self.parse_line(block_line, None).unwrap();
                if result == Results::BREAK || result == Results::CONTINUE {
                    return Ok(result);
                } 
            }
        }

        Ok(Results::OK)
    }

    pub fn parse(&mut self) {
        let mut lines_iter = self.contents.lines().peekable();

        while let Some(line) = lines_iter.next() {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            if line.starts_with("#") {
                continue;
            }

            if line.starts_with("loop") || line.starts_with("LOOP") {
                let mut loop_iter = tokens.iter().peekable();
                loop_iter.next();
                let current_token = loop_iter.next().unwrap();
                let loop_count: u128;

                if self.is_variable(&current_token) {
                    let num = self
                        .var_container
                        .get_number(&current_token)
                        .expect("Variable not found")
                        .floor() as u128;
                    loop_count = num;
                } else {
                    let num = current_token
                        .parse::<f64>()
                        .expect("Failed to parse loop count");
                    loop_count = num.floor() as u128;
                }

                let mut loop_block_lines: Vec<String> = Vec::new();

                // Collecting Loop Contents
                while let Some(line) = lines_iter.next() {
                    if line.trim().starts_with("loop_end") || line.trim().starts_with("LOOP_END") {
                        break;
                    }

                    if line.trim().starts_with("loop") {
                        panic!("Loops can't be nested");
                    }
                    loop_block_lines.push(line.trim().to_string());
                }

                'outer: for _ in 0..loop_count {
                    let lines = loop_block_lines.join("\n");
                    let mut blines_iter = lines.lines().into_iter().peekable();
                    while let Some(block_line) = blines_iter.next() {
                        match self.parse_line(block_line, 
                            Option::from(&mut blines_iter))
                        .unwrap()
                        {
                            Results::BREAK => {
                                break 'outer;
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                let _ = self
                    .parse_line(line, Option::from(&mut lines_iter))
                    .unwrap();
            }
        }
    }
}
