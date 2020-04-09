use std::ffi::CString;
use std::os::raw::c_char;

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

