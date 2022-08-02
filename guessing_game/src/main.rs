use std::io;
fn main() {
    println!("Enter into the word Guessing Game!\n");
    println!("What is your guess?");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("String error");
    println!("Your guess is {}", guess);

}
