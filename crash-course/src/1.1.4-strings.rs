/*
String e &str representam textos, porém o &str é um slice de string
&str não são mutáveis
podemos converter um texto liral em uma string usando os métodos: to_string(), String::from(), ou to_owned()
*/

struct Employee {
    name: String,
}

fn main() {
    let emp_name = "Jayson".to_owned();
    let emp_name = String::from("Jayson");
    let emp = Employee { name: emp_name };

    //============================================

    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);
}

fn print_it(data: &str) {
    println!("{:?}", data);
}

//============================================

struct LineItem {
    name: String,
    count: i32,
}
//recebe um slice de string
fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in receipt {
        //recebe um slice de string
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}
