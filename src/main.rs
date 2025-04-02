use slint::ToSharedString;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    let mut results: String = ui.get_display().to_string();

    ui.on_callback(move |value| {
        let ui = ui_handle.unwrap();
        let value = value.chars().next().unwrap();

        match value {
            '1'..='9' if results == "0" => results.clear(),
            _ => {}
        }

        match value {
            '1'..='9' => results.push(value),
            '0' if results.len() != 1 => results.push(value),
            '.' if !results.contains('.') => results.push(value),
            'C' => results = '0'.to_string(),
            _ => {}
        }

        ui.set_display(results.to_shared_string())
    });

    ui.run().unwrap();
}
