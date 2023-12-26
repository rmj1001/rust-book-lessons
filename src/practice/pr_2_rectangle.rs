use learning_rust::tui_formatting::ModuleFlags;

pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Rectangle",
        "N/A",
        main,
        Some(ModuleFlags::Practice),
    );
}

#[derive(Debug)] // <- This allows for formatting structs in the console
struct Rectangle {
    width: u64,
    height: u64,
}

// This allows for defining methods on structs
impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn perimeter(&self) -> u64 {
        (self.width * 2) + (self.height * 2)
    }

    fn can_fit_rectangle(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn main() {
    let rectangle = Rectangle {
        width: 5,
        height: 9,
    };

    let area: u64 = calculate_area_of_rectangle(&rectangle);

    println!("The area of rectangle '{:#?}' is '{}'.", rectangle, area);

    let rec2 = Rectangle {
        width: 10,
        height: 10,
    };

    println!(
        "The area of rectangle '{:#?}' is {}. The perimeter is '{:#?}'.",
        rec2,
        rec2.area(),
        rec2.perimeter()
    );

    println!(
        "Does the first rectangle fit inside the second? {} ",
        rec2.can_fit_rectangle(&rectangle)
    );
}

fn calculate_area_of_rectangle(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.height
}
