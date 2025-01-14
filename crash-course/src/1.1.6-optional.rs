/*
um optional pode retornar some ou none
caso seja necessario sair de um méotodo antes de executar todo ele usamos o return

*/

struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "mark@example.com".to_owned(),
    };

    let becky = Customer {
        age: None,
        email: "becky@example.com".to_owned(),
    };

    match becky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided"),
    }
}

struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem {
            name: "bananas".to_owned(),
            qty: 4,
        },
        GroceryItem {
            name: "eggs".to_owned(),
            qty: 12,
        },
        GroceryItem {
            name: "bread".to_owned(),
            qty: 1,
        },
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }

    None
}

fn main() {
    match find_quantity("bananas") {
        Some(qty) => println!("We have {} bananas", qty),
        None => println!("Item not found"),
    }

    match find_quantity("milk") {
        Some(qty) => println!("We have {} milk", qty),
        None => println!("Item not found"),
    }
}
