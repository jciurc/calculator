use slint::ToSharedString;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    let mut result: String = ui.get_result().to_string();
    let mut display: String = ui.get_display().to_string();
    let mut open_count = 0;

    ui.on_callback(move |key| {
        let ui = ui_handle.unwrap();
        let key = key.chars().next().unwrap();

        // build main string
        match key {
            // clear leading 0
            '0'..='9' | '(' | '-' if display == "0" => display.clear(),
            _ => {}
        }

        match key {
            '0'..='9' => display.push(key),
            '.' => {
                if display.len() == 0 || display.chars().last().unwrap() != '.' {
                    display.push(key)
                }
            }
            '-' | '+' | '*' | '/' | '%' | '^' => {
                if display.len() > 0 && !"(-+*/%^".contains(display.chars().last().unwrap()) {
                    display.push(key)
                }
            }
            '(' => {
                open_count += 1;
                display.push(key);
            }
            ')' => {
                if open_count > 0 && !"(-+*/%^.".contains(display.chars().last().unwrap()) {
                    open_count -= 1;
                    display.push(key);
                }
            }
            'C' => display = '0'.to_string(),
            _ => {}
        }

        ui.set_display(display.to_shared_string());

        // build result string
        match key {
            // clear leading 0
            '0'..='9' | '(' | '-' if result == "0" => result.clear(),
            _ => {}
        }

        match key {
            '0'..='9' => result.push(key),
            '.' => {
                if !result.contains('.') {
                    result.push(key);
                }
            }
            '=' => {}
            _ => result.clear(),
        }

        ui.set_result(result.to_shared_string());
    });

    ui.run().unwrap();
}
