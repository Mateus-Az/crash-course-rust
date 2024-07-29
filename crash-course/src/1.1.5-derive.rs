/*
as anotações derive servem para implementar trats automaticas como debug, clone e copy
a traits de debug permite usar ela para imprimir diretamente usando macro
copy usado para fazer copias simples e rapidas, bit a bit
clone copia profunda, uso explicito, usado para dados complexos
*/

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    //veja aqui não é preciso usar o & para copia
    print_employee(me);
    print_employee(me);
}
