//Implemente um código que (1) solicite dois números inteiros assinados de 32 bits do
//usuário e (2) uma operação matemática: soma, subtração, divisão ou multiplicação. Ao final,
//mostre o resultado de cada operação.
fn main() {
    let mut buffer = String::new();
    let mut entrada1 = String::new();
    let mut entrada2 = String::new();
    let mut resultado:i32 = 0;
    
    println!("Digite [1] para soma");
    println!("Digite [2] para subtração");
    println!("Digite [3] para multiplicação");
    println!("Digite [4] para divisão");
    
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("ERRO A LER A LINHA!!");
    let escolha_de_numero:i8= buffer
        .trim()
        .parse()
        .expect("ERRO A CONVERTER!!");
    println!("Digite o primeiro numero: ");
    std::io::stdin()
        .read_line(&mut entrada1)
        .expect("ERRO A LER A LINHA!!");
    let escolha_de_numero1:i32= entrada1
        .trim()
        .parse()
        .expect("ERRO A CONVERTER!! O PRIMEIRO NUMERO");
    println!("Digite o segundo numero:  ");
    std::io::stdin()
        .read_line(&mut entrada2)
        .expect("ERRO A LER A LINHA!!");
    let escolha_de_numero2:i32= entrada2
        .trim()
        .parse()
        .expect("ERRO A CONVERTER!! O SEGUNDO NUMERO");
    
    match escolha_de_numero{
        1 => resultado = escolha_de_numero1 + escolha_de_numero2,
        2 => resultado = escolha_de_numero1 - escolha_de_numero2,
        3 => resultado = escolha_de_numero1 * escolha_de_numero2,
        4 => resultado = escolha_de_numero1 / escolha_de_numero2,
        _ => panic!("NUMERO INVALIDO"),
    }
    println!("O RESULTADO É: {}",resultado)
}

