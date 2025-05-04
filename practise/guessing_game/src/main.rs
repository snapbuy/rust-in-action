use std::io;

fn main() {
    println!("Guess the number");
    println!("input your number:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed");

    println!("you guessed: {guess}");
}
