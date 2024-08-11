

use crate::parser::{math_expr::{evaluate_condition, evaluate_expression, parse_expression}, parser::handle_line};

use super::values::{expressions::Expression, variables::Variables};



pub fn handle_if(line: &str, variables: &mut Variables, lines: &Vec<&str>) {
    
    fn execute_block(block: &[&str], variables: &mut Variables, lines: &Vec<&str>) {
        for line in block {
            handle_line(line, variables, lines);
        }
    }
    
    // Extract the condition from the line
    let condition_start = line.find('(').expect("Missing '(' in if statement") + 1;
    let condition_end = line.find(')').expect("Missing ')' in if statement");
    let condition_str = &line[condition_start..condition_end].trim();

    // Parse the condition expression
    let condition_expr = parse_expression(condition_str)
        .expect("Invalid condition expression");

   let condition_bool = evaluate_condition(&condition_expr, variables)
        .expect("Error evaluating condition");
   
       let mut block_start_index = 0;
       for (index, line) in lines.iter().enumerate() {
           if line.contains('{') {
               block_start_index = index + 1;
               break;
           }
       }
       let true_block_end_index = lines[block_start_index..]
           .iter()
           .position(|&line| line.trim() == "}")
           .map(|pos| pos + block_start_index)
           .expect("Missing '}' in if statement");
   
       // check the line of the end true block for a !!! {
       if lines[true_block_end_index].contains("!!!") {
           let false_block_start_index = lines[true_block_end_index..]
               .iter()
               .position(|&line| line.trim() == "!!!")
               .map(|pos| pos + true_block_end_index)
               .expect("Missing '!!!' in if statement");
           
           let false_block_end_index = lines[false_block_start_index..]
               .iter()
               .position(|&line| line.trim() == "}")
               .map(|pos| pos + false_block_start_index)
               .expect("Missing '}' in if statement");
           
           let false_block = &lines[false_block_start_index..false_block_end_index];
           
           if !condition_bool {
               execute_block(false_block, variables, lines);
           }
           
       }
       
       let true_block = &lines[block_start_index..true_block_end_index];
       if condition_bool {
           execute_block(true_block, variables, lines);
       } 
}

pub fn handle_let_assignment(line: &str, variables: &mut Variables) {
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len() == 2 {
        let var_name = parts[0].trim().replace("let", "").trim().to_string();
        let expr_str = parts[1].trim().to_string();

        let expression = parse_expression(&expr_str)
            .expect(&format!("Invalid expression: {}", expr_str));

        if expression.is_math() {
            let evaluated_value = evaluate_expression(&expression, variables)
                .expect("Error evaluating expression");
            if !evaluated_value.is_number() {
                panic!("Invalid expression: {}", expr_str);
            }
            variables.expr_vars.insert(var_name, Expression::Number(evaluated_value.as_number().unwrap()));
        } else if expression.is_less_than() || expression.is_more_than() {
            let evaluated_value = evaluate_expression(&expression, variables)
                .expect("Error evaluating expression");
            
            if !evaluated_value.is_boolean() {
                panic!("Invalid expression: {} - `<` and `>` can only be used with boolean values", expr_str);
            } 
            variables.expr_vars.insert(var_name, Expression::Boolean(evaluated_value.as_boolean().unwrap()));
        }
        else if expression.is_string() {
            let value = expression.as_string().expect("Invalid string expression");
            variables.string_vars.insert(var_name, value);
        } else if expression.is_number() {
            let value = expression.as_number().expect("Invalid number expression");
            variables.expr_vars.insert(var_name, Expression::Number(value));
        } else if expression.is_variable() {
            let var_name = expression.variable_name().unwrap().to_string();
            if let Some(value) = variables.expr_vars.get(&var_name) {
                variables.expr_vars.insert(var_name.clone(), value.clone());
            } else {
                panic!("Undefined variable: {}", var_name);
            }
        } else {
            panic!("Invalid expression: {}", expr_str);
        }
    }
}

pub fn handle_throw(line: &str, variables: &mut Variables) {
    // Remove "throw"
    let string = line.replace("throw ", "");
    let string_parts = string.split_whitespace().collect::<Vec<&str>>();

    let mut message_parts = Vec::new();
    let mut error_message = String::new();

    for part in string_parts {
        if part.starts_with("$") {
            let var_name = part.trim().replace("$", "").trim().to_string();
            if let Some(value) = variables.expr_vars.get(&var_name) {
                // Format the variable value and add to the message parts
                let evaluated_value = evaluate_expression(value, variables);
                message_parts.push(format!("{}: {:?}", var_name, evaluated_value));
            } else {
                panic!("Undefined variable: {}", var_name);
            }
        } else {
            // Append non-variable parts of the message
            message_parts.push(part.to_string());
        }
    }

    // Join all message parts with spaces and format the final message
    error_message = message_parts.join(" ");
    println!("Thrown error line: {}\nMessage: {}", line, error_message);
}

pub fn handle_return(line: &str, variables: &mut Variables, last_line: &str) {
    let var_name = line.trim().replace(";", "").trim().to_string();
    if !line.ends_with(";") && last_line == line {
        if let Some(value) = variables.expr_vars.get(&var_name) {
            let value = evaluate_expression(value, variables).expect("Error evaluating expression");
            variables.expr_vars.insert("result".to_string(), Expression::Number(value.as_number().unwrap()));
        } else {
            panic!("Undefined variable: {}", var_name);
        }
    } else if line.ends_with(";") {
        if let Some(value) = variables.expr_vars.get(&var_name) {
            println!("{} ( Terminated Early ): {:?}", &var_name, evaluate_expression(value, variables));
            // Early exit
            return;
        } else {
            panic!("Undefined variable: {}", var_name);
        }
    }
}