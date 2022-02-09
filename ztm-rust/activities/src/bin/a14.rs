// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    color: String
}

fn print_info(name: &str, color: &str ){
    println!("Name: {:?},  favorite color: {:?}", name, color)
}



fn main() {
// * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 5,
            name: "Peter".to_owned(),
            color: String::from("blue")
        },
        Person {
            age: 11,
            name: "Mally".to_owned(),
            color: String::from("red")
        },
        Person {
            age: 9,
            name: "Dan".to_owned(),
            color: String::from("purple")
        }
    ];
    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age < 10{
            print_info(&person.name, &person.color);

        }
    }

}
