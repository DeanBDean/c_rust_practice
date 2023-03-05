use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config: cbindgen::Config = Default::default();
    cbindgen::generate_with_config(crate_dir, config)
      .unwrap()
      .write_to_file("../../target/example.h");
}