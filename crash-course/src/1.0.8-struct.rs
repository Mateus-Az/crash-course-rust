//declara os atributos de uma struct, strucs podem ter muitos tipos de dados
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    //é obrigatorio passar um valor para todos os campos declarados no struc ou haverá erro
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };
    //acesso a propriedade se dá pelo . como no java
    let tall = my_box.height;
    println!("The box is {} units tall", tall);
}
