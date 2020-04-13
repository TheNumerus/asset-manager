fn main() {
    cbindgen::generate(".")
        .expect("Unable to build bindings")
        .write_to_file("copydog.h");
}