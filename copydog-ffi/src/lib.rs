use std::ffi::CStr;
use std::os::raw::c_char;

use copydog::ConfigBuilder;
use copydog::config::Config;
use copydog::watch::Watcher;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct ConfigFfi(Config);

#[derive(Debug)]
pub struct WatcherFfi(Watcher);

#[no_mangle]
pub unsafe extern fn config_generate(input: *const c_char) -> *mut ConfigFfi {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let input = CStr::from_ptr(input).to_str().unwrap();
    let config = ConfigBuilder::new().toml(input).build();
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

#[no_mangle]
pub unsafe extern fn watcher_new(configffi: *mut ConfigFfi) -> *mut WatcherFfi {
    if configffi.is_null() {
        return std::ptr::null_mut();
    }
    let config_clone = (*configffi).to_owned().0;
    let watcher =  Watcher::new(config_clone);
    let watcher = Box::new(WatcherFfi(watcher));
    Box::into_raw(watcher)
}

#[no_mangle]
pub unsafe extern fn watcher_free(watcherffi: *mut WatcherFfi) {
    if watcherffi.is_null() {
        return;
    }
    let _watcherbox = Box::from_raw(watcherffi);
}

#[no_mangle]
pub unsafe extern fn watcher_start(watcherffi: *mut WatcherFfi) -> bool {
    if watcherffi.is_null() {
        return false;
    }
    (&mut *watcherffi).0.start().is_ok()
}

#[no_mangle]
pub unsafe extern fn watcher_stop(watcherffi: *mut WatcherFfi) -> bool {
    if watcherffi.is_null() {
        return false;
    }
    (&mut *watcherffi).0.stop().is_ok()
}
