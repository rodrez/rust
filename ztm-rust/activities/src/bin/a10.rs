// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


fn print_result(val: bool) {
    match val {
        true => println!("It's big"),
        false => println!("It's small")
    }
}

fn main() {
    let value = 110;
    let result = value > 100;

    print_result(result)
}
