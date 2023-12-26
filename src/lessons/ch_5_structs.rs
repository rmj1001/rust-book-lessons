use learning_rust::tui_formatting::ModuleFlags;

pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Structs",
        "N/A",
        main,
        Some(ModuleFlags::Lesson),
    );
}

//------------------------------------------------------------------------------
// Struct Example
//------------------------------------------------------------------------------

// Traditional Struct
struct User {
    active: bool,
    username: String,
    email: String,
    login_count: u64,
}

// Creating methods on a Struct
impl User {
    fn prettify(&self) {
        println!("User: {}", self.username);
        println!("Email: {}", self.email);
        println!("Is Active: {}", self.active);
        println!("Times logged in: {}\n", self.login_count);
    }
}

// This is an example of multiple impls & "associated functions" (String::new())
impl User {
    fn build(username: String, email: String) -> Self {
        Self {
            active: false,
            login_count: 0,
            username,
            email,
        }
    }

    // This is an example of a function changing a struct's data internally
    fn set_activity(&mut self, is_active: bool) {
        self.active = is_active
    }
}

// Tuple Structs
#[allow(dead_code)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

fn main() {
    // Individual fields cannot be marked mutable/immutable.
    let mut user1 = User {
        active: true,
        username: String::from("genesius"),
        email: String::from("someemail@gmail.com"),
        login_count: 1,
    };

    user1.login_count = 2;
    user1.email = String::from("invalid@gmail.com");

    // You can use functions to build structs
    let user2 = build_user(String::from("maddie"), String::from("idk@gmail.com"));

    // You can fill in fields in structs with data from other structs
    // but due to ownership this would delete the previous struct
    let user3 = User {
        active: user1.active,
        username: String::from("jeremy"),
        email: String::from("yasskingslay@gmail.com"),
        login_count: user1.login_count,
    };

    // Or just make a new struct with all fields from a previous struct, with
    // updates. This also deletes the previous struct
    let user4 = User {
        active: false,
        ..user2
    };

    #[allow(dead_code)]
    const BLACK: Color = Color(0, 0, 0);
    #[allow(dead_code)]
    const ORIGIN: Point = Point(0, 0, 0);

    user3.prettify();
    user4.prettify();

    let mut user5: User = User::build(String::from("snails"), String::from("testing@gmail.com"));

    user5.prettify();
    user5.set_activity(true);
    user5.prettify();
}

// This is an example of using variables to fill in struct fields
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        login_count: 0,
    }
}
