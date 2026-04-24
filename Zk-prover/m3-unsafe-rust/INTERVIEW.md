# M3 Interview Practice Sheet

## Question
**"You have unsafe code in a hot path. A colleague says to remove it. How do you justify keeping it?"**

---

## Your Principled Answer Template

```
[Suggested approach — this is an engineering judgment question, not a Rust trivia question]

1. Acknowledge the concern: unsafe code is harder to audit, increases maintenance burden
2. Present your evidence: benchmark numbers showing measurable speedup (e.g., 2x faster NTT)
3. Explain why it is necessary: safe alternative doesn't exist, or introduces unacceptable overhead (e.g., bounds checking in inner loop)
4. Show your safety proof: point to the documented SAFETY comments and invariants
5. Propose validation: Miri test suite passes, fuzz tests run, address sanitizer clean
6. Offer a middle ground: encapsulate unsafe behind a safe public API — only one person needs to reason about the invariants
```

---

## Marker Concepts

- [ ] Benchmarks are not opinions — numbers justify the unsafe block
- [ ] Safety comment is a legal contract — if it is wrong, it is a bug
- [ ] `cargo miri test` runs the test suite under the Miri interpreter (detects UB)
- [ ] Encapsulation: keep `unsafe` internals, expose a `pub fn` that is safe to call
- [ ] Fuzz testing (`cargo fuzz`) + address sanitizer (`ASAN`) as external validation tools
- [ ] The argument is NOT "unsafe is fine because I am careful" — it is "unsafe is necessary AND verified"

---

## Score Yourself

- [ ] Did not concede immediately ("my colleague is probably right")
- [ ] Led with evidence, not opinion
- [ ] Mentioned at least one validation tool (Miri, ASAN, fuzz)
- [ ] Proposed safe encapsulation as a compromise
- [ ] Under 90 seconds
