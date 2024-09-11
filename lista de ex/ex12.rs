//Implemente um código que realize a contagem de (1) vogais e (2) consoantes de uma
//variável do tipo String. A String deverá ser uma referência. Aqui você deverá transformar a String
//em um iterador utilizando o método .chars() da variável. Mostre os valores encontrados
fn main() {
    let mut entrada = String::new();
    println!("Digite uma palavra: ");
    
    std::io::stdin()
        .read_line(&mut entrada)
        .expect("Erro na entrada");
    
    let entrada = entrada.trim();
    
    conta_letras(&entrada);
}

fn conta_letras(frase:&str){
    let mut n_vogal:u8 = 0;
    let mut n_consoantes:u8 = 0;
    for c in frase.chars(){
        match c.to_ascii_lowercase(){
            'a' | 'e' | 'i' | 'o' | 'u' => n_vogal += 1,
            'a'..='z' => n_consoantes += 1,
            _ => {}
        }
    }
    println!("A palavra {} possui {} vogais e {} consoantes.", frase, n_vogal,n_consoantes)
    
}
