
// ### Task 5 — `monomorphization.rs`
// **Explore monomorphization: check binary size before/after adding a type parameter**

// - Build a small binary that calls a concrete `ntt_u64(coeffs: &mut [u64], omega: u64)`.
// - Record the binary size: `cargo build --release && ls -lh target/release/<binary>`.
// - Refactor to generic `ntt<F: Field>` and instantiate with 3 different concrete field types in main.
// - Record the new binary size. Observe: each concrete instantiation adds code.
// - Write a `// SIZE ANALYSIS:` comment block explaining what you observe.

// **What to understand**: Monomorphization is why Rust generics are "zero-cost" at runtime but may increase binary size. This is the tradeoff vs. Java type erasure / Haskell dictionary passing.

// ---
use  crate::field_trait::{Field, Fp};


fn ntt<F>(a: &mut [F], omega: F)
where
    F: Field + Copy,
{
    let n = a.len();
    let mut len = 1;

    while len < n {
        let mut w_len = F::one();
        for _ in 0..(n / (2 * len)) {
            w_len = w_len.mul(omega);
        }

        for i in (0..n).step_by(2 * len) {
            let mut w = F::one();

            for j in 0..len {
                let u = a[i + j];
                let v = a[i + j + len].mul(w);

                a[i + j] = u.add(v);
                a[i + j + len] = u.sub(v);

                w = w.mul(w_len);
            }
        }

        len *= 2;
    }
}

fn main() {
    let mut a = [
        Fp::<u64>::one(),
        Fp::<u64>::one(),
        Fp::<u64>::one(),
        Fp::<u64>::one(),
    ];

    let mut b = [
        Fp::<u128>::one(),
        Fp::<u128>::one(),
        Fp::<u128>::one(),
        Fp::<u128>::one(),
    ];

    // two instantiations → two copies in binary
    ntt(&mut a, Fp::<u64>::one());
    ntt(&mut b, Fp::<u128>::one());

    println!("done");
}