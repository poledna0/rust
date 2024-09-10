fn main() {
    let mut buffer = String::new();
    println!("Digite um numero para saber se é par ou impar");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("erro a ler a linha");
    let entrada:u8 = buffer
        .trim()
        .parse()
        .expect("erro em converter");
    
    if entrada % 2 == 0{
        println!("O numero {} é par",entrada);
    }else{
        println!("O numero {} é impar",entrada);
    }
    
}
