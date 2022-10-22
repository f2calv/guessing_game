#![allow(unused)]

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); //this empties the string making it equal to ""

    //word is now a number, but cannot be linked directly to a string as 's' is now emptied

    to_test(true);
    //println!(s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
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
