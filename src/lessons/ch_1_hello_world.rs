/*
*   Learning Rust Project
*   Title: 1-hello_world
*   Description: Write "Hello World!" to screen
*/

pub fn fmt() {
    use learning_rust::tui_formatting::ModuleFlags;
    learning_rust::tui_formatting::module_tui_formatter(
        "Hello World",
        "N/A",
        main,
        Some(ModuleFlags::Lesson),
    );
}

fn main() {
    println!("Hello World!");
}
