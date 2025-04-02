use slint::ToSharedString;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    let mut results: String = ui.get_results().to_string();
    let mut display: String = ui.get_display().to_string();

    ui.on_callback(move |key| {
        let ui = ui_handle.unwrap();
        let key = key.chars().next().unwrap();

        // build main string
        match key {
            // clear leading 0
            '0'..='9' if display == "0" => display.clear(),
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
                if display.len() > 0 && !"-+*/%^".contains(display.chars().last().unwrap()) {
                    display.push(key)
                }
            }
            'C' => display = '0'.to_string(),
            '=' => {}
            _ => display.push(key),
        }

        ui.set_display(display.to_shared_string());

        // build result string
        match key {
            // clear leading 0
            '0'..='9' if results == "0" => results.clear(),
            _ => {}
        }

        match key {
            '0'..='9' => results.push(key),
            '.' => {
                if !results.contains('.') {
                    results.push(key);
                }
            }
            '=' => {}
            _ => results.clear(),
        }

        ui.set_results(results.to_shared_string());
    });

    ui.run().unwrap();
}
