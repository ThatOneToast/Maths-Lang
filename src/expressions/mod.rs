use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    EqualToo,
    Power,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '>' => tokens.push(Token::GreaterThan),
            '<' => tokens.push(Token::LessThan),
            '=' => {
                if let Some(next_ch) = chars.peek() {
                    if next_ch.to_owned() == '=' {
                        tokens.push(Token::EqualToo);
                        chars.next();
                    }
                } else {
                    continue;
                }
            }
            '^' => tokens.push(Token::Power),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' => {
                let mut number = ch.to_string();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_digit(10) || next_ch == '.' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(f64::from_str(&number).unwrap()));
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }
    tokens
}

fn parse_expression(tokens: &[Token]) -> (f64, &[Token]) {
    parse_comparison(tokens) // Start by parsing comparison expressions
}

fn parse_comparison(tokens: &[Token]) -> (f64, &[Token]) {
    let (mut value, mut tokens) = parse_add_sub(tokens); // Parse the arithmetic expressions first

    while let Some(token) = tokens.first() {
        match token {
            Token::GreaterThan => {
                let (rhs, rest) = parse_add_sub(&tokens[1..]);
                value = if value > rhs { 1.0 } else { 0.0 }; // 1.0 for true, 0.0 for false
                tokens = rest;
            }
            Token::LessThan => {
                let (rhs, rest) = parse_add_sub(&tokens[1..]);
                value = if value < rhs { 1.0 } else { 0.0 };
                tokens = rest;
            }
            Token::EqualToo => {
                let (rhs, rest) = parse_add_sub(&tokens[1..]);
                value = if value == rhs { 1.0 } else { 0.0 };
                tokens = rest;
            }
            _ => break,
        }
    }
    (value, tokens)
}

fn parse_add_sub(tokens: &[Token]) -> (f64, &[Token]) {
    let (mut value, mut tokens) = parse_mul_div(tokens);

    while let Some(token) = tokens.first() {
        match token {
            Token::Plus => {
                let (rhs, rest) = parse_mul_div(&tokens[1..]);
                value += rhs;
                tokens = rest;
            }
            Token::Minus => {
                let (rhs, rest) = parse_mul_div(&tokens[1..]);
                value -= rhs;
                tokens = rest;
            }
            _ => break,
        }
    }
    (value, tokens)
}

fn parse_mul_div(tokens: &[Token]) -> (f64, &[Token]) {
    let (mut value, mut tokens) = parse_primary(tokens);

    while let Some(token) = tokens.first() {
        match token {
            Token::Multiply => {
                let (rhs, rest) = parse_primary(&tokens[1..]);
                value *= rhs;
                tokens = rest;
            }
            Token::Divide => {
                let (rhs, rest) = parse_primary(&tokens[1..]);
                value /= rhs;
                tokens = rest;
            }
            Token::Power => {
                let (rhs, rest) = parse_primary(&tokens[1..]);
                value = value.powf(rhs);
                tokens = rest;
            }
            _ => break,
        }
    }
    (value, tokens)
}

fn parse_primary(tokens: &[Token]) -> (f64, &[Token]) {
    match tokens.first() {
        Some(Token::Number(n)) => (n.clone(), &tokens[1..]),
        Some(Token::LParen) => {
            let (value, rest) = parse_expression(&tokens[1..]);
            match rest.first() {
                Some(Token::RParen) => (value, &rest[1..]),
                _ => panic!("Mismatched parentheses"),
            }
        }

        _ => panic!("Unexpected token"),
    }
}

pub fn evaluate_expression(expression: &str) -> f64 {
    let tokens = tokenize(expression);
    let (result, remaining_tokens) = parse_expression(&tokens);
    if remaining_tokens.is_empty() {
        result
    } else {
        panic!("Unexpected tokens remaining: {:?}", remaining_tokens)
    }
}
