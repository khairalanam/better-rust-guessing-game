use std::io;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess the number between 1 and 100: ");
        let mut user_input: String = String::new();

        io::stdin().read_line(&mut user_input).expect("Some Input Error.");

        let user_input: i32 = user_input.trim().parse().expect("Not an integer");

        if user_input < secret_number {
            println!("Too small!");
        } else if user_input > secret_number {
            println!("Too big!");
        } else {
            println!("You guessed it! It's {}!", secret_number);
            break;
        }
    }
}
