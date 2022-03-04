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
    let mut inventory: HashMap<&str, i32> = HashMap::new();

    inventory.insert("Chairs", 5);
    inventory.insert("Beds", 3);
    inventory.insert("Tables", 2);
    inventory.insert("Couches", 0);

    for (key, value) in inventory.iter() {
        println!("{:?} {:?}", key, value);
    }

    let mut total = 0;
    for value in inventory.values() {
        total += value;
    }

    println!("{:?}", total);
}
