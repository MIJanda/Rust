// PART 1: ask for user input, process that input, and check that the input is in the expected form
// PART 2: generating a random number
// PART 3: comparing the guess to the secret number
// PART 4: Allowing multiple guesses with looping

// import io from standard library
use std::io;
// import Rng from rand library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // print name of the game
   println!("GUESS THE NUMBER!");

   let secret_number = rand::thread_rng().gen_range(1..101);
   
    loop {
            // prompt user input
        println!("Please enter your guess below:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print guess
        println!("Your guess: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You Win!");
                break;
            }
        }
    }
}
