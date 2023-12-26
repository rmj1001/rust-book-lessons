use learning_rust::formatting::ModFlag;

pub fn fmt() {
    learning_rust::formatting::module_formatter(
        "Title",
        "Description",
        main,
        Some(ModFlag::Practice),
    );
}

pub fn main() {}
