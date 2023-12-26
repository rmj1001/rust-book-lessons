pub mod tui_formatting {
    pub fn create_line_string(length: usize) -> String {
        let mut line_string: String = String::new();
        let mut index: usize = 0;

        while index < length {
            line_string.push('-');
            index += 1;
        }

        line_string
    }

    /// Default length is 80 characters
    pub fn print_line_string(line_length: Option<usize>) {
        #[allow(unused_assignments)]
        let mut line_string: String = String::new();

        match line_length {
            None => line_string = create_line_string(80),
            Some(length) => line_string = create_line_string(length),
        }

        println!("{}", &line_string[..]);
    }

    /// Prints a header with a title spanning 80 characters.
    pub fn page_header(title: &str) {
        crate::terminal::clear_screen();

        fn add_spaces_to_string(s: &mut String, spaces: usize) {
            let mut index = 0;

            while index < spaces {
                s.push(' ');

                index += 1;
            }
        }

        let mut header_str = String::new();

        let spaces_on_one_side = (80 - (title.len() + 2)) / 2;

        header_str.push('|');

        add_spaces_to_string(&mut header_str, spaces_on_one_side);

        header_str.push_str(title);

        add_spaces_to_string(&mut header_str, spaces_on_one_side);

        if header_str.len() == 78 {
            header_str.push_str(" |");
        } else {
            header_str.push('|');
        }

        print_line_string(None);
        println!("{}", header_str);
        print_line_string(None);
    }

    /// Equivalent to DOS "pause" command
    pub fn press_enter_to_continue() {
        println!("\n[PRESS RETURN/ENTER TO CONTINUE.]");
        let mut garbage = String::new();
        let _ = std::io::stdin().read_line(&mut garbage);
    }

    pub fn prompt_for_input(prompt_string: &str) -> String {
        let input = dialoguer::Input::new()
            .with_prompt(format!("{} > ", prompt_string))
            .interact_text();
        let mut returned: String = String::new();

        match input {
            Ok(output) => returned = output,
            Err(_) => {
                println!("Expected input");
                crate::terminal::exit();
            }
        };

        returned
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
        println!("\nLesson: {}", name);
        println!("Description: {}\n", description);
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
