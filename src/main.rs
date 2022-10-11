#![allow(unused)]

fn main() {
    println!("Hello, world!");

    another_function(4, 'w');
    //let x = (let y = 6);

    let x = {
        let y = 6;
        y + 1 + 1_0_0_0_0_0_0
    };

    println!("the value of x is: {x}")
}

fn another_function(x: u32, unit_label: char) {
    println!("The value of x is: {x}, char is: {unit_label}");
}
