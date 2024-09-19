// Definindo a struct
struct Pessoa {
    nome: String,
    idade: u32,
}

// Implementando métodos para Pessoa
impl Pessoa {
    // Método que cria uma nova pessoa
    fn new(nome: &str, idade: u32) -> Pessoa {
        Pessoa {
            nome: String::from(nome),
            idade,
        }
    }

    // Método que exibe uma mensagem
    fn apresentar(&self) {
        println!("Olá, meu nome é {} e eu tenho {} anos.", self.nome, self.idade);
    }
}

fn main() {
    // Criando uma instância de Pessoa usando o método new
    let pessoa = Pessoa::new("Maria", 25);

    // Chamando o método apresentar
    pessoa.apresentar();
}
