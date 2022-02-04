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



// * Use an enum with color names as variants
enum Colors {
    Green,
    Red, 
    Blue
}
// * Use a function to print the color name
fn print_color(color: Colors) {

    match color {
        Colors::Green => println!("It's green!"),
        Colors::Red => println!("It's red!"),
        Colors::Blue => println!("It's blue!")
    }
}
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print



fn main() {
    print_color(Colors::Red)
}
