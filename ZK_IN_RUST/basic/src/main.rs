
use std::collections::HashMap;

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


fn build_witness() -> HashMap<String, u64> {
    // insert: "one"->1, "a"->3, "b"->4, "c"->12
    // return karo
    let mut witness = HashMap::new();
    witness.insert("one".to_string(), 1);
    witness.insert("a".to_string(), 3);
    witness.insert("b".to_string(), 4);
    witness.insert("c".to_string(), 12);
    witness
    
}

fn factorial_mod(n: u64) -> u64 {
    // base case: n==0 → 1
    // recursive: n * factorial_mod(n-1) % P
    if n == 0 {
        return 1;
    }
    (n * factorial_mod(n - 1)) % p
}


fn main() {
   let w = build_witness();
    println!("a = {}", w["a"]);  // should print 3
    println!("5! mod 17 = {}", factorial_mod(5)); 


 let witness: Vec<u64> = vec![1, 3, 4, 12];
     let l_row: [u64; 4] = [0, 1, 0, 0];
     
 
  println!("add(13,4)      = {}", add(13, 4));
    println!("mul(13,4)      = {}", mul(13, 4));
    println!("dot_product   = {}", dot_product(&l_row, &witness));
  
}