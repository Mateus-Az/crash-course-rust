enum Color {
    Green,
    Red,
    Blue,
}

fn main() {
    let color = Color::Red;
    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }
}
