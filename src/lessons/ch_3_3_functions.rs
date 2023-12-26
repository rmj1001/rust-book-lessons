pub fn fmt() {
    learning_rust::formatting::module_formatter(
        "Functions",
        "N/A",
        main,
        Some(learning_rust::formatting::ModFlag::Lesson),
    );
}

// -------------------------------------------------------------------------
// Calling Functions
// -------------------------------------------------------------------------

fn main() {
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
