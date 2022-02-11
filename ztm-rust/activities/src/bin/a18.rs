// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// * Use a struct to store at least the age of a customer
struct Customer {
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
fn verify_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("Customer below age".to_owned())
    } else {
        Ok(())
    }
}

fn main() {

    let new_costumer = Customer {
        age: 20
    };
    let test = verify_purchase(&new_costumer);

    println!("{:?}", test)
    // The return value when using results can be printed out directly from the the assign variable.

}
