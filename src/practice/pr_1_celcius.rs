use learning_rust::tui_formatting::ModuleFlags;

pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Celcius/Fahrenheit",
        "N/A",
        main,
        Some(ModuleFlags::Practice),
    );
}

pub fn main() {
    let fahrenheit = 32.0;
    let celcius = 0.0;

    println!("{fahrenheit}F is {:?}C", fahrenheit_to_celcius(fahrenheit));
    println!("{celcius}C is {:?}F", celcius_to_fahrenheit(celcius));
}

pub fn celcius_to_fahrenheit(temp: f64) -> f64 {
    // (0°C × 9/5) + 32
    (temp * (9.0 / 5.0)) + 32.0
}

pub fn fahrenheit_to_celcius(temp: f64) -> f64 {
    // (32°F − 32) × 5/9
    (temp - 32.0) * (5.0 / 9.0)
}
