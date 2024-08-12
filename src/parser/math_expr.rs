use super::values::{expressions::Expression, variables::Variables};

use std::str::FromStr;

pub fn evaluate_expression(expr: &Expression, variables: &Variables) -> Result<Box<Expression>, String> {
    match expr {
        Expression::Add(left, right) => {
            let left_value = evaluate_expression(left, variables).unwrap();
            let right_value = evaluate_expression(right, variables).unwrap();
            Ok(Box::new(Expression::Number(
                left_value.as_number().unwrap() + right_value.as_number().unwrap(),
            )))
        }
        Expression::Subtract(left, right) => {
            let left_value = evaluate_expression(left, variables).unwrap();
            let right_value = evaluate_expression(right, variables).unwrap();
            Ok(Box::new(Expression::Number(
                left_value.as_number().unwrap() - right_value.as_number().unwrap(),
            )))
        }
        Expression::Multiply(left, right) => {
            let left_value = evaluate_expression(left, variables).unwrap();
            let right_value = evaluate_expression(right, variables).unwrap();
            Ok(Box::new(Expression::Number(
                left_value.as_number().unwrap() * right_value.as_number().unwrap(),
            )))
        }
        Expression::Divide(left, right) => {
            let left_value = evaluate_expression(left, variables).unwrap();
            let right_value = evaluate_expression(right, variables).unwrap();
            Ok(Box::new(Expression::Number(
                left_value.as_number().unwrap() / right_value.as_number().unwrap(),
            )))
        }
        Expression::Power(left, right) => {
            if left.is_number() && right.is_number() {
                let left_val = evaluate_expression(left, variables)?.as_number().unwrap();
                let right_val = evaluate_expression(right, variables)?.as_number().unwrap();
                Ok(Box::new(Expression::Number(left_val.powf(right_val))))
            } else {
                Err("Invalid operands for power".to_string())
            }
        }
        Expression::Number(num) => Ok(Box::new(Expression::Number(*num))),
        Expression::Variable(var) => {
            if let Some(expr) = variables.expr_vars.get(var) {
                evaluate_expression(expr, variables)
            } else {
                panic!("Undefined variable: {}", var)
            }
        }

        Expression::LessThan(left, right) => {
            let left_eval = evaluate_expression(left, variables)
                .expect("Failed evalutating the left hand side of the less than operation.");
            
            let right_eval = evaluate_expression(right, variables)
                .expect("Failed evalutating the right hand side of the less than operation.");
            
            let left_value = left_eval.as_number()
                .expect("Failed convertying the left value to a number.");
            
            let right_value = right_eval.as_number()
                .expect("Failed converting the right vlaue to a number.");
            
            Ok(Box::new(Expression::Boolean(left_value < right_value)))
        }
        
        Expression::MoreThan(left, right) => {
            let left_eval = evaluate_expression(left, variables)
                .expect("Failed evalutating the left hand side of the more than operation.");
            
            let right_eval = evaluate_expression(right, variables)
                .expect("Failed evalutating the right hand side of the more than operation.");
            
            let left_value = left_eval.as_number()
                .expect("Failed convertying the left value to a number.");
            
            let right_value = right_eval.as_number()
                .expect("Failed converting the right vlaue to a number.");
            
            Ok(Box::new(Expression::Boolean(left_value > right_value)))
        }
    
        Expression::Equals(left, right) => {
            let left_eval = evaluate_expression(left, variables)
                .expect("Failed evalutating the left hand side of the equals operation.");
            
            let right_eval = evaluate_expression(right, variables)
                .expect("Failed evalutating the right hand side of the equals operation.");
            
            let left_value = left_eval.as_number()
                .expect("Failed convertying the left value to a number.");
            
            let right_value = right_eval.as_number()
                .expect("Failed converting the right vlaue to a number.");
            
            println!("Left: {}, Right: {}", left_value, right_value);
            Ok(Box::new(Expression::Boolean(left_value == right_value)))
        }

        Expression::Boolean(value) => Ok(Box::new(Expression::Boolean(*value))),
    }
}

pub fn parse_expression(expr_str: &str) -> Result<Expression, String> {
    let tokens: Vec<&str> = expr_str.split_whitespace().collect();
    let mut tokens_iter = tokens.iter().peekable();

    match parse_simple_expr(&mut tokens_iter) {
        Ok(expr) => Ok(expr),
        Err(e) => panic!("Error parsing expression: {}", e),
    }
}

fn parse_simple_expr<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a &'a str>,
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
            ">" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::MoreThan(Box::new(lhs), Box::new(rhs))
            }
            "<" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::LessThan(Box::new(lhs), Box::new(rhs))
            }
            "==" => {
                tokens.next();
                let rhs = parse_factor(tokens)?;
                Expression::Equals(Box::new(lhs), Box::new(rhs))
            }
            _ => break,
        };
        lhs = expression;
    }

    Ok(lhs)
}

fn parse_factor<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a &'a str>,
{
    if let Some(&&token) = tokens.peek() {
        if let Ok(num) = f64::from_str(token) {
            tokens.next(); // Consume the number
            return Ok(Expression::Number(num));
        } else if token.starts_with("\"") && token.ends_with("\"") {
            tokens.next(); // Consume the string
            return Ok(Expression::Variable(token.to_string())); // Use Variable to store raw string
        } else {
            tokens.next(); // Consume the variable
            return Ok(Expression::Variable(token.to_string()));
        }
    }
    Err("Unexpected end of expression".to_string())
}

pub fn evaluate_condition(
    condition_value: &Expression,
    variables: &Variables,
) -> Result<bool, String> {
    match condition_value {
        Expression::LessThan(left, right) => {
            let left_eval = evaluate_expression(left, variables)
                .expect("Error evaluating left side of comparison");
            let right_eval = evaluate_expression(right, variables)
                .expect("Error evaluating right side of comparison");
            
            let left_value = left_eval
                .as_number()
                .expect("Left side of comparison is not a number");
            let right_value = right_eval
                .as_number()
                .expect("Right side of comparison is not a number");
            
            Ok(left_value < right_value)
        }

        Expression::MoreThan(left, right) => {
            let left_eval = evaluate_expression(left, variables)
                .expect("Error evaluating left side of comparison");
            let right_eval = evaluate_expression(right, variables)
                .expect("Error evaluating right side of comparison");

            let left_value = left_eval
                .as_number()
                .expect("Left side of comparison is not a number");
            let right_value = right_eval
                .as_number()
                .expect("Right side of comparison is not a number");

            Ok(left_value > right_value)

        }

        Expression::Equals(left, right) => {
            let left_eval = evaluate_expression(left, variables)
                .expect("Error evaluating left side of comparison");
            let right_eval = evaluate_expression(right, variables)
                .expect("Error evaluating right side of comparison");

            let left_value = left_eval
                .as_number()
                .expect("Left side of comparison is not a number");
            let right_value = right_eval
                .as_number()
                .expect("Right side of comparison is not a number");

            Ok(left_value.abs() == right_value.abs())
        }
        Expression::Boolean(value) => Ok(*value),

        _ => Err("Not a proper condition format".to_string()),
    }
}
