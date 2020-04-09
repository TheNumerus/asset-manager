use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::generate(".")
        .expect("Unable to build bindings")
        .write_to_file("copydog.h");
}