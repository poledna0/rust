fn main(){
    let v2:[i32; 4] = [20,20,20,20];
    let media1:f64 = media_vetor(&v2);
    println!("{}",media1);
    
}
fn media_vetor(vetor: &[i32; 4])-> f64{
    let mut soma: f64 = 0.0;
    for valor in vetor{
        soma += *valor as f64;
    }
    soma / vetor.len() as f64
}
