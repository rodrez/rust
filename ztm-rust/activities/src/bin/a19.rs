use std::collections::HashMap;
// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

fn main() {
    let inventory = HashMap::from([("chairs", 5), ("beds", 3), ("tables", 2), ("couches", 0)]);

    for (item, count) in inventory {
        match count {
            0 => println!("Out of stock!"),
            _ => println!("Item:{:?}, Count: {:?}", item, count),
        }
    }
}
