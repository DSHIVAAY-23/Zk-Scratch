# M5 Interview Practice Sheet

## Question
**"Why does Montgomery form speed up modular multiplication?"**

---

## Your 90-Second Answer Template

```
[Suggested structure — explain the algorithm, not just the words:]

1. The problem: `(a * b) % p` requires integer division, which is slow
2. The insight: replace division by p with division by R = 2^k (which is just a bit shift)
3. Montgomery form: represent numbers as ã = a * R mod p
4. REDC (Montgomery Reduction): given T = ã * b̃, compute T * R^{-1} mod p using only:
   - multiplication mod R (masking the bottom k bits)
   - addition
   - right shift by k bits
5. No integer division by p at any point — the expensive operation is eliminated
6. Trade-off: you pay a conversion cost (a → Montgomery form) once, then all multiplications in the loop are fast
7. Used in: NTT inner loops, MSM scalar multiplications — anywhere you do millions of modmuls
```

---

## Marker Concepts

- [ ] Integer division (`% p`) is the slow operation being replaced
- [ ] R = 2^k — chosen so division by R is a free bit shift
- [ ] REDC: Montgomery Reduction — know the steps in order
- [ ] `m' = -p^{-1} mod R` — the precomputed inverse
- [ ] Conversion overhead is amortized over many multiplications
- [ ] NTT and MSM are the killer use cases

---

## Score Yourself

- [ ] Mentioned integer division specifically
- [ ] Explained what R is and why it was chosen
- [ ] Walked through REDC at the conceptual level (not just named it)
- [ ] Mentioned amortization of conversion cost
- [ ] Under 90 seconds
