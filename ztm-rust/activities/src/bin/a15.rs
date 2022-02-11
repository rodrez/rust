// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


// * Use an enum for the tickets with data associated with each variant
enum Tickets {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32)
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Tickets::Backstage(250, "Peter".to_owned()),
        Tickets::Vip(100, "John".to_owned()),
        Tickets::Standard(50)
    ];
    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Tickets::Backstage(amount, name) => println!("Ticket cost {:?} for {:?}", amount, name),
            Tickets::Vip(amount, name) => println!("Ticket cost {:?} for {:?}", amount, name),
            Tickets::Standard(amount) => println!("Ticket cost {:?} ", amount),
            _ => ()
        }
    }
}
