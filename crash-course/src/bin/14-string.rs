struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

impl Person {
    fn print_name(&self) {
        print!("{:?}", self.name);
    }

    fn print_color(&self) {
        println!(" {:?}", self.favorite_color);
    }
}

fn main() {
    let persons = vec![
        Person {
            age: 10,
            name: "Matt".to_owned(),
            favorite_color: "Blue".to_owned(),
        },
        Person {
            age: 5,
            name: "Erica".to_owned(),
            favorite_color: "Blue".to_owned(),
        },
        Person {
            age: 20,
            name: "Mila".to_owned(),
            favorite_color: "Blue".to_owned(),
        },
    ];

    for pers in persons {
        if pers.age >= 10 {
            pers.print_name();
            pers.print_color();
        }
    }
}
