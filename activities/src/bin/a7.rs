// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}
fn print_color(eye_color: Color) {
    match eye_color {
        Color::Red => println!("red"),
        Color::Green => println!("green"), 
        Color::Blue => println!("blue"),     
    }
}

enum Fart {
    Squeak,
    Toot,
    Powerstink
}

fn print_fart(sound: Fart) {
    match sound {
        Fart::Squeak => println!("squeak"),
        Fart::Toot => println!("toooot a hooooting"),
        Fart::Powerstink => println!("baaaaffffff"),
    }
}

fn main () {
    print_fart(Fart::Toot);
    print_fart(Fart::Powerstink);
    print_color(Color::Red);
    print_color(Color::Blue);
    print_fart(Fart::Squeak);
    print_color(Color::Green);
}