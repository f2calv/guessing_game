#![allow(unused)]

fn main() {
    if_conditions();
    try_loops();
}

fn try_loops() {
    let mut i = 0;
    loop {
        if (i % 1000 == 0) {
            println!("hey! {i}");
        }
        i += 1;
    }
}

fn if_conditions() {
    let number = 33;

    if number < 5 {
        println!("condition was true");
    } else if (number == 10) {
        //do nothing
    } else if (number % 3 == 0) {
        println!("number was divisible by 3");
    } else {
        println!("condition was false");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("the value of number is: {number}");
}
