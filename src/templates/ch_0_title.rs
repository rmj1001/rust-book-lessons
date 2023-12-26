pub fn fmt() {
    learning_rust::formatting::module_formatter(
        "Title",
        "Description",
        main,
        Some(learning_rust::formatting::ModFlag::Lesson),
    );
}

fn main() {}
