// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Locker {
    locker_number: Option<i32>,
    name: String
}


fn main() {
    let locker = Locker{
        locker_number: None,
        name: "Fabio".to_owned()
    };

    match locker.locker_number {
        Some(ans) => println!("Locker #: {:?}", ans),
        None => println!("No locker provided.")
    }
    match locker.name {
        name => println!("Name: {:?}", name),
    }

}
