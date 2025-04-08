# wasm_backtrace

Generate backtraces from within WASM.

Note: currently, [Rust does not support backtraces within WASM](https://users.rust-lang.org/t/what-is-the-current-state-of-backtraces-in-wasm/128002).
This crate actually uses javascript to generate a WASM backtrace.
This does not contain as much information as a real Rust backtrace, but it is the best that we can do for now.
