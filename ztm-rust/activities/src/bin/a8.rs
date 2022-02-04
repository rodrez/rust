// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Fruity,
    Sparkling
}

struct Drink {
    name: String,
    oz: i32,
    flavor: Flavors
}

fn print_flavor(){
    let coke = Drink {
        name: String::from("Coke"),
        oz: 16,
        flavor: Flavors::Sparkling
    };

    let mut flv = "";
    match Flavors::Sparkling {
        Flavors::Sparkling => flv = "sparkling",
        Flavors::Fruity => flv = "fruity"
    }

    println!("{}, {}, {}", coke.name, flv, coke.oz);
}

fn main() {

    print_flavor();
}
