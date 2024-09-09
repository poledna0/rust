fn main() {
    let mut buffer = String::new();
    
    println!("Digite segundos para saber a hora exata do mesmo: ");
    
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("erro a ler a linha");
    
    let entrada: u32 = buffer
        .trim()
        .parse()
        .expect("erro a converter");
    
    converte(entrada);
}

fn converte(segundos_entrada:u32){
    let  horas:u32 = segundos_entrada / 3600;
    let  minutos:u32 = (segundos_entrada%3600)/60;
    let  segundos:u32 = segundos_entrada%60;
    
    println!("{} segundos é igual á {}:{}:{}",segundos_entrada,horas,minutos,segundos);
}
