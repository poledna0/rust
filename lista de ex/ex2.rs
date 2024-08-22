fn main() {
    let mv = vec![2,5,4,3,5,7,5,3,2,5,7,6,4,3,7,9,72,5];
    let mut mm:u8 = 0;
    for n in mv{
        if n > mm{
            mm = n
        } 
    }
    println!("{}",mm)
}