use learning_rust::tui_formatting::print_line_string;

pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Common Collections",
        "Vectors mainly",
        main,
        Some(learning_rust::tui_formatting::ModuleFlags::Lesson),
    );
}

fn main() {
    vectors();
    print_line_string(None);
    storing_utf8_with_strings();
    print_line_string(None);
    hashmaps();
}

fn vectors() {
    /*
     * Vec<T>
     *
     * Vectors allow you to store more than one value, but all values must be
     * the same type.
     */

    // Example of vector init
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // Example of vector init
    let mut v = vec![10, 20, 30, 40];

    // updating a vector
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    // Read elements of a vector directly (panics if elements are accessed
    // outside the boundaries of the vector)
    let ev: &i32 = &v[0];
    println!("The first element of vector 'v' is {}", ev);

    // Read elements of a vector indirectly, does not panic if elements are
    // accessed outside the boundaries of the vector
    let second: Option<&i32> = v.get(2);

    match second {
        Some(second) => {
            println!("The second element is {second}");
        }
        None => println!("There is no second element."),
    };

    // accessing all elements in a vector
    for element in &mut v {
        *element += 1; // use *var to update the value in the reference
        println!("{element}");
    }

    //------------------------------------------------
    // using enum to store multiple types in a vector
    //------------------------------------------------
    #[derive(Debug)]
    enum DifferentTypes {
        Int(i32),
        Float(f32),
        String(String),
    }

    let mut v: Vec<DifferentTypes> = vec![DifferentTypes::Int(10)];

    v.push(DifferentTypes::Float(10.0));
    v.push(DifferentTypes::String("Testing".to_string()));

    println!("{:?}", v);
}

fn storing_utf8_with_strings() {
    /***
     * Rust only has one string type in the core lang, &str (string slice).
     * A string slice is a reference to some UTF-8 encoded string data.
     *
     * String is a type provided by the standard library which is
     * mutable, growable, owned, and UTF-8 encoded. This type builds
     * on the Vec type.
     */

    // Example of newly created empty string:

    let empty_string: String = String::new();

    // Example of new string created from a string literal:
    let string: String = String::from("Hello World!");
    println!("{}", string);

    // turn string iteral (&str) to String
    // you can also use the to_string() method on variables containing str types

    let string_lit_to_string: String = "Hello World".to_string();

    // Appending to string
    let mut string: String = "Test".to_string();
    string.push(':'); // this pushes a single char, in '', not "".
    string.push_str("Testing"); // this pushes a full &str.

    // Concatenating Strings
    let string2: String = "World".to_string();

    let string3: String = string + &string2; // $string is moved and now unusable

    // append strings without ownership rules
    let string4: String = format!("{string2}{string3}");
    println!("{}", string4);

    // String indexing
    let index = &string4[0..1]; // returns first character
    println!("{}", index);

    // iterating over strings as chars
    for c in string4.chars() {
        println!("{}", c);
    }

    // iterating over strings as bytes
    for b in string4.bytes() {
        println!("{}", b);
    }
}

fn hashmaps() {
    // This is a complex data type consisting of a list of (key, value) pairs.
    use std::collections::HashMap;

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert("Red Team".to_string(), 10);
    scores.insert("Blue Team".to_string(), 50);

    // accessing a value in a hashmap
    let team: &str = "Red Team";

    // this gets the value for $team (Option<valtype>), copies the value, then
    // unwraps it from Option<T>. If it can't unwrap it, it gives the default
    // value of 0.
    let score: u32 = scores.get(team).copied().unwrap_or(0);
    println!("{}: {}", team, score);

    // example of default value being used
    let team2: &str = "Gold Team";
    let score: u32 = scores.get(team2).copied().unwrap_or(0);
    println!("{}: {}", team2, score);

    // accessing values in a hashmap in a loop without owning scores
    for (team, score) in &scores {
        println!("{team}: {score}");
    }

    /*
     * for types that implement the Copy trait such as i32, the values are
     * copied to the hashmap. For heap values, ownership rules apply
     */

    /*
     * Hashmap keys are UNIQUE
     */

    // overwriting value
    scores.insert("Red Team".to_string(), 20); // updates Red Team

    // insert value if key does not exist
    scores.entry("Red Team".to_string()).or_insert(30);

    // update old value plus new value
    let entry = scores.entry("Red Team".to_string()).or_insert(30);
    *entry += 10; // updates the value of the "Red Team" key in the hash map
}
