fn precedence(char: char) -> i32 {
    match char {
        '+' => 2,
        '-' => 2,
        '*' => 3,
        '/' => 3,
        '%' => 3,
        '^' => 4,
        _ => 0,
    }
}

fn evaluate(mut tokens: Vec<String>) -> Option<f32> {
    tokens.reverse();
    let mut stack: Vec<f32> = Vec::new();

    while let Some(token) = tokens.pop() {
        match token {
            _ if token.parse::<f32>().is_ok() => stack.push(token.parse::<f32>().unwrap()),
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    "%" => left % right,
                    "^" => left.powf(right),
                    _ => continue,
                })
            }
        }
    }

    if stack.len() == 1 { stack.pop() } else { None }
}

pub fn calculate(expr: String) -> String {
    let mut output = Vec::new();
    let mut stack = Vec::new();
    let mut operand = String::new();

    let chars = expr.chars();
    for i in 0..expr.len() {
        let char = chars.clone().nth(i).unwrap();
        let prev_char_is_closing_bracket = i > 0 && chars.clone().nth(i - 1).unwrap() == ')';

        match char {
            // build operand
            '0'..='9' | '.' => {
                operand.push(char);
                // add explicit multiplication operator if omitted
                if prev_char_is_closing_bracket {
                    stack.push('*')
                }
            }
            // a leading '-' indicates that operand is a negative number
            '-' if operand.is_empty() && !prev_char_is_closing_bracket => {
                operand.push('-');
                continue;
            }
            // push operand
            '+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' => {
                // add explicit multiplication operator if omitted
                if char == '(' && (operand.len() > 0 || prev_char_is_closing_bracket) {
                    stack.push('*');
                }

                if operand.len() > 0 {
                    output.push(operand.clone());
                    operand.clear();
                }
            }
            // skip illegal characters
            _ => continue,
        }

        // handle operators
        match char {
            _ if precedence(char) > 0 => {
                while stack.len() > 0 && precedence(stack[stack.len() - 1]) >= precedence(char) {
                    output.push(stack.pop().unwrap().to_string());
                }
                stack.push(char)
            }
            '(' => stack.push(char),
            ')' => {
                while stack.len() > 0 && stack[stack.len() - 1] != '(' {
                    output.push(stack.pop().unwrap().to_string());
                }
                if stack.is_empty() {
                    return "Invalid parentheses".to_string();
                }
                stack.pop();
            }
            _ => {}
        }
    }

    if operand.len() > 0 {
        output.push(operand);
    }
    while stack.len() > 0 {
        output.push(stack.pop().unwrap().to_string());
    }

    let result = evaluate(output.clone());
    let result = if result.is_some() {
        result.unwrap().to_string()
    } else {
        "Invalid".to_string()
    };

    println!("\nexpression: \"{}\"", expr);
    println!("parsed: [{}]", output.join(","));
    println!("result: {}", result);
    result
}
