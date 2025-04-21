use rand::Rng;

fn get_random_number() -> i32{
    let mut rng_generator = rand::rng(); // must be mut because random number generators (like ThreadRng) update their internal state after generating each number to maintain randomness.
    let rng_number = rng_generator.random_range(1..=10);

    rng_number
}

fn main() {
    let random_number = get_random_number();

    loop {
        println!("Guess the number (between 1 and 10):");

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess == random_number {
            println!("Congratulations! You guessed the correct number: {}", random_number);
            break;
        } else if guess < random_number {
            println!("Too low! Try again.");
        } else {
            println!("Too high! Try again.");
        }
    }
}
