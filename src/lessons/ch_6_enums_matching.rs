pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Enums & Matching",
        "N/A",
        main,
        Some(learning_rust::tui_formatting::ModuleFlags::Lesson),
    );
}

fn main() {
    enums();
    plus_one(None);
    options();
    matching();
    matching_with_catchall_in_cases();
    catchall_patterns();
    iflet();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,

    address: String,
}

#[derive(Debug)]
enum IpAddrKindTwo {
    V4(u8, u8, u8, u8),

    V6(String),
}

#[derive(Debug)]
enum OptionExample<T> {
    // <T> is any type
    None, // equivalent of null

    Some(T), // equivalent of not-null
}

fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let address = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.168.0.1"),
    };

    println!("{:?}", address);

    let address_type_two = IpAddrKindTwo::V4(127, 168, 0, 1);

    println!("{:?}", address_type_two);

    let _some_number = Some(5);
    let _some_char = Some('e');

    let mut _absent_number: OptionExample<i32> = OptionExample::None; // this has no value
}

fn route(addr_type: IpAddrKind) {
    println!("{:?}", addr_type);
}

fn matching() -> u8 {
    enum Coin {
        Penny,

        Nickle,

        Dime,

        Quarter,

        HalfDollar,

        Dollar,
    }

    let coin = Coin::Penny;

    // match the value of the 'coin' variable
    match coin {
        Coin::Penny => {
            println!("Pretty penny!");
            1
            // Coin::Penny is the value of the 'coin' variable, so
            // it prints that string and returns 1.
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::HalfDollar => 50,
        Coin::Dollar => 100,
    }

    // match is exhaustive in rust. YOU MUST HANDLE EVERY CASE.
}

fn matching_with_catchall_in_cases() {
    enum Message {
        ChangeColor(u8, u8, u8),
        Echo(String),
        Move(u64),
        Quit,
    }

    /*
    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(_, _, _) => {
                self.color = (255, 0, 255);
            }
            Message::Echo(_) => {
                self.message = String::from("Hello world!");
            }
            Message::Quit => {
                self.quit = true;
            }
            Message::Move(_) => {
                self.position.x = 10;
                self.position.y = 15;
            }
        }
    } */
}

const fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn options() {
    const FIVE: Option<i32> = Some(5);
    const SIX: Option<i32> = plus_one(FIVE);
    const NONE: Option<i32> = plus_one(None);

    println!("{:?}, {:?}, {:?}", FIVE, SIX, NONE);
}

fn catchall_patterns() {
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    fn pattern_variable() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),

            // this binds the defaulted value to a variable ("other")
            // but the variable must be used.
            other => move_player(other),
        }
    }

    fn pattern_underscore_with_function_call() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),

            // this doesn't bind the defaulted value to a variable
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }

    fn pattern_underscore_without_function_call() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),

            // this doesn't bind the defaulted value to a variable
            // and it does nothing
            _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}

        reroll();
    }

    reroll();
    pattern_variable();
    pattern_underscore_with_function_call();
    pattern_underscore_without_function_call();
}

fn iflet() {
    fn without_iflet() {
        let config_max = Some(3u8);

        match config_max {
            // Assigns the value of config_max to max
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }
    }

    // Syntactic sugar for 'match' if you want to only match one pattern and
    // ignore all other cases
    fn with_iflet() {
        let config_max = Some(3u8);

        // If value isn't None, then assign config_max to max and do the code
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
    }

    without_iflet();
    with_iflet();
}
