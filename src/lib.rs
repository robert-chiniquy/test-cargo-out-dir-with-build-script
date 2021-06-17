pub const IMPORT_THIS: &str = "Testing if build.rs importing code from src/ triggers it";

#[cfg(test)]
mod test {
    const OUT_DIR: &str = env!("OUT_DIR");

    #[test]
    fn test_out_dir() {
        println!("main() printing OUT_DIR -> {}", OUT_DIR);
    }
}
