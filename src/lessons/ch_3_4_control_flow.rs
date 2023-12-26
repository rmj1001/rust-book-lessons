pub fn fmt() {
    learning_rust::formatting::module_formatter(
        "Control Flow",
        "If/Else and Loops",
        main,
        Some(learning_rust::formatting::ModFlag::Lesson),
    );
}

fn main() {
    ifelse();
    loops();
}

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
