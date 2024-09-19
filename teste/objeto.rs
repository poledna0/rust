enum Objetos {
    Cadeira { material: String },
    Computador { polegada: u32, marca: String },
    //Caneta { cor: String },
    //GarrafaAgua { ml: u32, cor: String },
    //Livro { capa_dura: bool, titulo: String },
}

fn mostra_objetos(vetor:Objetos) -> String {
    match vetor {
        Objetos::Cadeira { material } => { format!("É uma cadeira de {}", material)},
        Objetos::Computador { polegada, marca } => { format!("É um computador de {} polegadas da marca {}", polegada, marca)} ,

    }
}



fn main(){
    let mut meu_vetor: Vec<Objetos> = vec! [];
    meu_vetor.push(Objetos::Cadeira{ material: "Madeira".to_string() });
    meu_vetor.push(Objetos::Cadeira{ material: "Ferro".to_string() });
    meu_vetor.push(Objetos::Computador{ polegada: 29, marca: "Pichau".to_string()});

    for item in meu_vetor{
        println!("{}",mostra_objetos(item));
    } 
}
