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
            '-' if operand.is_empty() => operand.push(char),
            // push operand
            '+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' if operand.len() > 0 => {
                output.push(operand.clone());
                operand.clear();

                if char == '(' {
                    stack.push('*');
                }
            }
            // skip illegal characters
            _ => continue,
        }

        // handle operators
        match char {
            _ if precedence(char) > 0 => {
                while stack.len() > 0 && precedence(stack[stack.len() - 1]) < precedence(char) {
                    output.push(stack.pop().unwrap().to_string());
                }
                if char == '-' {
                    output.push("-1".to_string());
                    stack.push('+');
                    stack.push('/');
                } else {
                    stack.push(char);
                }
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
    output.join(" ")
}
