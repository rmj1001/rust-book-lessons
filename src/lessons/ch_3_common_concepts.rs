pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Common Programming Concepts",
        "Variables/Mutability, Data types, Functions, If/Else, Loops",
        main,
        Some(learning_rust::tui_formatting::ModuleFlags::Lesson),
    );
}

fn main() {
    variables_and_mutability();
    scalar_data_types();
    compound_data_types();
    functions();
    ifelse();
    loops();
}

// -------------------------------------------------------------------------
// Variables and Mutability
// -------------------------------------------------------------------------
fn variables_and_mutability() {
    // Immutable variable (default)
    // let yes: u32 = 1;

    // Mutable variable
    // let mut yes: u32 = 1;
    // yes = 2;

    let mut yes: u32 = 1;
    println!("Mutable: {yes}");

    yes = 2;
    println!("Mutable: {yes}");

    // The below is a constant. They are always immutable, MUST be declared with
    // a type, and can be defined in global scope.
    const THIS_IS_CONSTANT_NAMING_CONVENTION: u32 = 0;
    println!("Constants: {THIS_IS_CONSTANT_NAMING_CONVENTION}");

    // You can re-declare variables with new values. This is called "Shadowing".
    // Shadowing only lasts until being shadowed or the scope ends. Only 'let'
    // vars may be shadowed. Shadowing also allows for type changing.
    //
    let yes: u32 = 1; // this is 1
    println!("Shadowing: {yes}");
    let yes: u32 = 2; // this is now 2
    println!("Shadowing: {yes}");

    // (w/ type change)
    let spaces = "      ";
    println!("Shadowing w/ Type Change: '{spaces}'");
    let spaces = spaces.len();
    println!("Shadowing w/ Type Change: {spaces}");
}

fn scalar_data_types() {
    // Scalars are single-value data types (ints, floats, bools, chars)

    // -------------------------------------------------------------------------
    // Integer Types
    // -------------------------------------------------------------------------
    // (i for signed, u for unsigned)
    // (signed means positive/negative)
    //
    // 8 bit: i8, u8
    // 16 bit: i16, u16
    // 32 bit: i32, u32
    // 64 bit: i64, u64
    // 128 bit: i128, u128
    // arch: isize, usize
    //
    // Signed variants: From -(2^(n-1)) to 2^(n-1)-1
    // Unsigned variants: From 0 to (2^n)-1
    //
    // Additionally, the isize and usize types depend on the architecture of
    // the computer your program is running on, which is denoted in the table as
    // “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re
    // on a 32-bit architecture.
    //
    // Default integer type is i32.
    let unsigned_int_32: u32 = 10;
    println!("Integers: {unsigned_int_32}");

    // -------------------------------------------------------------------------
    // Floating Points
    // -------------------------------------------------------------------------
    // Types: f32, f64 (both signed), f64 is default and more precise.
    // f32 is single, f64 is double.
    let floating_64: f64 = 32.2;
    println!("Floating Points: {floating_64}");

    // -------------------------------------------------------------------------
    // Numeric Operations
    // -------------------------------------------------------------------------
    // addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");

    // remainder (after dividing, what is the whole number remainder)
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");

    // -------------------------------------------------------------------------
    // Booleans
    // -------------------------------------------------------------------------
    let truthy: bool = true;
    println!("True? {truthy}");

    let falsey: bool = false;
    println!("Falsey? {falsey}");

    // -------------------------------------------------------------------------
    // Character
    // -------------------------------------------------------------------------
    let d: char = 'd';
    println!("Character: {d}");

    let emoji: char = '😀';
    println!("Character: {emoji}");
}

