#![allow(unused)]
#![allow(dead_code)]
use std::io;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    invalid_array_element_access();
    test_stuff();
}

fn invalid_array_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn test_stuff() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let _spaces = spaces.len();

    let _guess: u32 = "42".parse().expect("Not a number!");

    {
        let _x = 2.0; // f64
        let _y: f32 = 3.0; // f32
    }

    {
        // addition
        let _sum = 5 + 10;

        // subtraction
        let _difference = 95.5 - 4.3;

        // multiplication
        let _product = 4 * 30;

        // division
        let _quotient = 56.7 / 32.2;
        let _floored = 2 / 3; // Results in 0

        // remainder
        let _remainder = 43 % 5;
    }

    //booleans
    {
        let _t = true;
        let _f: bool = false; // with explicit type annotation
    }
    //characters
    {
        let _c = 'z';
        let _z: char = 'ℤ'; // with explicit type annotation
        let _heart_eyed_cat = '😻';
    }
    //tuples
    {
        let _tup: (i32, f64, u8) = (500, 6.4, 1);

        let tup2 = (500, 6.4, 1);

        let (_x, y, _z) = tup2;

        println!("The value of y is: {y}");
    }
    {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let _five_hundred = x.0;

        let _six_point_four = x.1;

        let _one = x.2;
    }

    //arrays
    {
        let _a = [1, 2, 3, 4, 5];

        let _b = [(500, 6.4, 1), (600, 7.4, 2)];

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];

        let a2 = [3; 5];
        let a2 = [-0.0897; 5];
    }
}
