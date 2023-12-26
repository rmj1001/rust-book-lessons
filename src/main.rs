use learning_rust::tui_formatting::*;
use std::collections::BTreeMap;

pub mod lessons {
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
            0 => lessons_menu(),
            1 => practice_modules_menu(),
            2 => learning_rust::terminal::exit(),
            _ => {
                println!("Expected valid input.");
                learning_rust::terminal::exit();
            }
        }
    }
}

fn lessons_menu() {
    loop {
        let lessons = lessons_cache_builder();
        page_header("Rust Book Lesson Repository");

        let mut options = Vec::new();

        for value in lessons.values() {
            options.push(value.name.clone());
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

        let choice = choice as u8;

        let lesson_option = lessons.get(&choice);

        match lesson_option {
            None => {
                println!("Invalid lesson.");
                learning_rust::tui_formatting::press_enter_to_continue();
                continue;
            }

            Some(lesson) => (lesson.code)(),
        }
    }
}

fn practice_modules_menu() {
    loop {
        let practice_modules = practice_mods_cache_builder();
        page_header("Rust Book Practice Module Repository");

        let mut options = Vec::new();

        for value in practice_modules.values() {
            options.push(value.name.clone());
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

        let choice = choice as u8;

        let practice_mod_option = practice_modules.get(&choice);

        match practice_mod_option {
            None => {
                println!("Invalid practice_module.");
                learning_rust::tui_formatting::press_enter_to_continue();
                continue;
            }

            Some(practice_module) => (practice_module.code)(),
        }
    }
}

fn lessons_cache_builder() -> BTreeMap<u8, MenuEntry> {
    let mut menu_cache: BTreeMap<u8, MenuEntry> = BTreeMap::new();

    menu_cache.insert(
        0,
        MenuEntry {
            name: string_slice_to_string("1. Hello World"),
            code: lessons::ch_1_hello_world::fmt,
        },
    );

    menu_cache.insert(
        1,
        MenuEntry {
            name: string_slice_to_string("2. Guessing Game"),
            code: lessons::ch_2_guessing_game::fmt,
        },
    );

    menu_cache.insert(
        2,
        MenuEntry {
            name: string_slice_to_string("3. Common Programming Concepts"),
            code: lessons::ch_3_common_concepts::fmt,
        },
    );

    menu_cache.insert(
        3,
        MenuEntry {
            name: string_slice_to_string("4. Ownership & Borrowing"),
            code: lessons::ch_4_ownership::fmt,
        },
    );

    menu_cache.insert(
        4,
        MenuEntry {
            name: string_slice_to_string("5. Structs & Tuple Structs"),
            code: lessons::ch_5_structs::fmt,
        },
    );

    menu_cache.insert(
        5,
        MenuEntry {
            name: string_slice_to_string("6. Enums and Matching"),
            code: lessons::ch_6_enums_matching::fmt,
        },
    );

    menu_cache.insert(
        6,
        MenuEntry {
            name: string_slice_to_string("7. Project Management"),
            code: lessons::ch_7_project_management::fmt,
        },
    );

    menu_cache.insert(
        7,
        MenuEntry {
            name: string_slice_to_string("8. Common Collections"),
            code: lessons::ch_8_common_collections::fmt,
        },
    );

    menu_cache.insert(
        8,
        MenuEntry {
            name: string_slice_to_string("9. Error Handling"),
            code: lessons::ch_9_error_handling::fmt,
        },
    );

    menu_cache
}

fn practice_mods_cache_builder() -> BTreeMap<u8, MenuEntry> {
    let mut menu_cache: BTreeMap<u8, MenuEntry> = BTreeMap::new();

    menu_cache.insert(
        0,
        MenuEntry {
            name: string_slice_to_string("1. Celcius"),
            code: practice::pr_1_celcius::fmt,
        },
    );
    menu_cache.insert(
        1,
        MenuEntry {
            name: string_slice_to_string("2. Rectangle"),
            code: practice::pr_2_rectangle::fmt,
        },
    );

    menu_cache
}
