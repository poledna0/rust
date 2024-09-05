//Implemente um código que para verificar se um triângulo é equilátero, isósceles ou
//escaleno. Solicite do usuário os três lados do triângulo.
fn main() {
    let mut l1 = String::new();
    let mut l2 = String::new();
    let mut l3 = String::new();

    //variavel1 as f32 variavel2 as f32, apenas no lugar, n muda a variavel
    // let mut variavel_float = variavel as f32 :)
    
    println!("Digite o primeiro lado do tringulo: ");
    
    std::io::stdin()
        .read_line(&mut l1)
        .expect("ERRO A LER A LINHA!!");
    let l12:u8= l1
        .trim()
        .parse()
        .expect("ERRO A CONVERTER!! O PRIMEIRO NUMERO");
        
    println!("Digite o segundo lado do tringulo");
    std::io::stdin()
        .read_line(&mut l2)
        .expect("ERRO A LER A LINHA!!");
    let l22:u8= l2
        .trim()
        .parse()
        .expect("ERRO A CONVERTER!! O SEGUNDO NUMERO");
        
    println!("Digite o terceiro lado do tringulo");
    std::io::stdin()
        .read_line(&mut l3)
        .expect("ERRO A LER A LINHA!!");
    let l32:u8= l3
        .trim()
        .parse()
        .expect("ERRO A CONVERTER!! O TERCEIRO NUMERO");
        
    if l1 == l2 && l2 == l3 {
        println!("O triangulo e equilatero.");
    } else if l1 == l2 || l1 == l3 || l2 == l3 {
        println!("O triangulo e isosceles.");
    } else {
        println!("O triangulo e escaleno.");
    }
}
