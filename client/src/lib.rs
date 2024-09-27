#[no_mangle]
pub extern "C" fn get_hello_message() -> *const u8 {
    let message = "Hello from WASM".to_string();
    message.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_message_len() -> usize {
    "Hello from WASM".len()
}
