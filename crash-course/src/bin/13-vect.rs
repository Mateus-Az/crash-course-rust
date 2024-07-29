fn main() {
    let values = vec![10, 20, 30, 40];

    //possibility 1
    // for item in &values {
    //     if *item == 30 {
    //         println!("{:?}", "Thirty");
    //     } else {
    //         println!("{:?}", item);
    //     }
    // }

    // println!("Total itens in values is: {:?}", values.len());
    //
    //=========================================================

    //possibility 2

    for item in &values {
        match item {
            30 => println!("{:?}", "Thirty"),
            _ => println!("{:?}", item),
        }
    }

    println!("Total itens in values is: {:?}", values.len());
    //=========================================================
    //possibility 3
    // println!("Total itens in values is: {:?}", values.len());

    // for item in values {
    //     if item == 30 {
    //         println!("{:?}", "Thirty");
    //     } else {
    //         println!("{:?}", item);
    //     }
    // }
}
