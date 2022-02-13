use std::io;

fn main() {
    println!("Lets play guessing game!!!");
    println!("Input your guess.");

    let mut guess = String::new(); // Allocates new String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess it {}", guess);
}
