# M1 — Rust Ownership & Memory: The Foundation

**Timeline**: Weeks 1–2  
**Roles**: ZK dev · Blockchain dev · Technical writer

> This module is your Rust foundation. Every ZK prover bug you will ever hit traces back to ownership, lifetimes, or borrow rules. Master this before touching crypto math.

---

## 📂 Files in This Module

```
m1-ownership-memory/
├── README.md                  ← you are here
├── field_element.rs           ← Task 1: stack-allocated FieldElement
├── heap_buffer.rs             ← Task 2: HeapBuffer wrapping Box<[T]>
├── polynomial_vec.rs          ← Task 3: PolynomialVec newtype over Vec
├── slices_and_lifetimes.rs    ← Task 4: function returning &[u64] with lifetimes
├── from_iterator.rs           ← Task 5: FromIterator for polynomial type
├── borrow_errors.rs           ← Task 6: 3 failing borrow-checker tests + fixes
└── INTERVIEW.md               ← Task 7: Interview Q&A practice sheet
```

---

## ✅ Tasks

### Task 1 — `field_element.rs`
**Implement a stack-allocated `FieldElement`**

- Define a struct `FieldElement(u64)` that lives entirely on the stack (no heap allocation).
- Implement the `Add`, `Sub`, and `Mul` traits from `std::ops`.
- All arithmetic must be modular — pick a prime modulus (e.g., a small Mersenne prime) and store it or hard-code it.
- Derive `Copy`, `Clone`, `Debug`, `PartialEq`.
- Write unit tests verifying `(a + b) % p == (b + a) % p` (commutativity).

**What to understand**: Why `Copy` is derivable here. Why you don't need `Drop`. Stack vs heap tradeoff for small fixed-size math types.

---

### Task 2 — `heap_buffer.rs`
**Write a `HeapBuffer<T>` that wraps `Box<[T]>`**

- Define `struct HeapBuffer<T> { inner: Box<[T]> }`.
- Implement `Drop` manually — add a `println!` or counter to confirm it fires exactly once.
- Implement `Deref` targeting `[T]` so you can call slice methods on `HeapBuffer`.
- Implement `DerefMut` so callers can mutate elements through it.
- Write a test that creates a `HeapBuffer`, mutates an element through `DerefMut`, drops it, and asserts the drop ran.

**What to understand**: When `Box<[T]>` is better than `Vec<T>`. The difference between `Deref` coercions and explicit method calls. Why `Drop` is called automatically.

---

### Task 3 — `polynomial_vec.rs`
**Build `PolynomialVec` — a newtype over `Vec<u64>`**

- Define `struct PolynomialVec(Vec<u64>)`.
- Implement `Deref` and `DerefMut` targeting `Vec<u64>`.
- Write a `fn scale(&mut self, factor: u64)` that multiplies every coefficient by a scalar.
- Write a comment above every `.clone()` call explaining: *"I need to clone here because ___"*.
- Add a method `fn add_poly(&self, other: &PolynomialVec) -> PolynomialVec` — reason about whether you need `Clone` on the output.

**What to understand**: When `clone` is the right answer vs. when it signals a design smell. Newtype pattern advantages (control over trait impls, clarity of intent).

---

### Task 4 — `slices_and_lifetimes.rs`
**Write a function returning `&[u64]` and annotate every lifetime out loud**

- Write `fn first_half<'a>(data: &'a [u64]) -> &'a [u64]` — explain why `'a` appears on both sides.
- Write a struct `Coefficients<'a> { data: &'a [u64] }` and a method on it that returns another `&'a [u64]`.
- Write a second function with two input slices of different lifetimes — fail to compile it, then fix it.
- Leave a `// EXPLAIN:` comment next to every lifetime annotation describing what it constrains.

**What to understand**: Lifetime elision rules (when can you omit `'a`?). The NLL (Non-Lexical Lifetimes) mental model. Why the compiler needs lifetime annotations at all — it is not the compiler being pedantic, it is you communicating intent.

---

### Task 5 — `from_iterator.rs`
**Implement `FromIterator` for your polynomial type**

- Implement `FromIterator<u64>` for `PolynomialVec` so that `iter.collect::<PolynomialVec>()` works.
- Use it in a test: create a `PolynomialVec` from `(0u64..8).map(|i| i * 3)`.
- Also implement `IntoIterator` for `&PolynomialVec` so you can use it in a `for` loop.
- Bonus: implement `Extend<u64>` for `PolynomialVec`.

**What to understand**: The `Iterator` adapter chain. How `collect()` dispatches via `FromIterator`. The difference between `into_iter()`, `iter()`, and `iter_mut()`.

---

### Task 6 — `borrow_errors.rs`
**Write 3 failing tests that demonstrate borrow checker errors, then fix each**

Write three separate test functions, each of which **initially fails to compile**:

1. **Use-after-move**: move a value into a function, then try to use the original binding.
2. **Simultaneous mutable + immutable borrow**: hold a `&PolynomialVec` and a `&mut PolynomialVec` at the same time.
3. **Lifetime too short**: return a reference to a local variable from a function.

For each:
- Leave the broken code commented out with `// BROKEN:` prefix.
- Below it write the fixed version with a `// FIX:` comment explaining the rule that was violated.

**What to understand**: The three rules of ownership. How to read compiler error messages (E0505, E0502, E0106). The borrow checker is a static analysis pass, not a runtime check.

---

## 🎙️ Interview Drill — `INTERVIEW.md`

**Question**: *"Why can you have multiple `&T` borrows but only one `&mut T`?"*

**Your goal**: Answer in ≤ 90 seconds without looking at code or notes.

**Hints to build your answer around**:
- Shared borrows guarantee the value will not change during the borrow — so aliasing is safe.
- A mutable borrow is an exclusive lock — no other reader or writer can exist simultaneously.
- This eliminates data races **at compile time**, not at runtime.
- Relate to the "readers-writer lock" mental model.
- Mention: this is why `Rc<RefCell<T>>` exists — to move the check to runtime when you genuinely need it.

See `INTERVIEW.md` for a full template to write and rehearse your answer.

---

## 📚 Reference Reading

- [The Rust Book — Chapter 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Rust Book — Chapter 10.3: Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rustonomicon — Chapter 3: Ownership](https://doc.rust-lang.org/nomicon/ownership.html)
