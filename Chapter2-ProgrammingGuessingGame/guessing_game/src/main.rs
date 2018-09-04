// Use the "rand" crate that was added to the Cargo.toml file.s
extern crate rand;

// This is necessary to specify which _traits_ the package is using. We won't just know this by
// default. You need to read the documentation for the crate to figure out what traits need to be
// specified.
// You can also run cargo doc --open to open the documentation for all crates locally using a web
// browser.
use rand::Rng;
// Ordering is another enum, but the values are Less, Greater, or Equal.
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // Variables in rust are immutable by default.
        // 'mut' makes a variable mutable.

        // The :: on String indicates this is an _associated function_ of the String class, meaning
        // it is called on the String class, rather than on an instance. Also known as a static
        // method in other languages.
        let mut a_guess = String::new();

        // The ampersand indicates this is a reference to guess that's being passed to the function.
        // References, like variables, are immutable by default. So, we need to tell it to make the
        // reference mutable with '&mut'.
        io::stdin()
        .read_line(&mut a_guess)
        // .read_line returns an io::Result, which is an enum
        // The possible values are Ok or Err. This is to encode failure conditions more succinctly.
        // If the value is Err, then expect() will crash and report the message to the stdout.
        .expect("Failed to read a_guess");

        // We need to convert guess to a number in order to perform the comparison later on. Note that
        // the name 'guess' is _shadowed_, allowing us to basically replace it without coming up with
        // another variable name.
        let a_guess: u32 = match a_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", a_guess);

        // A match expression consists of arms. Basically, it's a switch statement in other languages.
        match a_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
