use crate::*;

use std::ffi::CString;

const CONFIG: &str = r#"
source ="."
[jpg]
target="images"
"#;

#[test]
pub fn qt_sim() {
    let config = CString::new(CONFIG).unwrap();
    unsafe {
        let c = config_generate(config.as_ptr());
        let w = watcher_new(c);
        watcher_start(w);
        watcher_stop(w);
        config_free(c);
        watcher_free(w);
    }
}