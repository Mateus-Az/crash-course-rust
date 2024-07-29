// Define a struct Temperature que contém um campo degrees_f do tipo f64
struct Temperature {
    degrees_f: f64,
}

// Implementa a struct Temperature com métodos associados
impl Temperature {
    // Método estático que cria uma nova instância de Temperature com a temperatura de congelamento (32.0°F)
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    // Método estático que cria uma nova instância de Temperature com a temperatura de ebulição (212.0°F)
    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }

    // Método de instância que exibe a temperatura armazenada no campo degrees_f
    fn show_temp(&self) {
        println!("{} degrees F", self.degrees_f);
    }
}

fn main() {
    // Cria uma instância de Temperature representando uma temperatura quente (99.9°F)
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp(); // Exibe "99.9 degrees F"

    // Cria uma instância de Temperature com a temperatura de congelamento usando o método estático freezing
    let mut cold = Temperature::freezing();
    cold.show_temp(); // Exibe "32.0 degrees F"

    cold.show_temp(); // Exibe "212.0 degrees F"

    // Cria uma instância de Temperature com a temperatura de ebulição usando o método estático boiling
    let boiling = Temperature::boiling();
    boiling.show_temp(); // Exibe "212.0 degrees F"
}
