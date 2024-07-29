enum Discount {
    //define um tipo para o enum
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        //esta armazenando qualquer valor que não foi pego na condicional acima e armazenando na variavel
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        // mesma coisa do other
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        //para qualquer outro valor não faça nada
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    match concert {
        //se o price for 50 e houver um event faça
        Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
        //ele pega só o campo price o restante é ignorado ..
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
