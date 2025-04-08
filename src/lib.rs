// This is adapted from `console_error_panic_hook`.
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(js_namespace = console)]
            fn error(msg: String);

            type Error;

            #[wasm_bindgen(constructor)]
            fn new() -> Error;

            #[wasm_bindgen(structural, method, getter)]
            fn stack(error: &Error) -> String;
        }

        /// Print the current stack.
        pub fn backtrace() -> String {
            let e = Error::new();
            e.stack()
        }
    } else {
        /// Print the current stack.
        pub fn backtrace() -> String {
            std::backtrace::Backtrace::capture().to_string()
        }
    }
}
