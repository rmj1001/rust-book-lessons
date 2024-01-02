pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Title",
        "Description",
        main,
        Some(learning_rust::tui_formatting::ModuleFlags::Practice),
    );
}

fn main() {}
