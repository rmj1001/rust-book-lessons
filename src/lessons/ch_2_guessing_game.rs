use rand::Rng;
use std::cmp::Ordering;
use std::io;

use learning_rust::tui_formatting::press_enter_to_continue;

pub fn fmt() {
    use learning_rust::tui_formatting::ModuleFlags;
    learning_rust::tui_formatting::module_tui_formatter(
        "Guessing Game",
        "N/A",
        main,
        Some(ModuleFlags::Lesson),
    );
}

fn main() {
    press_enter_to_continue();
    learning_rust::terminal::clear_screen();
    println!("-----------------------------------------");
    println!("Guess the number!");
    println!("-----------------------------------------\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input a number between 1 and 100.");

        let mut guess = String::new();

        // This appends a string read from stdin to an existing string
        io::stdin()
            .read_line(&mut guess)
            .expect("Please input text!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // stdin::io::stdin also works if you don't use the import statement at the
        // top. (The first 'stdin' is the library call)

        // Print guess variable to stdout
        println!("\nYou guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
