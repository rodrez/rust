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
fn verify_age(age: i32) -> Result<String, String> {
    if age >= 21 {
        Ok("Thanks for your purchase.".to_owned())
    } else {
        // * The Err variant should detail the reason why they cannot make a purchase
        Err("You age is less than 21".to_owned())
    }
}

fn main() {
    let adult_age = Customer { age: 10 };

    let res = verify_age(adult_age.age);

    match res {
        Ok(ans) => println!("{:?}", ans),
        Err(e) => println!("{:?}", e)
    }
}
