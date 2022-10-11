#![allow(unused)]

fn main() {
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
