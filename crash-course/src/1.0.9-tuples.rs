/*
Tipo de record
fácil de desestruturar em uma variável
não é preciso nomear os campos como um enum ou struct
usado para retornar pares de dados em funções
usado quando não temos muitos campos para mais de 3 usar strucs
tuplas podem ter vários tipos de dados
*/

enum Access {
  Full,
}


//retorna uma tupla
fn one_two_three() -> (i32, i32, i32) {
  (1, 2, 3)
}

let numbers = one_two_three();

//desestruturação da tupla
let(x, y, z) = one_two_three();
println!("{:?}, {:?}", x, numbers.0); // 1
println!("{:?}, {:?}", y, numbers.1); // 2
println!("{:?}, {:?}", z, numbers.2); // 3

let(employee, access) = ("Jake", Access::Full);
