#![allow(unused)]

fn main() {
    try_loops();
    let result = try_loops_ret_value();
    println!("loop result is {result}");
}

fn try_loops() {
    let mut i = 0;
    loop {
        if (i % 1000 == 0) {
            println!("hey! {i}");
        }
        i += 1;

        if i > 1_000_000 {
            break;
        }
    }
}

fn try_loops_ret_value() -> i32 {
    let mut i = 0;
    loop {
        if (i % 1000 == 0) {
            println!("hey! {i}");
        }
        i += 1;

        if i > 1_000_000 {
            break i * 2_000;
        }
    }
}
