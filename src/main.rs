use slint::ToSharedString;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    let mut display: String = ui.get_display().to_string();
    let mut open_count = 0;

    ui.on_callback(move |key| {
        let ui = ui_handle.unwrap();
        let key = key.chars().next().unwrap();

        match key {
            // clear leading 0
            '0'..='9' | '(' | '-' if display == "0" => display.clear(),
            _ => {}
        }

        match key {
            'C' => display = '0'.to_string(),
            '0'..='9' => display.push(key),
            // ignore keys until decimal is closed
            _ if display.len() > 0 && display.chars().last().unwrap() == '.' => {}
            '.' => display.push(key),
            '-' => {
                if display.len() == 0 || display.chars().last().unwrap() != '-' {
                    display.push(key);
                }
            }
            '(' => {
                open_count += 1;
                display.push(key);
            }
            // prevent dangling operators
            _ if "-+*/%^".contains(display.chars().last().unwrap()) => {}
            '+' | '*' | '/' | '%' | '^' => {
                if display.len() > 0 {
                    display.push(key)
                }
            }
            ')' => {
                if open_count > 0 && display.chars().last().unwrap() != '(' {
                    open_count -= 1;
                    display.push(key);
                }
            }
            _ => {}
        }

        ui.set_display(display.to_shared_string());
        ui.set_result(match key {
            'C' => "".to_shared_string(),
            '=' if !"-+*/%^".contains(display.chars().last().unwrap()) => "".to_shared_string(),
            _ => display.to_shared_string(),
        })
    });

    ui.run().unwrap();
}
