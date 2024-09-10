//Implemente um código que realize a contagem de caracteres em uma variável do tipo
//String. A String deverá ser uma referência. Aqui você deverá transformar a String em um iterador
//utilizando o método .chars() da variável. Mostre a quantidade e o valor da variável.

fn main() {
    let mut entrada = String::new();
    println!("Digite uma frase");
    std::io::stdin()
        .read_line(&mut entrada)
        .expect("erro a ler a linha");
        
    let entrada = entrada.trim();
    
    conta_char(&entrada);
}
fn conta_char(frase:&str){
    let mut contador:u8 = 1;
    for c in frase.chars(){
        println!("caracter = {} contador = {}",c,contador);
        contador += 1;
    }
}
