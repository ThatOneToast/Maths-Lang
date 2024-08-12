use crate::parser::{
    math_expr::{evaluate_condition, evaluate_expression, parse_expression},
    parser::handle_line,
};

use super::values::{expressions::Expression, variables::Variables};

pub fn handle_if(line: &str, variables: &mut Variables, lines: &Vec<&str>) {
    fn execute_block(block: &Vec<String>, variables: &mut Variables, lines: &Vec<&str>) {
        for line in block {
            handle_line(line, variables, lines);
        }
    }

    let condition_start = line.find('(').expect("Missing '(' in if statement") + 1;
    let condition_end = line.find(')').expect("Missing ')' in if statement");
    let condition_str = &line[condition_start..condition_end].trim();
    let condition_expr = parse_expression(condition_str).expect("Invalid condition expression");

    if !(condition_expr.is_less_than() 
        || condition_expr.is_more_than() 
        || condition_expr.is_equals()
    ) {
        panic!(
            "Condition expression is not a boolean result => {}",
            condition_str
        );
    }

    let condition_bool = evaluate_condition(&condition_expr, variables).expect("Error evaluating condition");

    println!("Condition Boolean: {}", condition_bool);

    let mut line_index = lines
        .iter()
        .position(|line| line == &line.to_owned())
        .expect("Line not found")
        + 1;

    let mut next_line = lines.get(line_index).unwrap_or(&"");
    let mut contents: Vec<String> = Vec::new();
    let mut else_contents: Vec<String> = Vec::new();

    while !next_line.contains("}") {
        let mut line = next_line.to_string();
        if let Some(index) = line.find("{") {
            line = line[index + 1..].to_string();
        }
        contents.push(line);
        line_index += 1;
        next_line = lines.get(line_index).unwrap_or(&"");
        if line_index >= lines.len() {
            panic!("Reached end of input without finding closing '}}'");
        }
    }

    // Handle the closing brace
    if next_line.contains("} !!!") {
        line_index += 1;
        next_line = lines.get(line_index).unwrap_or(&"");

        while !next_line.contains("}") {
            else_contents.push(next_line.to_string());
            line_index += 1;
            next_line = lines.get(line_index).unwrap_or(&"");
            if line_index >= lines.len() {
                panic!("Reached end of input without finding closing '}}' for else block");
            }
        }
    } 

    if condition_bool {
        execute_block(&contents, variables, lines);
    } else {
        if !else_contents.is_empty() {
            execute_block(&else_contents, variables, lines);
        } else {
            panic!("No else block found");
        }
    }
}


pub fn handle_let_assignment(line: &str, variables: &mut Variables) {
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len() == 2 {
        let var_name = parts[0].trim().replace("let", "").trim().to_string();
        let expr_str = parts[1].trim().to_string();

        let expression =
            parse_expression(&expr_str).expect(&format!("Invalid expression: {}", expr_str));

        if expression.is_math() {
            let evaluated_value =
                evaluate_expression(&expression, variables).expect("Error evaluating expression");
            if !evaluated_value.is_number() {
                panic!("Invalid expression: {}", expr_str);
            }
            variables.expr_vars.insert(
                var_name,
                Expression::Number(evaluated_value.as_number().unwrap()),
            );
        } else if expression.is_less_than() 
            || expression.is_more_than() 
            || expression.is_equals() {
            let evaluated_value =
                evaluate_expression(&expression, variables).expect("Error evaluating expression");

            if !evaluated_value.is_boolean() {
                panic!(
                    "Invalid expression: {} - `<` and `>` can only be used with boolean values",
                    expr_str
                );
            }
            variables.expr_vars.insert(
                var_name,
                Expression::Boolean(evaluated_value.as_boolean().unwrap()),
            );
        } else if expression.is_string() {
            let value = expression.as_string().expect("Invalid string expression");
            variables.string_vars.insert(var_name, value);
        } else if expression.is_number() {
            let value = expression.as_number().expect("Invalid number expression");
            variables
                .expr_vars
                .insert(var_name, Expression::Number(value));
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
    let string = line.replace("throw ", "");
    let string_parts = string.split_whitespace().collect::<Vec<&str>>();

    let mut message_parts = Vec::new();

    for part in string_parts {
        if part.starts_with("$") {
            let var_name = part.trim().replace("$", "").trim().to_string();
            if let Some(value) = variables.expr_vars.get(&var_name) {
                let evaluated_value = evaluate_expression(value, variables);
                message_parts.push(format!("{}: {:?}", var_name, evaluated_value));
            } else {
                panic!("Undefined variable: {}", var_name);
            }
        } else {
            message_parts.push(part.to_string());
        }
    }

    let message = message_parts.join(" ");
    println!("Thrown: {}", message);

}

pub fn handle_return(line: &str, variables: &mut Variables, last_line: &str) {
    let var_name = line.trim().replace(";", "").trim().to_string();
    if !line.ends_with(";") && last_line == line {
        if let Some(value) = variables.expr_vars.get(&var_name) {
            let value = evaluate_expression(value, variables).expect("Error evaluating expression");
            variables.expr_vars.insert(
                "result".to_string(),
                Expression::Number(value.as_number().unwrap()),
            );
        } else {
            panic!("Undefined variable: {}", var_name);
        }
    } else if line.ends_with(";") {
        if let Some(value) = variables.expr_vars.get(&var_name) {
            println!(
                "{} ( Terminated Early ): {:?}",
                &var_name,
                evaluate_expression(value, variables)
            );
            // Early exit
            return;
        } else {
            panic!("Undefined variable: {}", var_name);
        }
    }
}
