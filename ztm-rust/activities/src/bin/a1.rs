// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal
fn first_name(name: String) -> String {
    return name;
}

fn last_name(name: String) -> String {
    return name;
}

fn main() {

    let f = first_name("Fabian".to_string());
    let l = last_name("Rodrez".to_string());

    println!("{:?}", f);
    println!("{:?}", l)
}
