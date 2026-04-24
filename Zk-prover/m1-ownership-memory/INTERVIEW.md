# M1 Interview Practice Sheet

## Question
**"Why can you have multiple `&T` borrows but only one `&mut T`?"**

---

## Your 90-Second Answer Template

Write your answer in the space below. Time yourself. Aim for under 90 seconds spoken aloud.

```
[Write your answer here. Suggested flow:]

1. State the core rule (1 sentence)
2. Explain WHY this rule exists — aliasing + mutation = undefined behavior (2-3 sentences)
3. Give a concrete memory safety example (1-2 sentences)
4. Mention how Rust enforces this at compile time vs runtime (1 sentence)
5. Optional: mention RefCell<T> as the runtime escape hatch
```

---

## Marker Concepts — All of These Should Appear in Your Answer

- [ ] Shared borrows (`&T`) allow aliasing — multiple readers are safe because nobody can mutate
- [ ] Exclusive borrow (`&mut T`) = no aliasing allowed — this is the "XOR" rule: either shared OR exclusive, never both
- [ ] This prevents data races — the same guarantee that a readers-writer lock gives, but at compile time
- [ ] No runtime overhead — the borrow checker is a static analysis pass, zero cost at runtime
- [ ] `Rc<RefCell<T>>` moves the check to runtime when you genuinely need dynamic aliasing

---

## Common Mistakes to Avoid

- ❌ "Because Rust doesn't have garbage collection" — wrong direction, this is about memory safety not GC
- ❌ "For performance reasons" — secondary benefit, not the primary reason
- ❌ Saying `&mut` is a "reference" without emphasizing it is **exclusive**

---

## Score Yourself

After each attempt, mark:
- [ ] Under 90 seconds
- [ ] Mentioned aliasing specifically
- [ ] Mentioned compile-time guarantee
- [ ] No filler phrases ("um", "basically", "kind of")
