use std::env;

#[path = "src/lib.rs"]
mod lib;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    println!("cargo:warning={:?}", lib::IMPORT_THIS);
    println!(
        "cargo:warning=build.rs printing env:OUT_DIR -> {:?}",
        out_dir
    );
}
