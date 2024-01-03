pub fn fmt() {
    use learning_rust::tui_formatting::ModuleFlags;
    learning_rust::tui_formatting::module_tui_formatter(
        "Ownership & Borrowing",
        "N/A",
        main,
        Some(ModuleFlags::Lesson),
    );
}

fn main() {
    ownership_info();
    references_and_borrowing();
}

pub fn ownership_info() {
    // The following is an example of "unsafe" code which will not compile
    // (value is read before defined)
    //
    // read(x);
    // let x = 10;

    // -------------------------------------------------------------------------
    // Ownership Rules
    // -------------------------------------------------------------------------

    // 1. Each value in Rust has a variable called it's owner.
    // 2. There may only be one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped from memory.

    // Example of variable ownership
    //
    {
        // s is not yet valid
        let mut s: &str = "hello world"; // s is now valid
        println!("{s}");
    } // s is no longer valid

    // Values on the stack are copied to new variables. See below:
    let x: i64 = 10;
    let _y: i64 = x; // x is still valid, y copied the value from x.

    // Below is a mutable string which is stored on the heap
    let mut hello: String = String::from("Hello World");

    // Values on the heap must be explicitly copied by method, not like how
    // values are copied from the stack. This transfers ownership of var
    // 'hello' data to a new var 'world'. 'hello' is no longer a valid variable.
    let mut world: String = hello;

    // Below is an example of a heap value clone
    let mut _helloworld: String = world.clone();

    print!("Testing");

    // If I tried to run the below, the compiler would panic because 'hello' was
    // rendered invalid by 'world'
    //
    // println!("{hello}")

    // Passing heap variables into functions also moves ownership to those
    // function parameters. In the below example, read_string() now owns the
    // value from 'hello', and 'hello' is no longer valid.
    //
    // let mut hello: String = String::from("Hello World");
    // read_string(hello);

    // If you want to let a function 'borrow' a value, not claim ownership, you
    // can pass in references. Use '&' to denote references. See below:
    //
    // let mut helloworld: String = String::from("Hello World");
    // read_string(&helloworld);
    //
    // fn read_string(str: &String) {
    //     println!("{:?}", &str);
    // }

    // References are mutable by default. See below to use a mutable reference:
    //
    // let mut helloworld: String = String::from("Hello World");
    // read_and_write_string(&mut helloworld);
    //
    // fn read_and_write_string(str: &mut String) {
    //     str.push_str(string: "!");
    // }

    // You may only have one mutable reference to a piece of data per scope.
    // The following would cause a data race at compile time, so this is
    // not safe.
    //
    // let mut helloworld: String = String::from("Hello World");
    // let r1: &mut String = &mut helloworld;
    // let r2: &mut String = &mut helloworld;
    //
    // Instead, you may have multiple immutable references.
    //
    // let r1: &String = &helloworld;
    // let r2: &String = &helloworld;
    //
    // You cannot have both mutable and immutable references to the same data
    // in the same scope. The below would cause the borrow checker to complain
    //
    // let r1: &String = &helloworld;
    // let r2: &mut String = &mut helloworld;
    //
    // To summarize:
    // 1. At any given time, you may have one mutable reference OR unlimited
    // immutable references.
    // 2. References MUST ALWAYS BE VALID.

    // Returning references to the stack:
    //
    // The below code would cause the compiler to complain because the reference
    // being returned does not live long enough to be returned outside the scope.
    //
    // fn return_a_string() -> &String {
    //     let s = String::from("Hello world");
    //     &s
    // }
    //
    // You can extend the lifetime by moving ownership of the string outside the
    // function:
    //
    // fn return_a_string() -> String {
    //     let s = String::from("Hello world");
    //     s
    // }
    //
    // You can also return a literal:
    //
    // fn return_a_string() -> &'static str {
    //     "Hello world"
    // }
    //
    // You could also use garbage collection by cloning the data and returning
    // the clone
    //
    // use std::rc::Rc;
    // fn return_a_string() -> Rc<String> {
    //     let s = Rc::new(String::from("Hello world"));
    //     Rc::clone(&s)
    // }
}

fn references_and_borrowing() {
    /*
     * A reference is like a pointer in that it’s an address we can follow to
     * access the data stored at that address; that data is owned by some other
     * variable. Unlike a pointer, a reference is guaranteed to point to a valid
     * value of a particular type for the life of that reference.
     *
     * A reference is notated as &var
     */

    /*
     * The Rules of References:
     *
     * 1. At any given time, you can have either one mutable reference or any
     * number of immutable references.
     *
     * 2. References must always be valid.
     */
    let side1: u64 = 10;
    let side2: u64 = 20;

    // this borrow's the values of side1 & side2, returning it to the owner
    // once the function has completed.
    calculate_area(&side1, &side2);

    // references are immutable by default.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    /*
    * The following is an example of dangled references.
    *
    * fn dangle() {
       let s: String = String::from("Dangled reference");
       &s // we return a reference to the String, s
      } // s goes out of scope so the reference returned points to invalid
        // memory
    *
    *
    */
}

fn calculate_area(s1: &u64, s2: &u64) -> u64 {
    s1 * s2
} // s1 & s2 out of scope, but not dropped since they were only borrowed
