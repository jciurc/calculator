use std::io::{Write, stdin, stdout};

fn read(prompt: &str) -> String {
    let mut input = String::new();

    print!("{prompt}");
    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut input).expect("failed to read");
    input
}

fn main() {
    println!("== Calculator ==");
    println!("----------------");

    let x = read("\nx: ");
    let y = read("y: ");
    let operator = read("Operator [+-*/]: ");

    let x: f32 = x.trim().parse().unwrap();
    let y: f32 = y.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let result = match operator {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        _ => {
            println!("Operator not recognized");
            return;
        }
    };

    println!("{x} {operator} {y} = {result}");
}
