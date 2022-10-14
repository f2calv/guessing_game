fn main() {
    to_test(true);
    println!("Hello, world!");
}

fn to_test(output: bool) -> bool {
    output
}

#[cfg(test)] // The module is only compiled when testing.
mod test {
    use super::to_test;

    // This function is a test function. It will be executed and the test will succeed if the function exits cleanly.
    #[test]
    fn test_to_test_ok() {
        assert!(to_test(true));
    }

    // That test on the other hand will only succeed when the function panics.
    #[test]
    #[should_panic]
    fn test_to_test_fail() {
        assert!(!to_test(true));
    }
}
