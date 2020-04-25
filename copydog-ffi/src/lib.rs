use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

use copydog::ConfigBuilder;

#[no_mangle]
pub extern fn test() -> i32 {
    5
}

#[no_mangle]
pub extern fn test_str() -> *const c_char {
    let cstr = CString::new("HELLO FROM RUST!").unwrap();
    let ptr = cstr.as_ptr();
    std::mem::forget(cstr);
    ptr
}

#[no_mangle]
pub unsafe extern fn print_input(input: *const c_char) {
    let input = CStr::from_ptr(input).to_str().unwrap();
    let config = ConfigBuilder::new().toml(input).build();
    println!("{:?}", config);
}

