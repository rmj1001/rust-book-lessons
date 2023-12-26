use learning_rust::formatting::ModFlag;

pub fn fmt() {
    learning_rust::formatting::module_formatter(
        "Variables and Data Types",
        "N/A",
        main,
        Some(ModFlag::Lesson),
    );
}

fn main() {
    variables_and_mutability();
    scalar_data_types();
    compound_data_types();
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
