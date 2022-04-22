//! WebAssembly version of this app.

use std::panic;

use wasm_bindgen::prelude::*;

/// WebAssembly main function
#[wasm_bindgen]
pub fn wasm_main() {
	// This hook is necessary to get panic messages in the console
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	let res = super::run();

	if let Err(err) = res {
		web_sys::console::error_1(&format!("Error running the app: {err}").into());
	}
}
