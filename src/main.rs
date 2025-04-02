use slint::ToSharedString;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    let mut results: String = ui.get_results().to_string();
    let mut display: String = ui.get_display().to_string();

    ui.on_callback(move |value| {
        let ui = ui_handle.unwrap();
        let value = value.chars().next().unwrap();

        match value {
            '0' => {
                if results != "0" {
                    results.push(value)
                }
            }
            '.' => {
                if results.len() == 0 || results.chars().last().unwrap() != '.' {
                    results.push(value)
                }
            }
            'C' | '=' => results.clear(),
            _ => results.push(value),
        }

        ui.set_results(results.to_shared_string());

        match value {
            '0'..='9' if display == "0" => display.clear(),
            _ => {}
        }

        match value {
            '0'..='9' => display.push(value),
            '.' => {
                if !display.contains('.') {
                    display.push(value);
                }
            }
            'C' => display = '0'.to_string(),
            _ => display.clear(),
        }

        ui.set_display(display.to_shared_string());
    });

    ui.run().unwrap();
}
