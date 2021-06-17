use std::env;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!(
        "cargo:warning=build.rs printing env:OUT_DIR -> {:?}",
        out_dir
    );
}
