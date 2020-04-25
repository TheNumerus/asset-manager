use std::ffi::CStr;
use std::os::raw::c_char;

use copydog::ConfigBuilder;
use copydog::config::Config;

#[derive(Debug)]
pub struct ConfigFfi(Config);

#[no_mangle]
pub unsafe extern fn config_generate(input: *const c_char) -> *mut ConfigFfi {
    let input = CStr::from_ptr(input).to_str().unwrap();
    let config = ConfigBuilder::new().toml(input).build();
    println!("{:?}", config);
    if let Ok(c) = config {
        let config_box = Box::new(ConfigFfi(c));
        Box::into_raw(config_box)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern fn config_free(configffi: *mut ConfigFfi) {
    if configffi.is_null() {
        return;
    }
    let _configbox = Box::from_raw(configffi);
}
