use std::io;

// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

#[derive(Debug)]
enum States {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl States {
    fn from_str(s: &str) -> Option<States> {
        match s.trim().to_lowercase().as_str() {
            "off" => Some(States::Off),
            "sleep" => Some(States::Sleep),
            "reboot" => Some(States::Reboot),
            "shutdown" => Some(States::Shutdown),
            "hibernate" => Some(States::Hibernate),
            _ => None,
        }
    }
}

fn print_msg(state: &States) {
    // * Use a function with a match expression to print out the power messages
    // * The function should accept the enum as an input

    use States::*;
    match state {
        Off => println!("Powering off"),
        Sleep => println!("Going to sleep"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    // * Use a match expression to convert the user input into the power state enum
    let mut buffer = String::new();
    let user_input = io::stdin().read_line(&mut buffer);

    if user_input.is_ok() {
        match States::from_str(&buffer) {
            Some(state) => print_msg(&state),
            None => println!("Invalid entry"),
        }
    } else {
        println!("Error reading input")
    }
}
