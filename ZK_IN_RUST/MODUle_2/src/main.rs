struct FiniteField {
    a: u64, // element in the field
    b: u64, // another element in the field
    p: u64, // prime modulus
}

impl FiniteField {
    fn new(a: u64, b: u64, p: u64) -> Self {
        Self { a, b, p }
    }

    fn add(&self) -> u64 {
        (self.a + self.b) % self.p
    }

    fn mul(&self) -> u64 {
        (self.a * self.b) % self.p
    }
    fn additive_inverse(&self) -> u64 {
        (self.p - self.a) % self.p
    }
    fn multiplicative_inverse(&self) -> Option<u64> {
        for x in 1..self.p {
            if (self.a * x) % self.p == 1 {
                return Some(x);
            }
        }
        None // No inverse exists if a is 0
    }

}
fn main() {

    let field = FiniteField::new(5, 9, 17);
    let sum = field.add();
    let product = field.mul();
    let additive_inverse = field.additive_inverse();
    let multiplicative_inverse = field.multiplicative_inverse();

    print!("Additive Inverse: {}\n", additive_inverse); // Output: Additive Inverse: 12
    match multiplicative_inverse {
        Some(inv) => println!("Multiplicative Inverse: {}", inv), // Output: Multiplicative Inverse: 7
        None => println!("No multiplicative inverse exists"),
    }

    println!("Sum: {}", sum); // Output: Sum: 1
    println!("Product: {}", product); // Output: Product: 1
    println!("Hello, world!");
}
