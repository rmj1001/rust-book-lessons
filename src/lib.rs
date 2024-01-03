#![allow(unused_assignments)]

pub mod tui_formatting {
    pub struct Module {
        pub name: String,
        pub code: fn(),
    }

    /// Create a string of dashes with desired length
    ///
    /// Example: create_line_string(3) // ---
    pub fn create_line_string(total_length: usize) -> String {
        let mut line_string: String = String::new();
        let mut current_length: usize = 1;

        while current_length <= total_length {
            line_string.push('-');
            current_length += 1;
        }

        line_string
    }

    /// Print a line of dashes to STDOUT.
    ///
    /// Default length is 80 characters
    pub fn print_line_string(total_length: Option<usize>) {
        let mut line_string: String = String::new();

        match total_length {
            None => line_string = create_line_string(80),
            Some(length) => line_string = create_line_string(length),
        }

        println!("{}", &line_string[..]);
    }

    /// Prints a header with a title, using a line of dashes on the top
    /// and bottom. The title is centered.
    pub fn page_header(title: &str) {
        crate::terminal::clear_screen();

        fn add_spaces_to_string(s: &mut String, spaces: usize) {
            let mut index = 0;

            while index < spaces {
                s.push(' ');

                index += 1;
            }
        }

        let mut header = String::new();

        header.push_str(&create_line_string(80));
        header.push('\n');

        let spaces_on_one_side = (80 - (title.len() + 2)) / 2 + 1;

        add_spaces_to_string(&mut header, spaces_on_one_side);

        header.push_str(title);

        header.push('\n');
        header.push_str(&create_line_string(80));

        println!("{}", header);
    }

    /// Equivalent to DOS "pause" command
    pub fn press_enter_to_continue() {
        println!("\n[PRESS RETURN/ENTER TO CONTINUE.]");
        let mut garbage = String::new();
        let _ = std::io::stdin().read_line(&mut garbage);
    }

    /// Returns the index of the element in the vector selected.
    pub fn dialogue_selector(
        options: &[String],
        default_index: usize,
        optional_prompt: Option<&str>,
    ) -> usize {
        match optional_prompt {
            Some(prompt) => dialoguer::Select::new()
                .with_prompt(prompt)
                .items(options)
                .default(0)
                .interact()
                .unwrap_or(default_index),
            None => dialoguer::Select::new()
                .with_prompt(
                    "Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it",
                )
                .items(options)
                .default(0)
                .interact()
                .unwrap_or(default_index),
        }
    }

    pub enum ModuleFlags {
        Lesson,
        Practice,
    }

    pub fn module_tui_formatter(
        name: &str,
        description: &str,
        code: fn(),
        flag: Option<ModuleFlags>,
    ) {
        let mut title: String = String::from("Rust Book ");

        match flag {
            None => {
                title.push_str("Module");
            }
            Some(flagnum) => match flagnum {
                ModuleFlags::Lesson => title.push_str("Lesson"),
                ModuleFlags::Practice => title.push_str("Practice"),
            },
        };

        let title = &title[..];

        page_header(title);
        println!("Lesson: {}", name);
        println!("Description: {}", description);
        print_line_string(None);
        println!("Output:\n");
        (code)();
        print_line_string(None);
        press_enter_to_continue();
    }
}

pub mod terminal {
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn exit() {
        clear_screen();
        std::process::exit(0);
    }
}
