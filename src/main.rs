use std::io;

fn main() {
    println!("Guess the number!");
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

    println!("You guessed: {guess}");
}
