#![allow(unused)]

fn main() {
    println!("Hello, world!");

    another_function(4, 'w');
    //let x = (let y = 6);

    let x = {
        let y = 6;
        y + 1 + 1_0_0_0_0_0_0
    };

    println!("the value of x is: {x}");

    {
        let x = five();
        println!("the value of x is: {x}");

        let y = plus_one(x);
        println!("the value of y is: {y}"); //this is a statement (ends with semi-colon)
    }
}

//here is a really long comment which needs
//to be multiline
fn another_function(x: u32, unit_label: char) {
    println!("The value of x is: {x}, char is: {unit_label}");
}

fn five() -> i32 {
    5 //this is an expression
}

fn plus_one(value: i32) -> i32 {
    value + 1
}
