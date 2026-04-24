// ### Task 4 — `display_debug.rs`
// **Implement `Display` and `Debug` for `FieldElement`**

// - Implement `std::fmt::Display` for `FieldElement` so `println!("{}", fe)` prints `"FieldElement(42 mod 97)"`.
// - Implement `std::fmt::Debug` — derive it first, then write a manual version that includes the modulus.
// - Explain in a comment why blog post code snippets should prefer `Display` over `Debug`.
// - Write a test using `format!("{}", fe)` to assert the string output.

// **What to understand**: `Display` is for end users; `Debug` is for developers. `{:?}` always uses `Debug`. Blog posts targeting readers expect `Display`.



use std::fmt;
use crate::field_trait::{Fp};

const P64: u64 = 17;
const P128: u128 = (1u128 << 127) - 1;




impl fmt::Display for Fp<u64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement({} mod {})", self.0, P64)
    }
}

impl fmt::Display for Fp<u128> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement({} mod {})", self.0, P128)
    }
}

iimpl fmt::Debug for Fp<u64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fp {{ value: {}, modulus: {} }}", self.0, P64)
    }
}

impl fmt::Debug for Fp<u128> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fp {{ value: {}, modulus: {} }}", self.0, P128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_trait::Fp;

    #[test]
    fn test_display() {
        let fe = Fp::<u64>(42);
        assert_eq!(format!("{}", fe), "FieldElement(42 mod 17)");
    }

    #[test]
    fn test_debug() {
        let fe = Fp::<u64>(42);
        assert_eq!(
            format!("{:?}", fe),
            "Fp { value: 42, modulus: 17 }"
        );
    }
}


// Display is preferred in blog posts because it produces clean,
// human-readable output, while Debug is intended for developers
// and includes more structural/internal details.