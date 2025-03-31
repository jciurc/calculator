slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let main_window = MainWindow::new().unwrap();

    main_window.run().unwrap();
}
