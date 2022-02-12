// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketing,
    Managers,
    LineSups,
    Kitchen,
    Assembly,
}
// Segregated the status 
enum Status {
    Active,
    Inactive,
}

struct Employee {
    position: Position,
    status: Status,
}

// Because there is nothing return, we pass the uni type
fn try_access(employee: &Employee) -> Result<(), String> {

    // We match the status by employee, in this case we only care about the 
    // inactive, because we can ignore the rest.
    match employee.status {
        Status::Inactive => return Err("Terminated".to_owned()),
        _ => ()
    };

    // Similar to the match above. We only care about the ones with access, the rest we ignored them
    match employee.position {
        Position::Managers => Ok(()),
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        _ => Err("Invalid position".to_owned())
    }
}

// For the ? operator to work we need to return either of the results, 
// In the function below we either return the attempt or the Ok at the end 
//  Todo need more practice on Results with ? operator
fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt = try_access(employee)?;
    println!("Access granted");
    Ok(())
}

fn main() {

    // simple variable to hold the employee
    let manager = Employee {
        position: Position::Kitchen,
        status: Status::Active
    };

    // Match to print the acess
    match print_access(&manager) {
        Err(e) => println!("Access denied: {:?}", e),
        _ => ()
    }
}
