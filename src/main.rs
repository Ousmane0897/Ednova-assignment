
use  rand::Rng; //Import the Rng trait to use random number generation
use std::io; // Import the standard input/output library
use std::cmp::Ordering; // For comparing the player's guess with the target
fn main() {

    let randomly_generated_number =rand::thread_rng().gen_range(1..100);  // Generate a random number between 1 and 100
    let mut guess = String::new();
    let mut number_of_attemps = 0;

    println!("Welcome to the guessing game!");
    println!("Please give a number between 1 and 100");

    loop {
        

        // Read the player's input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to a number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, // If successful, store the number
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // Skip the rest of the loop and prompt the player again
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess with the secret number
        match guess.cmp(&randomly_generated_number) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number correctly.");
                break; // Exit the loop if the guess is correct
            }
        }
    }


}
