

const p :u64 = 17;

fn add(a:u64,b:u64)->u64{
    (a+b)%p
}
fn mul(a:u64,b:u64)->u64{
    (a*b)%p
}
fn dot_product(a: &[u64], b: &[u64]) -> u64 {
    a.iter()
     .zip(b.iter())
     .map(|(x, y)| mul(*x, *y))
     .fold(0, |acc, val| add(acc, val))
}
fn main() {



 let witness: Vec<u64> = vec![1, 3, 4, 12];
     let l_row: [u64; 4] = [0, 1, 0, 0];
     
 
  println!("add(13,4)      = {}", add(13, 4));
    println!("mul(13,4)      = {}", mul(13, 4));
    println!("dot_product   = {}", dot_product(&l_row, &witness));
  
}