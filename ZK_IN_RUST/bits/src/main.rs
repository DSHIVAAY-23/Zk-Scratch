


// Ek function likho jo check kare:
// balance >= threshold

fn is_valid_witness(balance: u64, threshold: u64) -> bool{
    balance >= threshold
}



// Balance ko 8-bit binary mein decompose karo
// (ZK circuits mein yahi hota hai Chapter 2 mein)

fn to_bits(n: u8) -> [u8; 8]
{
    let mut bits = [0; 8];
    for i in 0..8 {
        bits[i] = (n >> i) & 1;
    }
    bits
}

// to_bits(13) → [1, 0, 1, 1, 0, 0, 0, 0]  (LSB first)
// to_bits(0)  → [0, 0, 0, 0, 0, 0, 0, 0]


// ZK mein har bit ke liye constraint hota hai:
//   bit * (bit - 1) == 0

// Ek function likho jo array of bits leke
// check kare ki sab valid bits hain (0 ya 1 only)

fn check_bit_constraints(bits: &[i64]) -> bool{
    for &bit in bits {
        if bit * (bit - 1) != 0 {
            return false;
        }
    }
    true
}





// check_bit_constraints(&[1,0,1,0]) → true
// check_bit_constraints(&[1,2,0,1]) → false  (2 invalid hai)



// BN128 curve ka prime p use karo (simplified version mein
// sirf 17 use karo as small prime for practice)

fn add_mod(a: u64, b: u64, p: u64) -> u64{
    (a + b) % p
}
fn mul_mod(a: u64, b: u64, p: u64) -> u64{
    (a * b) % p
}

// add_mod(15, 9, 17) → 7   (15+9=24, 24 mod 17 = 7)
// mul_mod(5, 7, 17)  → 1   (35 mod 17 = 1)
// mul_mod(3, 6, 17)  → 1   (18 mod 17 = 1)



fn main() {


    add_mod(15,9,17);
    mul_mod(5,7,17);
    mul_mod(3,6,17);

    print!("{} ", add_mod(15,9,17));
    print!("{} ", mul_mod(5,7,17));
    println!("{} ", mul_mod(3,6,17));

    check_bit_constraints(&[1,0,1,0]);
    check_bit_constraints(&[1,2,0,1]);


    to_bits(13);
    to_bits(0);
    println!("{:?}", to_bits(13));
    println!("{:?}", to_bits(0));
    println!("Hello, world!");
    
    let balance = 1<<32 -1;
    println!("{}", balance);
    println!("{}", std::i32::MAX);
    println!("Hello, world!");
}
