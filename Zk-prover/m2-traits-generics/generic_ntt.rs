// ### Task 2 — `generic_ntt.rs`
// **Write a generic `ntt(coeffs: &mut [F], omega: F)` — no concrete type inside**

// - Write `fn ntt<F: Field>(coeffs: &mut [F], omega: F)` using Cooley-Tukey butterfly structure.
// - The function body must call only trait methods from `Field` — no `u64` literals inside.
// - Test it with both `Fp<u64>` and `Fp<u128>` by calling the same function with different type args.
// - Add bounds as needed (`Copy`, `Clone`, etc.) and explain each one in a comment.

// **What to understand**: What it means for a function to be generic at the call site. Why `where F: Field + Copy` is different from `where F: Field + Clone`.



use crate::field_trait::Field;

fn ntt<F: Field>(coeffs: &mut [F], omega: F) {
    where
    F:Field+Copy,

    {
        let n = coeffs.len();
        let mut len = 1;

        while len < n {
           let mut w_len = F::one();
           for _ in 0..(n / (2 * len)) {
            w_len = w_len.mul(omega);
        }

        for i in (0..n).step_by(2 * len) {
            let mut w = F::one();

            for j in 0..len {
                let u = coeffs[i + j];
                let v = coeffs[i + j + len].mul(w);

                coeffs[i + j] = u.add(v);
                coeffs[i + j + len] = u.sub(v);

                w = w.mul(w_len);
            }
        }

        len *= 2;
    }


}