//Implemente um código que (1) encontre as duas raízes de uma função quadrática
//(considere números reais de 64 bits) e (2) solicite ao usuário os parâmetros da função. Ao final,
//mostre as raízes. Utilize as equações abaixo para encontrar as raízes (cada letra deverá ser uma
//variável no seu código, inclusive ∆(delta)). Para obter a raiz quadrada, utilize o método
//.sqrt()da variável ∆. Para obter a potenciação, utilize o método .powf()da variável b.

fn main(){
    let mut buffer = String::new();

    println!("Digite o A");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Erro a ler a linha");
    let a:f64 = buffer
        .trim()
        .parse()
        .expect("Erro a converter numero 1");

    buffer.clear();

    println!("Digite o B");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Erro a ler a linha 2");

    let b:f64 = buffer
        .trim()
        .parse()
        .expect("Erro a converter numero 2");
        
    buffer.clear();
    
    println!("Digite o C");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Erro em ler linha 3");
    let c:f64 = buffer
        .trim()
        .parse()
        .expect("Erro a converter numero 3");

    let delta:f64 = b.powf(2.0) - 4.0*a*c;
    println!("delta = {}",delta);

    if delta < 0.0{
        println!("A equação não possui raízes reais");
    }else{
        let raiz_delta: f64 = delta.sqrt();
        let x1:f64 = (-b + raiz_delta) / (2.0 * a);
        let x2:f64 = (-b - raiz_delta) / (2.0 * a);
        println!("As raízes da equação são:");
        println!("x1 = {}",x1);
        println!("x2 = {}",x2);

    }
}
