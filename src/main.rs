use crate::calculator::calculate;
use slint::ToSharedString;
mod calculator;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    let mut expression: String = ui.get_expression().to_string();
    let mut display: String = ui.get_display().to_string();
    let mut open_count = 0;
    let mut decimal_used = false;

    ui.on_callback(move |char| {
        let ui = ui_handle.unwrap();
        let char = char.chars().next().unwrap();
        let prev_char = display.chars().last().unwrap_or(' ');

        if display.contains("Invalid") {
            display.clear();
        }
        match char {
            // clear leading 0
            '0'..='9' | '(' | '-' if display == "0" => display.clear(),
            '0'..='9' | '(' if expression != display => display.clear(),
            _ => {}
        }

        match char {
            'C' => display = '0'.to_string(),
            '⌫' if display.len() > 0 => {
                let deleted = display.pop();

                match deleted {
                    Some('.') => decimal_used = false,
                    Some('(') => open_count -= 1,
                    Some(')') => open_count += 1,
                    _ => {}
                }
            }
            '0'..='9' => display.push(char),
            // ignore remaining chars until decimal is closed
            _ if prev_char == '.' => {}
            '.' => {
                if !decimal_used {
                    display.push(char);
                    decimal_used = true;
                }
            }
            // the remaining chars start a new number so a decimal is allowed again
            _ => {
                decimal_used = false;
                match char {
                    '-' => {
                        if display.len() == 0 || prev_char != '-' {
                            display.push(char);
                        }
                    }
                    '(' => {
                        display.push(char);
                        open_count += 1;
                    }
                    // prevent dangling operators
                    _ if display.len() == 0 || "(-+*/%^".contains(prev_char) => {}
                    '+' | '*' | '/' | '%' | '^' => display.push(char),
                    ')' => {
                        if open_count > 0 {
                            display.push(char);
                            open_count -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        let is_valid_expression = open_count == 0 && !"-+*/%^.".contains(prev_char);
        match char {
            'C' => {
                expression = "".to_string();
                display = "0".to_string();
            }
            '=' if is_valid_expression && expression.len() > 0 => {
                let result = calculate(expression.clone());
                display = result.to_string();
            }
            _ => {
                expression = display.clone();
            }
        };

        ui.set_expression(expression.to_shared_string());
        ui.set_display(display.to_shared_string());
    });

    ui.run().unwrap();
}
