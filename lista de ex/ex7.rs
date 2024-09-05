//Implemente um código que calcule a potência de um número. Solicite do usuário a base e
//o expoente. Ao final, mostre o resultado.
fn main(){
    let mut buffer = String::new();
    println!("digite uma base");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Erro a ler a linha");
    let base:u16 = buffer
        .trim()
        .parse()
        .expect("erro a converter");
    buffer.clear();
    println!("digite um expoente");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("erro a lera a linha2");
    let expoente:u16 = buffer
        .trim()
        .parse()
        .expect("erro a converter numero2");
        
    let mut resultado:u16 = 1;
    
    if expoente == 0{
    resultado = 1
    
    }else{
        for _ in 0..expoente {
            resultado *= base;
        }
    }

    
    println!("o resultado de {} elevado à {} é: {}",base, expoente, resultado);
}
