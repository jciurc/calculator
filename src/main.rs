use slint::ToSharedString;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_callback(move |value| {
        let ui = ui_handle.unwrap();
        ui.set_display((ui.get_display() + &value.to_shared_string()).into())
    });

    ui.run().unwrap();
}
