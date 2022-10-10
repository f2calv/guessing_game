use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess...");

    //let mut guess = String::new();
    let mut guess;
    guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // match io::stdin().read_line(&mut guess) {
    //     Ok(n) => {
    //         println!("{n} bytes read");
    //         println!("{guess}");
    //     }
    //     Err(error) => println!("error: {error}"),
    // }

    println!("You guessed, {guess}");

    // let x = 5;
    // let y = 10;

    // println!("{x}");
    // println!("{y}");
    // println!("{x} and {y}");
    // println!("x = {} and y = {}", x, y);
}
