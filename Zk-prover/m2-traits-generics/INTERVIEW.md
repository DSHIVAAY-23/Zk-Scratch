# M2 Interview Practice Sheet

## Question
**"What is monomorphization and how does it differ from Java generics?"**

---

## Your 90-Second Answer Template

```
[Suggested structure:]

1. Define monomorphization in one sentence
2. What happens at compile time: compiler stamps out a concrete function per type
3. Runtime cost: zero — no boxing, no vtable lookup, no type tag
4. Java comparison: generics are erased at compile time → List<Integer> becomes List<Object> at runtime → requires boxing for primitives
5. Tradeoff: Rust: bigger binary, faster runtime. Java: smaller binary, runtime overhead for primitives
6. Bonus: mention Haskell's dictionary-passing approach as a third alternative
```

---

## Marker Concepts

- [ ] "Monomorphization" — the compiler generates one concrete copy per type argument
- [ ] No runtime overhead (no vtable, no boxing in the generic case)
- [ ] Binary size grows with each unique instantiation
- [ ] Java type erasure — generics disappear at runtime
- [ ] `dyn Trait` in Rust = runtime dispatch (closest Rust gets to Java generics)
- [ ] `impl Trait` = static dispatch = monomorphized by the compiler

---

## Score Yourself

- [ ] Under 90 seconds
- [ ] Used the word "monomorphize"
- [ ] Correctly described Java type erasure
- [ ] Mentioned the binary size tradeoff
