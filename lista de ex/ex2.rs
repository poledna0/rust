//Implemente um código que, através de um laço de repetição (while, loop ou for), encontre
//o maior número (qualquer tipo numérico) de um vetor. Ao final, mostre o valor encontrado.
fn main() {
    let mv = vec![2,5,4,3,5,7,5,3,2,5,7,6,4,3,7,9,72,5];
    let mut mm:u8 = mv[0];
    for n in mv{
        if n > mm{
            mm = n
        } 
    }
    println!("{}",mm)
}
