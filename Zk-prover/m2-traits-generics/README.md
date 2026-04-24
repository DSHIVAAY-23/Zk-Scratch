# M2 — Traits, Generics & Zero-Cost Abstractions

**Timeline**: Weeks 2–3  
**Roles**: ZK dev · Blockchain dev · Technical writer

> Rust generics are monomorphized — the compiler stamps out concrete copies so there is zero runtime dispatch. This is why generic NTT runs as fast as a hand-written concrete version. Understand this deeply; interviewers always probe it.

---

## 📂 Files in This Module

```
m2-traits-generics/
├── README.md                  ← you are here
├── field_trait.rs             ← Task 1: Field trait + Fp<u64> and Fp<u128> impls
├── generic_ntt.rs             ← Task 2: generic NTT over any type F: Field
├── where_clauses.rs           ← Task 3: where vs inline bounds, side-by-side
├── display_debug.rs           ← Task 4: Display + Debug for FieldElement
├── monomorphization.rs        ← Task 5: explore binary size impact
├── curve_point.rs             ← Task 6: CurvePoint<F: Field>
└── INTERVIEW.md               ← Task 7: monomorphization Q&A
```

---

## ✅ Tasks

### Task 1 — `field_trait.rs`
**Define a `Field` trait and implement it for `Fp<u64>` and `Fp<u128>`**

- Define `trait Field` with associated methods: `add`, `sub`, `mul`, `inv`, `zero`, `one`.
- Create `struct Fp<T>(T)` — a prime field element parameterised by the underlying integer type.
- Implement `Field` for `Fp<u64>` using a fixed prime modulus.
- Implement `Field` for `Fp<u128>` using a larger prime modulus.
- Verify that `Fp<u64>::zero().add(Fp<u64>::one()) == Fp<u64>::one()`.

**What to understand**: Why traits are Rust's answer to interfaces — but without vtable overhead by default. Why you cannot use `trait Field` as an object (`dyn Field`) without extra bounds.

---

### Task 2 — `generic_ntt.rs`
**Write a generic `ntt(coeffs: &mut [F], omega: F)` — no concrete type inside**

- Write `fn ntt<F: Field>(coeffs: &mut [F], omega: F)` using Cooley-Tukey butterfly structure.
- The function body must call only trait methods from `Field` — no `u64` literals inside.
- Test it with both `Fp<u64>` and `Fp<u128>` by calling the same function with different type args.
- Add bounds as needed (`Copy`, `Clone`, etc.) and explain each one in a comment.

**What to understand**: What it means for a function to be generic at the call site. Why `where F: Field + Copy` is different from `where F: Field + Clone`.

---

### Task 3 — `where_clauses.rs`
**Use `where` clauses vs inline bounds — understand when each is clearer**

- Take three functions from Task 2 and rewrite them twice: once with inline bounds, once with `where` clauses.
- Write a comment above each version: *"Prefer inline when ___ because ___. Prefer `where` when ___ because ___."*
- Find a case where inline bounds become unreadable (long type constraints) — show the `where` version is cleaner.

**What to understand**: Both are semantically identical. `where` is not just style — it is sometimes required (e.g., when the bound is on an associated type). Clippy enforces style consistency.

---

### Task 4 — `display_debug.rs`
**Implement `Display` and `Debug` for `FieldElement`**

- Implement `std::fmt::Display` for `FieldElement` so `println!("{}", fe)` prints `"FieldElement(42 mod 97)"`.
- Implement `std::fmt::Debug` — derive it first, then write a manual version that includes the modulus.
- Explain in a comment why blog post code snippets should prefer `Display` over `Debug`.
- Write a test using `format!("{}", fe)` to assert the string output.

**What to understand**: `Display` is for end users; `Debug` is for developers. `{:?}` always uses `Debug`. Blog posts targeting readers expect `Display`.

---

### Task 5 — `monomorphization.rs`
**Explore monomorphization: check binary size before/after adding a type parameter**

- Build a small binary that calls a concrete `ntt_u64(coeffs: &mut [u64], omega: u64)`.
- Record the binary size: `cargo build --release && ls -lh target/release/<binary>`.
- Refactor to generic `ntt<F: Field>` and instantiate with 3 different concrete field types in main.
- Record the new binary size. Observe: each concrete instantiation adds code.
- Write a `// SIZE ANALYSIS:` comment block explaining what you observe.

**What to understand**: Monomorphization is why Rust generics are "zero-cost" at runtime but may increase binary size. This is the tradeoff vs. Java type erasure / Haskell dictionary passing.

---

### Task 6 — `curve_point.rs`
**Write a `CurvePoint<F>` generic over your field — add trait bounds for curve operations**

- Define `struct CurvePoint<F: Field> { x: F, y: F }`.
- Implement point addition on the curve (use simplified Weierstrass: `y² = x³ + ax + b`).
- Required bounds: `F: Field + PartialEq + Copy`.
- Implement the point at infinity as a sentinel (use an `Option<CurvePoint<F>>` or an `is_infinity` flag).
- Write a test: `P + (-P) == point_at_infinity`.

**What to understand**: Generic structs carry their bounds through — every method implementation must repeat or refine them. This is often the source of "bound not satisfied" compiler errors.

---

## 🎙️ Interview Drill — `INTERVIEW.md`

**Question**: *"What is monomorphization and how does it differ from Java generics?"*

**Know cold**: binary size tradeoff, zero runtime overhead, Java type erasure, Haskell dictionary passing.

---

## 📚 Reference Reading

- [The Rust Book — Chapter 10: Generics, Traits, Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [The Rust Reference — Trait Bounds](https://doc.rust-lang.org/reference/trait-bounds.html)
- [Jon Gjengset — Crust of Rust: Generics](https://www.youtube.com/watch?v=BnnqfKM1bDo)
