#[cfg(test)]
mod test {
    const OUT_DIR: &str = env!("OUT_DIR");

    #[test]
    fn test_out_dir() {
        println!("main() printing OUT_DIR -> {}", OUT_DIR);
    }
}
