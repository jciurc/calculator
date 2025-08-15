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
            '0'..='9' | '.' => operand.push(char),
            '-' if operand.len() == 0 => operand.push(char),
            _ if precedence(char) > 0 => {
                if operand.len() > 0 {
                    output.push(operand.clone());
                    operand.clear();
                }
                while stack.len() > 0 && precedence(stack[stack.len() - 1]) < precedence(char) {
                    output.push(stack.pop().unwrap().to_string());
                }
                stack.push(char);
            }
            '(' => {
                if operand.len() > 0 {
                    output.push(operand.clone());
                    operand.clear();
                    stack.push('*');
                }
                stack.push(char);
            }
            ')' => {
                if operand.len() > 0 {
                    output.push(operand.clone());
                    operand.clear();
                }
                while stack.len() > 0 && stack[stack.len() - 1] != '(' {
                    output.push(stack.pop().unwrap().to_string());
                }
                if stack.len() > 0 && stack[stack.len() - 1] == '(' {
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
