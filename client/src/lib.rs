use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn my_func(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn other_func() -> *mut c_char {
    let s = CString::new("hello, world").unwrap();
    s.into_raw()
}
