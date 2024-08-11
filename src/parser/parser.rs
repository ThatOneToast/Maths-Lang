use std::collections::HashMap;
use std::str::FromStr;

use super::values::expressions::Expression;
use super::variables::Variables;


fn handle_let_assignment(line: &str, variables: &mut Variables) {
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len() == 2 {
        let var_name = parts[0].trim().replace("let", "").trim().to_string();
        let expr_str = parts[1].trim().to_string();

        let expression = parse_expression(&expr_str, variables)
            .expect(&format!("Invalid expression: {}", expr_str));

        if expression.is_math() {
            let evaluated_value = evaluate_expression(&expression, variables)
                .expect("Error evaluating expression");
            if !evaluated_value.is_number() {
                panic!("Invalid expression: {}", expr_str);
            }
            variables.expr_vars.insert(var_name, Expression::Number(evaluated_value.as_number().unwrap()));
        } else if expression.is_string() {
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

fn handle_throw(line: &str, variables: &mut Variables) {
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


pub fn parse_expression_file(input: &str) -> Variables {
    let mut variables = Variables {
        string_vars: HashMap::new(),
        expr_vars: HashMap::new(),
    };

    let lines = input
        .lines()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let last_line = lines.last().unwrap_or(&"").to_owned();

    for line in lines {
        handle_line(line, &mut variables, &last_line);
    }

    Variables {
        expr_vars: variables.expr_vars,
        string_vars: variables.string_vars,
    }
}

fn handle_if(line: &str, variables: &mut Variables) {
    // Extract the condition and blocks from the line
    let parts: Vec<&str> = line.split("???").collect();
    if parts.len() < 2 {
        panic!("Invalid if statement: {}", line);
    }

    let condition_and_blocks = parts[1].trim();
    let mut blocks = condition_and_blocks.split("!!!");

    let condition_block = blocks.next().unwrap_or("").trim();

    // Extract condition and true block
    let condition_start = condition_block.find('(').expect("Missing '(' in if statement") + 1;
    let condition_end = condition_block.find(')').expect("Missing ')' in if statement");
    let condition_str = &condition_block[condition_start..condition_end].trim();

    // Find the opening brace for the true block
    let true_block_start = condition_block.find('{').expect("Missing '{' in if statement") + 1;
    // Find the closing brace for the true block
    let true_block_end = condition_block.rfind('}').expect("Missing '}' in if statement");
    let true_block_str = &condition_block[true_block_start..true_block_end].trim();

    // Evaluate the condition
    let condition_expr = parse_expression(condition_str, variables).expect("Invalid condition expression");
    let condition_value = evaluate_expression(&condition_expr, variables)
        .expect("Error evaluating condition").as_ref().to_owned();
    
    let result = match condition_value {
        Expression::LessThan(left, right) => {
            if left.is_number() && right.is_number() {
                let left_num = evaluate_expression(&left, variables).expect("Error evaluating expression")
                    .as_number().expect("Failed converting to number");
                let right_num = evaluate_expression(&right, variables).expect("Error evaluating expression")
                    .as_number().expect("Failed converting to number");
                
                let bool_val = left_num < right_num;
                
                Ok(bool_val)
            } else {
                Err("Invalid operands for conditional".to_string())
            }   
        }
       
       Expression::MoreThan(left, right) => {
           if left.is_number() && right.is_number() {
               let left_num = evaluate_expression(&left, variables).expect("Error evaluating expression")
                   .as_number().expect("Failed converting to number");
               let right_num = evaluate_expression(&right, variables).expect("Error evaluating expression")
                   .as_number().expect("Failed converting to number");
               
               let bool_val = left_num > right_num;
               
               Ok(bool_val)
           } else {
               Err("Invalid operands for conditional".to_string())
           }
        }
        
        Expression::Boolean(value) => {
            Ok(value.to_owned())
        }
        
        _ => Err("Not propper conditioning format".to_string())
    };            

    // Execute the true block if the condition is true
    if result.expect("Error evaluating condition") {
        let true_lines = true_block_str.lines().map(|s| s.trim());
        for line in true_lines {
            handle_line(line, variables, "");
        }
    } else if let Some(false_block) = blocks.next() {
        // Find the opening brace for the false block
        let false_block_start = false_block.find('{').expect("Missing '{' in else statement") + 1;
        // Find the closing brace for the false block
        let false_block_end = false_block.rfind('}').expect("Missing '}' in else statement");
        let false_block_str = &false_block[false_block_start..false_block_end].trim();

        let false_lines = false_block_str.lines().map(|s| s.trim());
        for line in false_lines {
            handle_line(line, variables, "");
        }
    } else {
        panic!("Missing 'else' block in if statement: {}", line);
    }
}


fn handle_line(line: &str, variables: &mut Variables, last_line: &str) {
    if line.starts_with("let") {
        handle_let_assignment(line, variables);
    } else if line.starts_with(";") {
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
    } else if line.starts_with("throw") {
        handle_throw(line, variables);
    } else if line.starts_with("???") {
        handle_if(line, variables)
    }
}


fn parse_expression(expr_str: &str, variables: &Variables) -> Result<Expression, String> {
    let tokens: Vec<&str> = expr_str.split_whitespace().collect();
    let mut tokens_iter = tokens.iter().peekable();
    
    match parse_simple_expr(&mut tokens_iter) {
        Ok(expr) => Ok(expr),
        Err(_) => parse_advanced_expr(&mut tokens_iter, variables),
    }
}

fn parse_advanced_expr<'a, I>(tokens: &mut std::iter::Peekable<I>, variables: &Variables) -> Result<Expression, String>
where
    I: Iterator<Item = &'a &'a str>,
{
    let mut expression = parse_simple_expr(tokens)?;


    Ok(expression)
}



fn parse_simple_expr<'a, I>(tokens: &mut std::iter::Peekable<I>,) -> Result<Expression, String> where I: Iterator<Item = &'a &'a str>,
{
    let mut lhs = parse_factor(tokens)?;

    while let Some(&&token) = tokens.peek() {
        let expression = match token {
            "+" => {
                tokens.next(); // Consume the operator
                let rhs = parse_factor(tokens)?;
                Expression::Add(Box::new(lhs), Box::new(rhs))
            }
            "-" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::Subtract(Box::new(lhs), Box::new(rhs))
            }
            "*" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::Multiply(Box::new(lhs), Box::new(rhs))
            }
            "/" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::Divide(Box::new(lhs), Box::new(rhs))
            }
            "^" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::Power(Box::new(lhs), Box::new(rhs))
            }
            _ => break,
        };
        lhs = expression;
    }

    Ok(lhs)
}

