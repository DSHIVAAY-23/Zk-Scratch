// ### Task 1 — `field_trait.rs`
// **Define a `Field` trait and implement it for `Fp<u64>` and `Fp<u128>`**

// - Define `trait Field` with associated methods: `add`, `sub`, `mul`, `inv`, `zero`, `one`.
// - Create `struct Fp<T>(T)` — a prime field element parameterised by the underlying integer type.
// - Implement `Field` for `Fp<u64>` using a fixed prime modulus.
// - Implement `Field` for `Fp<u128>` using a larger prime modulus.
// - Verify that `Fp<u64>::zero().add(Fp<u64>::one()) == Fp<u64>::one()`.

// **What to understand**: Why traits are Rust's answer to interfaces — but without vtable overhead by default. Why you cannot use `trait Field` as an object (`dyn Field`) without extra bounds.




use std::ops;

const P64: u64 = 17;

const P128: u128 = (1u128 << 127) - 1;

pub trait Field: Sized + Copy {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn inv(self) -> Self;

    fn zero() -> Self;
    fn one() -> Self;
}

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fp<T>(T);

impl Field for Fp<u64> {
    fn add(self, other: Self) -> Self {
        Fp((self.0 + other.0) % P64)
    }

    fn sub(self, other: Self) -> Self {
        Fp((self.0 + P64 - other.0) % P64)
    }

    fn mul(self, other: Self) -> Self {
        Fp((self.0 * other.0) % P64)
    }

    fn inv(self) -> Self {
        // Fermat’s little theorem: a^(p-2)
        Fp(mod_pow(self.0, P64 - 2, P64))
    }

    fn zero() -> Self {
        Fp(0)
    }

    fn one() -> Self {
        Fp(1)
    }
}
fn mod_pow_u128(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }

    result
}

impl Field for Fp<u128> {
    fn add(self, other: Self) -> Self {
        Fp((self.0 + other.0) % P128)
    }

    fn sub(self, other: Self) -> Self {
        Fp((self.0 + P128 - other.0) % P128)
    }

    fn mul(self, other: Self) -> Self {
        Fp((self.0 * other.0) % P128)
    }

    fn inv(self) -> Self {
        Fp(mod_pow_u128(self.0, P128 - 2, P128))
    }

    fn zero() -> Self {
        Fp(0)
    }

    fn one() -> Self {
        Fp(1)
    }
}


#[test]
fn test_field_basic() {
    let zero = Fp::<u64>::zero();
    let one = Fp::<u64>::one();

    assert_eq!(zero.add(one), one);
}