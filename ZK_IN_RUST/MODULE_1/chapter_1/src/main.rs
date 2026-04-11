
struct  Witness{
    balance: u64,
    secret_key: u64,
    merkle_path: Vec<u64>,

}

struct Statement{
    root: u64,
    threshold: u64,
}


fn verify(witness: &witness, statement: &statement) -> bool{
      // Step 1: balance >= threshold check
    if witness.balance < statement.threshold {
        return false;
    }

    // Step 2: merkle path valid hai (simplified: 
    //         path ka XOR == merkle_root)
    let mut computed_root = 0;
    for &node in &witness.merkle_path {
        computed_root ^= node; // XOR operation
    }
    if computed_root != statement.root {
        return false;
    }
    // path.iter().fold(0, |acc, x| acc ^ x) == merkle_root;

    // Step 3: secret_key != 0 (non-trivial witness)
    if witness.secret_key == 0 {
        return false;
    }

    

    // Teeno true hone chahiye → return true
    true


}

fn main() {
     // Honest prover — valid witness
    let valid_witness = Witness {
        balance: 1<<32 -1, // 2^32 - 1
        secret_key: 12345,
        merkle_path: vec![0xabcde, 0x12345, 0x67890], // example path
    };
    // Cheating prover — invalid witness (balance kam hai)
    let invalid_witness = Witness {
        balance: 100, // threshold se kam
        secret_key: 12345,
        merkle_path: vec![0xabcde, 0x12345, 0x67890], // same path
    };
    // Print karo: "Proof accepted" ya "Proof rejected"
    let statement = Statement {
        root: 0xabcde ^ 0x12345 ^ 0x67890, // valid root for the path
        threshold: 1000, // threshold set karo
    };
    if verify(&valid_witness, &statement) {
        println!("Proof accepted for valid witness");
    } else {
        println!("Proof rejected for valid witness");
    }
  
}
