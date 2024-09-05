fn troca(x: &mut i32, y: &mut i32){
    let buffer:i32 = *x;
    *x = *y;
    *y = buffer;
    
}


fn main(){
    let mut a:i32 = 9;
    let mut b:i32 = 11;
    println!("a = {} b = {}",a,b);
    troca(&mut a, &mut b);
    println!("a = {} b = {}",a,b);
    
}
