struct Grocery {
    id: i32,
    quantity: i32,
}

fn main() {
    let grocery = Grocery {
        id: 1,
        quantity: 12,
    };

    show_id(&grocery);
    show_quantity(grocery);
}

fn show_id(grocery: &Grocery) {
    println!("id {:?}", grocery.id);
}

fn show_quantity(grocery: Grocery) {
    println!("quantity {:?}", grocery.quantity);
}
