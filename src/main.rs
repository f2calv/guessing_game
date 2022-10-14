#![allow(unused)]
fn main() {
    to_test(true);
    println!("Hello, world!");

    //let mut s: String;
    {
        let mut s = String::from("hello");

        s.push_str(" 123");

        println!("{}", s);
    }
    //println!("{}", s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("s1 = {}, s2 = {}", s1, s2);
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
