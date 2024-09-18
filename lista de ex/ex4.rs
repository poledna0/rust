//Implemente um código que para verificar se um triângulo é equilátero, isósceles ou
//escaleno. Solicite do usuário os três lados do triângulo.
fn main() {
    let mut lado_um = String::new();
    let mut lado_dois = String::new();
    let mut lado_tres = String::new();
    
    println!("Digite o primeiro lado do tringulo: ");
    
    std::io::stdin()
        .read_line(&mut lado_um)
        .expect("ERRO A LER A LINHA!!");

    println!("Digite o segundo lado do tringulo");
    std::io::stdin()
        .read_line(&mut lado_dois)
        .expect("ERRO A LER A LINHA!!");

    println!("Digite o terceiro lado do tringulo");
    std::io::stdin()
        .read_line(&mut lado_tres)
        .expect("ERRO A LER A LINHA!!");

    if lado_um == lado_dois && lado_dois == lado_tres {
        println!("O triangulo e equilatero.");
    } else if lado_um == lado_dois || lado_um == lado_tres || lado_dois == lado_tres {
        println!("O triangulo e isosceles.");
    } else {
        println!("O triangulo e escaleno.");
    }
}
