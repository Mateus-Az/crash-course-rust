enum Menu {
    Burger,
    Fries,
    Drink,
}

fn main() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 { true } else { false };

    // Forma simplificada
    let is_lt_5 = my_num < 5;

    //utilizando em conjunto com match
    let message = match my_num {
        1 => "hello",
        _ => "goodbye",
    };

    //é possivel aninhar expressões multiplas vezes, aumenta complexidade rs
    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };
}
