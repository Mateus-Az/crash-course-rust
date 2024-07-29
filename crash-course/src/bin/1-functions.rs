/*
 *Para rodar o programa executamos no terminal o comando ( cargo run --bin {nome do arquivo sem extensão }) / flag -q remove as mensagens de execução
 *O ponto de entrada da aplicação começa no fn main como se fosse o application do spring.
 *
*/
fn main() {
    name();
    last_name();

    print_macro();
}

fn name() {
    print!("Mateus");
}

fn last_name() {
    print!(" Azevedo \n");
}

fn print_macro() {
    let full_name = "Mateus Azevedo";
    print!("{:?}", full_name);
}
