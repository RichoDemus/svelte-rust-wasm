use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn greet() -> Result<JsValue, JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

    // Your code goes here!
    let result = JsValue::from_str("Hello from Rust!");
    console::log_1(&result);

    Ok(result)
}

#[wasm_bindgen]
pub struct Person {
    pub age: u32,
}

#[wasm_bindgen]
pub fn get_object() -> Result<JsValue,JsValue> {
    Ok(Person { age: 10 }.into())
}

#[wasm_bindgen]
pub fn throw_exception() -> Result<(),JsValue> {
    Err("Error message".into())
}
