use learning_rust::{terminal::clear_screen, tui_formatting::*};

pub mod lessons {
    pub mod ch_10_generics_traits_lifetimes;
    pub mod ch_1_hello_world;
    pub mod ch_2_guessing_game;
    pub mod ch_3_common_concepts;
    pub mod ch_4_ownership;
    pub mod ch_5_structs;
    pub mod ch_6_enums_matching;
    pub mod ch_7_project_management;
    pub mod ch_8_common_collections;
    pub mod ch_9_error_handling;
}

pub mod practice {
    pub mod pr_1_celcius;
    pub mod pr_2_rectangle;
}
fn main() {
    loop {
        page_header("Rust Book Code Repository");

        let options: Vec<String> = vec![
            "1. Lessons Menu".to_string(),
            "2. Practice Modules Menu".to_string(),
            "NAV: Exit".to_string(),
        ];

        let choice: usize = dialogue_selector(&options, 2);

        match choice {
            0 => modules_menu(ModuleType::Lesson),
            1 => modules_menu(ModuleType::Practice),
            2 => learning_rust::terminal::exit(),
            _ => {
                println!("Expected valid input.");
                learning_rust::terminal::exit();
            }
        }
    }
}
enum ModuleType {
    Lesson,
    Practice,
}

fn modules_menu(menu_type: ModuleType) {
    loop {
        clear_screen();

        let modules: Vec<Module>;
        let mut header: String = String::new();

        match menu_type {
            ModuleType::Lesson => {
                modules = new_lessons_cache();
                header.push_str("Rust Book Lesson Repository");
            }

            ModuleType::Practice => {
                modules = new_practice_cache();
                header.push_str("Rust Book Practice Repository");
            }
        }

        page_header(&header);

        let mut options = Vec::new();

        for entry in &modules {
            options.push(entry.name.clone());
        }

        options.push("NAV: Main Menu".to_string());
        options.push("NAV: Exit".to_string());

        let choice = dialogue_selector(&options, options.len() - 1);

        if choice == (options.len() - 2) {
            #[allow(clippy::main_recursion)]
            main();
        }

        if choice == (options.len() - 1) {
            learning_rust::terminal::exit();
        }

        let module_option = modules.get(choice);

        match module_option {
            Some(module) => {
                (module.code)();
            }

            None => {
                println!("Invalid module.");

                match menu_type {
                    ModuleType::Lesson => modules_menu(ModuleType::Lesson),
                    ModuleType::Practice => modules_menu(ModuleType::Practice),
                }
            }
        };
    }
}

fn new_lessons_cache() -> Vec<Module> {
    vec![
        Module {
            name: "1. Hello World".to_string(),
            code: lessons::ch_1_hello_world::fmt,
        },
        Module {
            name: "2. Guessing Game".to_string(),
            code: lessons::ch_2_guessing_game::fmt,
        },
        Module {
            name: "3. Common Programming Concepts".to_string(),
            code: lessons::ch_3_common_concepts::fmt,
        },
        Module {
            name: "4. Ownership & Borrowing".to_string(),
            code: lessons::ch_4_ownership::fmt,
        },
        Module {
            name: "5. Structs & Tuple Structs".to_string(),
            code: lessons::ch_5_structs::fmt,
        },
        Module {
            name: "6. Enums and Matching".to_string(),
            code: lessons::ch_6_enums_matching::fmt,
        },
        Module {
            name: "7. Project Management".to_string(),
            code: lessons::ch_7_project_management::fmt,
        },
        Module {
            name: "8. Common Collections".to_string(),
            code: lessons::ch_8_common_collections::fmt,
        },
        Module {
            name: "9. Error Handling".to_string(),
            code: lessons::ch_9_error_handling::fmt,
        },
        Module {
            name: "10. Generic Types, Traits, Lifetimes".to_string(),
            code: lessons::ch_10_generics_traits_lifetimes::fmt,
        },
    ]
}

fn new_practice_cache() -> Vec<Module> {
    vec![
        Module {
            name: "1. Celcius".to_string(),
            code: practice::pr_1_celcius::fmt,
        },
        Module {
            name: "2. Rectangle".to_string(),
            code: practice::pr_2_rectangle::fmt,
        },
    ]
}
