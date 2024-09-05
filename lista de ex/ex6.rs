//Implemente um código que (1) receba um número inteiro, não assinado, de 8 bits e (2)
//verifique se é maior de idade ou não. Ao final, mostre na tela o resultado.
fn main(){
    let mut buffer = String::new();
    println!("Digite sua idade: ");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("erro a ler a linha");
    let idade:u8 = buffer
        .trim()
        .parse()
        .expect("erro a converter");
    if idade < 18{
        println!("voce é de menor");
    }else{
        println!("voce é de maior");
    }
}