fn parse_factor<'a, I>(tokens: &mut std::iter::Peekable<I>,) -> Result<Expression, String> where I: Iterator<Item = &'a &'a str>,
{
    if let Some(&&token) = tokens.peek() {
        if let Ok(num) = f64::from_str(token) {
            tokens.next(); // Consume the number
            return Ok(Expression::Number(num));
        } else if token.starts_with("\"") && token.ends_with("\"") {
            tokens.next(); // Consume the string
            return Ok(Expression::Variable(token.to_string())); // Use Variable to store raw string
        }  
        else {
            tokens.next(); // Consume the variable
            return Ok(Expression::Variable(token.to_string()));
        }
    }
    Err("Unexpected end of expression".to_string())
}

fn evaluate_expression(expr: &Expression, variables: &Variables) -> Result<Box<Expression>, String> {
    match expr {
        Expression::Add(left, right) => {
            if left.is_number() && right.is_number() {
                Ok(Box::new(Expression::Number(evaluate_expression(left, variables)?.as_number().unwrap() + evaluate_expression(right, variables)?.as_number().unwrap())))
            } else {
                Err("Invalid operands for addition".to_string())
            }
        }
        Expression::Subtract(left, right) => {
            if left.is_number() && right.is_number() {
                Ok(Box::new(Expression::Number(evaluate_expression(left, variables)?.as_number().unwrap() - evaluate_expression(right, variables)?.as_number().unwrap())))
            } else {
                Err("Invalid operands for subtraction".to_string())
            }
        }
        Expression::Multiply(left, right) => {
            if left.is_number() && right.is_number() {
                Ok(Box::new(Expression::Number(evaluate_expression(left, variables)?.as_number().unwrap() * evaluate_expression(right, variables)?.as_number().unwrap())))
            } else {
                Err("Invalid operands for multiplication".to_string())
            }
        }
        Expression::Divide(left, right) => {
            if left.is_number() && right.is_number() {
                Ok(Box::new(Expression::Number(evaluate_expression(left, variables)?.as_number().unwrap() / evaluate_expression(right, variables)?.as_number().unwrap())))
            } else {
                Err("Invalid operands for division".to_string())
            }
        }
        Expression::Power(left, right) => {
            if left.is_number() && right.is_number() {
                // do the power calculation return the result as a number expression
                let left_val = evaluate_expression(left, variables)?.as_number().unwrap();
                let right_val = evaluate_expression(right, variables)?.as_number().unwrap();
                Ok(Box::new(Expression::Number(left_val.powf(right_val))))
            } else {
                Err("Invalid operands for power".to_string())
            }
        }
        Expression::Number(num) => {
            Ok(Box::new(Expression::Number(*num)))
        }
        Expression::Variable(var) => {
            if let Some(expr) = variables.expr_vars.get(var) {
                evaluate_expression(expr, variables)
            } else {
                panic!("Undefined variable: {}", var)
            }
        }
        
        Expression::LessThan(left, right) => {
            if left.is_number() && right.is_number() {
                Ok(Box::new(Expression::Boolean(evaluate_expression(left, variables)?.as_number().unwrap() < evaluate_expression(right, variables)?.as_number().unwrap())))
            } else {
                Err("Invalid operands for conditional".to_string())
            }
        }
        
        Expression::MoreThan(left, right) => {
            if left.is_number() && right.is_number() {
                Ok(Box::new(Expression::Boolean(evaluate_expression(left, variables)?.as_number().unwrap() > evaluate_expression(right, variables)?.as_number().unwrap())))
            } else {
                Err("Invalid operands for conditional".to_string())
            }
        }

        Expression::Boolean(value) => {
            Ok(Box::new(Expression::Boolean(*value)))
        }
                
    }
}



pub fn calculate_sequence(sequence: &mut Variables) -> Box<Expression> {
    if let Some(expr) = sequence.expr_vars.get("result") {
        let variables = Variables {
            expr_vars: sequence.expr_vars.clone(),
            string_vars: sequence.string_vars.clone(),
        };
        return evaluate_expression(expr, &variables)
            .expect("Error evaluating expression");
    }

    Box::new(Expression::Number(0.0)) // Return 0 if 'result' is not defined
}
