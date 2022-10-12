#![allow(unused)]

fn main() {
    try_loops();
    let result = try_loops_ret_value();
    println!("loop result is {result}");

    loop_labels();

    while_loop();

    while_array();

    for_array();

    for_range();
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

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                println!("exit all loops now...");
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut counter = 10;
    while counter != 0 {
        println!("counter={counter}");
        counter -= 1;
    }
    println!("EXITED while loop...")
}

fn while_array() {
    let a = [12, 13, 14, 15];

    let mut index = 0;

    while index < a.len() {
        let val = a[index];
        println!("index={index}, val={val}");
        index += 1;
    }
}

fn for_array() {
    let a = [12, 13, 14, 15];
    for val in a {
        println!("val={val}");
    }
}

fn for_range() {
    for number in (1..4) {
        println!("number={number}");
    }

    for number in (1..4).rev() {
        println!("number={number}");
    }
}
