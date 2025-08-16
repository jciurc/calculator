fn evaluate(mut tokens: Vec<String>) -> Option<f32> {
    tokens.reverse();
    let mut stack: Vec<f32> = Vec::new();

    while let Some(token) = tokens.pop() {
        match token {
            _ if token.parse::<f32>().is_ok() => stack.push(token.parse::<f32>().unwrap()),
            _ => {
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

    return Some(stack[0]);
}

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

pub fn calculate(expr: String) -> String {
    let mut output = Vec::new();
    let mut stack = Vec::new();
    let mut operand = String::new();

    for char in expr.chars() {
        match char {
            // build operand
            '0'..='9' | '.' => operand.push(char),
            // a leading '-' indicates operand is a negative number
            '-' if operand.is_empty() => {
                operand.push('-');
                continue;
            }
            // push operand
            '+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' => {
                if operand.len() > 0 {
                    output.push(operand.clone());
                    operand.clear();

                    if char == '(' {
                        stack.push('*');
                    }
                }
            }
            // skip illegal characters
            _ => continue,
        }

        // handle operators
        match char {
            _ if precedence(char) > 0 => {
                while stack.len() > 0
                    && precedence(stack[stack.len() - 1]) > 0
                    && precedence(stack[stack.len() - 1]) < precedence(char)
                {
                    output.push(stack.pop().unwrap().to_string());
                }
                stack.push(char)
            }
            '(' => stack.push(char),
            ')' => {
                while stack.len() > 0 && stack[stack.len() - 1] != '(' {
                    output.push(stack.pop().unwrap().to_string());
                }
                if stack.len() > 0 {
                    stack.pop();
                }
            }
            _ => {}
        }
    }

    if operand.len() > 0 {
        output.push(operand);
    }
    for op in stack {
        output.push(op.to_string());
    }

    println!("expression: {}", output.join(","));
    evaluate(output).unwrap().to_string()
}
