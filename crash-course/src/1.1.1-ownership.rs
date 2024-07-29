/*
caso a propriedade passada para um método não tiver o atributo & significa que esse méotodo pode destruir a referencia;
caso ele tenha & somente onde a variavel foi declarada pode destruir, a referência é emprestada/ barrow

*/

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}
