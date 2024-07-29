/*
usado quando temos a possibilidade de erros como fazer uma requisição ao banco de dados ou acessar um recurso remoto

estrutura do result:

enum Result<T,E>{
    ok(T),
    Err(E)
}
*/

struct SoundData {
    name: String,
}

impl SoundData {
    fn new(name: &str) -> Self {
        SoundData {
            name: name.to_owned(),
        }
    }
}

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"))
    } else {
        Err("unable to find sound data".to_owned())
    }
}

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located"),
        Err(e) => println!("error: {:?}", e),
    }
}
