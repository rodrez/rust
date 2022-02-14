#[derive(Debug)]
enum Employee {
    Manager,
    Regular
}

fn check_emp(input: &str) -> Result<Employee, String> {
    match input {
        "Manager" => Ok(Employee::Manager),
        "Regular" => Ok(Employee::Regular),
        _ => Err("Can't access".to_owned()),
    }
}

fn print_choice(emp: &Employee){
    println!("Employee: {:?}", emp);
}

fn print_access(input: &str) -> Result<(), String> {
    let result: Employee = check_emp(input)?;
    // If the ? returns an error then print choice is never run
    // Which means we would not see the error in the console
    // To do that we need a separate println statement.
    print_choice(&result);
    Ok(())
    
}

fn main() {
    
    let choice = print_access("Regular");
    println!("{:?}", &choice);

}
