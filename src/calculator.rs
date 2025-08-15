pub fn calculate(expr: String) -> String {
    let mut current = String::new();
    let mut operators = String::new();
    let mut output = Vec::new();

    for char in expr.chars() {
        match char {
            '0'..='9' => current.push(char),
            '-' if current.len() == 0 => current.push(char),
            '(' | '-' | '+' | '*' | '/' | '%' | '^' => {
                operators.push(char);
                output.push(current.clone());
                current.clear();
            }
            _ => {}
        }
    }

    output.push(current);
    output.push(operators);
    output.join(" ")
}
