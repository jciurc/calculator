use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("== Calculator ==");
    println!("----------------");

    let mut x = String::new();
    let mut y = String::new();
    let mut operator = String::new();

    print!("x: ");
    read(&mut x);

    print!("y: ");
    read(&mut y);

    print!("Operator [+-*/]: ");
    read(&mut operator);

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
