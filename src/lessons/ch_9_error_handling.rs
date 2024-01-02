use learning_rust::tui_formatting::print_line_string;
use std::io::{Error, Read};

pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Error Handling",
        "N/A",
        main,
        Some(learning_rust::tui_formatting::ModuleFlags::Lesson),
    );
}

fn main() {
    //? There are two kinds of errors, recoverable errors (not bugs, more than)
    //? (likely user error) and unrecoverable errors (bugs in code). Rust handles
    //? each separately, using Result<T,E> for recoverables, panic!() for
    //? unrecoverables.

    println!("Recoverable Errors:");
    recoverable_errors();

    print_line_string(Some(40));
    println!("To panic or not to panic:");
    to_panic_or_not();

    print_line_string(Some(40));

    println!("Unrecoverable Error (panic):");
    unrecoverable_errors();
}

fn unrecoverable_errors() {
    //? panic!() macro causes the program to stop entirely due to an unrecoverable
    //? error. It can either be called explicitly by the developer, or called
    //? implicitly when code does something it shouldn't causing Rust itself to
    //? panic (such as accessing memory outside the boundaries of an array).

    //? UNWINDING
    //?
    //? By default, Rust will unwind when it panics, meaning it will travel up
    //? the stack and clean up data it left behind. You can choose instead to
    //? abort upon panic and leave the Operating System to clean up the memory for
    //? you. Add the following line to Caro.toml for aborting:
    //? [profile.release]
    //? panic='abort'

    panic!("Crash and Burn");

    //? Panic Output Example:

    //? ```
    //? thread 'main' panicked at src/lessons/ch_9_error_handling.rs:47:5:
    //? Crash and Burn
    //? note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //?
    //? RUST_BACKTRACE=1 provides a backtrace with debug symbols
    //? ```

    //? As you can see above, the backtrace points to not only where the function
    //? panicked, but also which line and character the panic began, along with an
    //? error message. This can be very useful for finding where bugs are in code.

    /* Example of Implicit Panic (caused by library)
    let v: Vec<i32> = vec![1, 2, 3];

    v[99]; // this will panic because it accesses memory outside the vec's bounds
    */

    // Debug symbols are enabled by default when you run/compile the code with
    // `cargo run` or `cargo build` without the `--release` flag
}

fn recoverable_errors() {
    //? Recoverable errors are handled as an enum:
    //? enum Result {
    //?     Ok(T),
    //?     Err(E),
    //? }

    //? T and E are handled as generic type parameters, T being the OK value
    //? and E being the Error value

    // Example of an operation returning a Result wrappe
    use std::fs::File;
    let path: &str = "hello.txt";
    let file_result = File::open(path);

    match file_result {
        Ok(file) => {
            println!("{:?}", file);
        }
        Err(error) => println!("Problem opening file {path}. Error: {error}"),
    }

    // Or:
    // let file_result = match file_result {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening file {path}. Error: {error}"),
    // }

    /* MATCHING OTHER ERROR KINDS */
    use std::io::ErrorKind;
    let file_result = File::open(path);

    let file = match file_result {
        Ok(buffer) => buffer,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(f) => f,
                Err(error) => panic!("Could not find or create '{path}': {error:?}"),
            },
            error => panic!("Problem opening file '{path}'. Error: {error:?}"),
        },
    };

    println!("{file:?}");

    /* Using .unwrap() for cleaner code */
    let file: File = File::open(path).unwrap(); // this will panic if not successful
    println!("{file:?}");

    /* Using .expect() to handle an error */
    let file: File = File::open(path).expect("{path} should be included in the project.");
    // the above will send the string literal to the panic message

    println!("{file:?}");

    //? QUESTION MARK OPERATOR
    //?
    //? '?' is only allowed in functions that return type 'Result',
    //? 'Option', or another type that implements `FromResidual`.

    fn using_question_mark_for_results() -> Result<String, Error> {
        //? '?' Is used for early-returning Errors to functions
        //?
        //? Example:
        //? let mut username_file = File::open("./helper-files/username.txt")?;
        //? let mut username = String::new();
        //? username_file.read_to_string(&mut username)?;

        // You can also chain multiple ?
        let mut username: String = String::new();
        File::open("./src/lessons/helper-files/username.txt")?.read_to_string(&mut username)?;

        // the above can also be shortened further:
        // let username: String = std::fs::read_to_string(path);

        Ok(username)
    }

    fn using_question_mark_for_options(text: &str) -> Option<char> {
        //? When the '?' operator is used on an Option<T> type, it returns
        //? early if the value is 'None'. If the value is 'Some(T)', the value
        //? inside 'Some(T)' is the resulting value of the expression and the
        //? function will continue.

        // this returns the last character of the first line of text
        text.lines().next()?.chars().last()

        // this returns Option<char> since there is possibly no character
    }

    println!("{:?}", using_question_mark_for_results());
    println!("{:?}", using_question_mark_for_options("text"));
}

fn to_panic_or_not() {
    //? Guidelines for Error Handling
    //? -----------------------------
    //? 1. Make your code panic when:
    //?  - Some assumption, guarantee, contract, or invariant has been broken.
    //?  - The bad state is something __unexpected__
    //?  - Your code needs to stop checking for every problem at every step
    //?  - There isn't a good way to encode information in the types you use.
    //?  - Calling external code out of your control and it returns invalid
    //?    state which you can't fix.
    //? 2. Handle errors when __failure is expected__.
    //? 3. Create custom types for validating data and handling errors with the
    //?    compiler.

    #[allow(dead_code)]
    struct Guess {
        value: i32,
    }

    impl Guess {
        #[allow(dead_code)]
        pub fn new(value: i32) -> Option<Guess> {
            #[allow(clippy::manual_range_contains)]
            if value >= 1 && value <= 100 {
                return Some(Guess { value });
            }

            None
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
