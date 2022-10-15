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

    {
        let mut x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
        x += 1;
        println!("x = {}, y = {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("hello"); //s comes into scope
        takes_ownership(s); //s's value moves into the function... and is no longer valid here
                            //println!("{s}");

        let x = 5; //x comes into scope
        makes_copy(x); //x would move into the function but i32 type has Copy trait so it's OK to use x afterwards
    }

    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s = takes_and_gives_back(s2);
    }

    {
        let s1 = String::from("hello");
        let (s2, length) = calculate_length(s1);
        println!("the length of '{}' is {}.", s2, length);
    }

    {
        let s1 = String::from("hello");
        let length = calculate_length2(&s1);
        println!("the length of '{}' is {}.", s1, length);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{s}");
    }

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s;
        //println!("{r1}, {r2}");
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;
        println!("{r1}, {r2}, {r3}");
    }
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    println!("{some_integer}");
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
