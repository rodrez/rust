// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    quantity: i32,
    id: i32
}

fn display_qty(item: &Item) {
    println!("Qty: {:?}", item.quantity);
}

fn display_id(item: &Item) {
    println!("ID: {:?}", item.id);
}

fn main() {
    let new_item = Item{
        quantity: 2,
        id:1
    };
    
    display_id(&new_item);
    display_qty(&new_item);

}
