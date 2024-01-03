pub fn fmt() {
    use learning_rust::tui_formatting::ModuleFlags;
    learning_rust::tui_formatting::module_tui_formatter(
        "Project Management",
        "N/A",
        main,
        Some(ModuleFlags::Lesson),
    );
}

fn main() {
    eat_at_restaurant();
}

/*
 * Packages: A Cargo feature that lets you build, test, and share crates
 *
 * Crates: A tree of modules that produces a library or executable
 *
 * Modules and use: Let you control the organization, scope, and privacy of
 * paths
 *
 * Paths: A way of naming an item, such as a struct, function, or module
 */

/*
 * CRATE
 *
 * Basically a file containing code (modules etc) that either becomes a
 * compiled binary, or a library of code for other crates.
 *
 * BINARIES contain a main() function
 * LIBRARIES do not contain a main() function.
 *
 * The crate root is a source file that the Rust compiler starts from and
 * makes up the root module of your crate (we’ll explain modules in depth in
 *  the “Defining Modules to Control Scope and Privacy” section).
 */

/*
 * PACKAGES
 *
 * Bundle of one or more crates containing `cargo.toml` which describes
 * how to build those crates.
 *
 * A package can contain unlimited binary crates, but only one library
 * crate.
 *
 * To create a package:
 * $ cargo new my-project
 *
 * To make multiple binary crates, place the files in src/bin.
 */

/*
 * Modules Cheat Sheet
 *
 * Here we provide a quick reference on how modules, paths, the
 * use keyword, and the pub keyword work in the compiler, and how most
 * developers organize their code. We’ll be going through examples of each
 * of these rules throughout this ch, but this is a great place to
 * refer to as a reminder of how modules work.
 *
 * Start from the crate root: When compiling a crate, the compiler first
 * looks in the crate root file (usually src/lib.rs for a library crate or
 * src/main.rs for a binary crate) for code to compile.
 *
 * Declaring modules: In the crate root file, you can declare new modules;
 * say, you declare a “garden” module with mod garden;. The compiler will
 * look for the module’s code in these places:
 *  - Inline, within curly brackets that replace the semicolon following
 *    mod garden
 *  - In the file src/garden.rs
 *  - In the file src/garden/mod.rs
 *
 * Declaring submodules: In any file other than the crate root, you can
 * declare submodules. For example, you might declare mod vegetables; in
 * src/garden.rs. The compiler will look for the submodule’s code within
 * the directory named for the parent module in these places:
 *  - Inline, directly following mod vegetables, within curly brackets
 *    instead of the semicolon
 *  - In the file src/garden/vegetables.rs
 *  - In the file src/garden/vegetables/mod.rs
 *
 * Paths to code in modules: Once a module is part of your crate, you can
 * refer to code in that module from anywhere else in that same crate, as
 * long as the privacy rules allow, using the path to the code. For example,
 * an Asparagus type in the garden vegetables module would be found at
 * crate::garden::vegetables::Asparagus.
 *
 * Private vs public: Code within a module is private from its parent
 * modules by default. To make a module public, declare it with pub
 *  mod instead of mod. To make items within a public module public as well,
 * use pub before their declarations.
 *
 * The use keyword: Within a scope, the use keyword creates shortcuts to
 * items to reduce repetition of long paths. In any scope that can refer to
 * crate::garden::vegetables::Asparagus, you can create a shortcut with use
 * crate::garden::vegetables::Asparagus; and from then on you only need to
 * write Asparagus to make use of that type in the scope.
 */

/* Module Example
   backyard
   ├── Cargo.lock
   ├── Cargo.toml
   └── src
       ├── garden
       │   └── vegetables.rs
       ├── garden.rs
       └── main.rs

*/

//------------------------------------------------------------------------------
// Below is `main.rs`

/*

# imports `Asparagus` from `src/garden/vegetables.rs`
use crate::garden::vegetables::Asparagus;

# tells the compiler to include code from `src/garden.rs`
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
*/

//------------------------------------------------------------------------------

/*
* Example of private module with sub modules.
* Everything in modules are private by default.
*
* Modules can hold functions, types, enums, structs, constants,
* traits, etc.
*
* Here is the structure of the code below:
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
*/
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// throwaway for module below

fn deliver_order() {}

// example of public module
pub mod back_of_house {
    use self::kitchen::cook;

    // public submodule
    // using pub on a module doesn't expose the inner contents, but instead only
    // allows ancestor modules to REFER to it. Inner contents MUST BE PUBLISHED
    // ALSO.
    pub mod kitchen {
        pub fn cook() {} // exposed

        fn clean() {} // not exposed
    }

    // Ancestor modules cannot refer to this or its inner contents.
    mod office {

        fn payroll() {}

        fn hiring() {}

        fn reports() {}
    }

    fn fix_incorrect_order() {
        cook();

        // Use 'super' to access code outside the parent scope
        super::deliver_order();
    }
}

// this function is now exposed as a public function (`pub`)
pub fn eat_at_restaurant() {
    // absolute path (`crate` is the root path)
    crate::lessons::ch_7_project_management::back_of_house::kitchen::cook();

    // relative path - the beginning of the path is at the same level of the
    // module tree as this function
    back_of_house::kitchen::cook();

    // the code above will cause the compiler to panic because the module
    // called was not published.
}

// example of public struct
// the fields and methods are private by default.
pub struct TestMessage {
    pub name: String, // publicly accessible

    memory_address: u64, // this is not accessible outside the parent scope
}

// All variants of Enums are public by default
pub enum Colors {
    RED,
    GREEN,
    BLUE,
}

// example of bringing paths into scope
// use crate::lessons::ownership;
// ownership::ownership_info();

mod testing {
    // in this scope you can't use previous use statements, so you need to
    // re-import them.

    // You can also import individual paths
    // ex. use super::ownership::ownership_info;

    // ownership_info();
}

// You cannot have multiple paths of the same name
mod multiple_same_name_paths {
    // use std::fmt;
    // use std::io as stdio; // you can import alias
    // pub use std::io as pubstdio; // you can export imports and aliases

    /*
    fn function1() -> fmt::Result {
        // --snip--
    }

    fn function2() -> io::Result<()> {
        // --snip--
    }
    */
}

// You can also do multiple imports per line:
// use std::{fmt, io as newio};

// also clean this up:
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// or you can import glob
// use std::fs::*;

/*
Note that you only need to load a file using a mod declaration once in your
module tree. Once the compiler knows the file is part of the project (and knows
where in the module tree the code resides because of where you’ve put the
mod statement), other files in your project should refer to the loaded
file’s code using a path to where it was declared.
*/
