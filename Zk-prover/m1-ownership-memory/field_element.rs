use std::ops::{Add, Sub, Mul};

const p :u64 = 17;



//Derive `Copy`, `Clone`, `Debug`, `PartialEq`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct FieldElement(u64);


impl FieldElement{
    fn new(a:u64)->Self{
        Self(a%p)
    }
}

impl Add for FieldElement{
    type output self;
    fn add(self,other:self)->Self{
        FieldElement::new((self.0 + other.0) % p)
    }
}

impl Sub for FieldElement{
    type output self;
    fn sub(self,other:self)->Self{
        FieldElement::new((self.0 + other.0) % p)
    }
}

impl Mul for FieldElement{
    type output self;
    fn mul(self,other:self)->Self{
        FieldElement::new((self.0 + other.0) % p)
    }
}



//Write unit tests verifying `(a + b) % p == (b + a) % p` (commutativity).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_commutativity() {
        let a = FieldElement::new(13);
        let b = FieldElement::new(4);

        assert_eq!(a + b, b + a);
    }

    #[test]
    fn test_mul_commutativity() {
        let a = FieldElement::new(5);
        let b = FieldElement::new(3);

        assert_eq!(a * b, b * a);
    }
}