fn compound_data_types() {
    // -------------------------------------------------------------------------
    // Tuples
    // -------------------------------------------------------------------------
    // (Grouping together mixed types into fixed-length compound type)
    let tup = ('a', 3, false);
    println!("Tuple: {:?}", tup);

    // with explicit typing
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", x);

    // -------------------------------------------------------------------------
    // Arrays
    // -------------------------------------------------------------------------
    // (Grouping together elements of same type into fixed-length compound type)
    // (Useful if you want your data allocated on stack rather than the heap)
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Array: {:?}", months);

    let ints: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", ints);

    let first = ints[0];
    println!("Array Index 0: {first}");

    // This creates an array containing one hundred 1's.
    let one_hundred_ones = [1; 100];
    println!("{:?}", one_hundred_ones);
}

// -------------------------------------------------------------------------
// Calling Functions
// -------------------------------------------------------------------------

fn functions() {
    println!("This is the main function.");
    snake_case_is_the_norm();

    let z = add(1, 2);
    println!("Added 1+2 = {z}");

    let x = {
        let y = 5;
        y + 1
    };

    println!("{:?}", x);

    println!("{:?}", five());
}

// -------------------------------------------------------------------------
// Defining functions & naming conventions
// -------------------------------------------------------------------------

fn snake_case_is_the_norm() {
    println!("snake_case_is_the_norm_for_function_names.");
}

// -------------------------------------------------------------------------
// Functions returning expressions
// -------------------------------------------------------------------------

// If a function returns an expression (value), it must declare the type
// returned.
fn five() -> i32 {
    5
}

// Don't add ';' or it will be a statement (code run only), not a returned
// expression
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// -------------------------------------------------------------------------
// Control Flow
// -------------------------------------------------------------------------
fn ifelse() {
    // -------------------------------------------------------------------------
    // If-Else statements
    // -------------------------------------------------------------------------

    let number: i32 = 5;

    if number < 10 {
        println!("{number} is less than 10.");
    } else {
        println!("{number} is not less than 10.");
    }

    if number != 0 {
        println!("{number} is not 0.");
    }

    // -------------------------------------------------------------------------
    // "else if" in conditional statements
    // -------------------------------------------------------------------------

    let min: i32 = 0;
    let max: i32 = 100;

    if number > max {
        println!("{number} is greater than {max}.");
    } else if number < min {
        println!("{number} is less than {min}.");
    } else {
        println!("{number} is between {min} and {max}.");
    }

    // -------------------------------------------------------------------------
    // Conditional Variable Assignments
    // -------------------------------------------------------------------------

    // You can assign a value to 'let' based on if statement
    // don't mismatch types in the condition returns (i.e. 5 and 'six')
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("{number}");
}

fn loops() {
    let mut counter = 1;

    loop {
        // this loop runs 10 times

        if counter < 10 {
            println!("{counter}: Again!");
            counter += 1;
            continue;
        } else {
            println!("{counter}: All done.");
            break;
        }
    }

    // -------------------------------------------------------------------------
    // Loop as variable value
    // -------------------------------------------------------------------------

    counter = 1; // reset counter

    let iters = loop {
        if counter < 10 {
            counter += 1;
            continue;
        } else {
            break counter;
        }
    };

    println!("{iters}");

    // -------------------------------------------------------------------------
    // Labeled Loops
    // -------------------------------------------------------------------------

    // 'counting: loop {
    //     // this is a labeled loop which you can continue or break inside inner
    //     // loops
    //     let mut counter: u32 = 1;
    //     loop {
    //         if counter < 5 {
    //             counter += 1;
    //             continue;
    //         } else {
    //             // breaking the outer loop which was labeled
    //             break 'counting;
    //         }
    //     }
    // }

    // -------------------------------------------------------------------------
    // Conditional Loops
    // -------------------------------------------------------------------------

    let mut counter = 0;

    while counter != 10 {
        println!("{counter} is not 10!");
        counter += 1;
    }

    // -------------------------------------------------------------------------
    // Looping over collections (arrays, tuples, vectors)
    // -------------------------------------------------------------------------

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
        println!("{element}");
    }

    println!("Blasting off in...");
    for number in (1..4).rev() {
        println!("{number}"); // 3, 2, 1
    }
    println!("Liftoff!");
}
