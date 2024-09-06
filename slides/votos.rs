enum Votos {
    Branco,
    Afavor,
    Contra,
}

fn calculadora() {
    let mut vetor: Vec<Votos> = vec![];
    vetor.push(Votos::Branco);
    vetor.push(Votos::Branco);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Branco);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Branco);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Branco);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Contra);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Afavor);
    vetor.push(Votos::Contra);
    funfa(&vetor);
}

fn funfa(vet: &Vec<Votos>) {
    let mut votos_b_vetor: u8 = 0;
    let mut votos_c_vetor: u8 = 0;
    let mut votos_a_vetor: u8 = 0;
    
    for voto in vet {
        match voto {
            Votos::Branco => votos_b_vetor += 1,
            Votos::Afavor => votos_a_vetor += 1,
            Votos::Contra => votos_c_vetor += 1,
        }
    }
    
    println!("Branco: {}, Contra: {}, A Favor: {}", votos_b_vetor, votos_c_vetor, votos_a_vetor);
}

fn main() {
    calculadora();
}
