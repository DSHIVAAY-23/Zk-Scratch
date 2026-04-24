# M6 Interview Practice Sheet

## Question
**"Walk me through a component you built — explain a hard memory ownership decision."**

---

## Three Strong Answer Options — Pick One and Own It

### Option A — `DeviceBuffer` (unsafe + PhantomData)
```
"I built DeviceBuffer<T>, a wrapper around a raw *mut T pointing to GPU memory.
The hard ownership decision: I had to convince the compiler this type owns a T,
even though the memory is managed by CUDA, not Rust's allocator.
Solution: I added PhantomData<T>. This tells Rust: 'treat this as if it owns a T
for the purpose of drop check and variance analysis.' Without it, Rust would allow
unsound variance — you could coerce a DeviceBuffer<&'short T> into DeviceBuffer<&'long T>.
I also had to manually implement Drop to call cuda_free — Rust's default Drop doesn't
know about GPU allocations. And I wrote unsafe impl Send + Sync with safety comments
justifying why multiple threads can hold a reference to GPU memory safely."
```

### Option B — FFI lifetime bounds (icicle_wrapper)
```
"I wrote a safe wrapper around ICICLE's GPU MSM C function via FFI.
The hard decision: the C function takes raw pointers to scalar and point arrays.
Rust can't verify the pointers are valid for the duration of the GPU computation.
I added lifetime annotations: fn gpu_msm<'a>(scalars: &'a [...], points: &'a [...]).
These constraints mean: 'the input data must live at least as long as this call.'
Without them, the caller could drop the Vec between scheduling the GPU work
and the kernel completing — silent memory corruption. The lifetimes are the compile-time
proof that this cannot happen."
```

### Option C — Bump allocator in prover context
```
"I built a bump allocator for witness generation.
The hard decision: when is it safe to skip individual deallocation?
Answer: only when the lifetime of all allocations is the same — they all die together.
In a ZK prover, we allocate witness data, run NTT and MSM, then throw everything away.
No witness object outlives the proof. So the bump allocator's 'free everything at once'
is sound — and it gives us O(1) allocation with zero fragmentation.
The unsafe part was implementing GlobalAlloc — I had to ensure pointer alignment was
correct for every allocation, and that the backing array outlived every pointer derived from it."
```

---

## Score Yourself

- [ ] Named a specific struct or function from your codebase
- [ ] Identified the exact ownership decision (variance, lifetime bounds, drop, alignment)
- [ ] Explained WHY the decision was hard — what would go wrong without it
- [ ] Explained the solution without reading code
- [ ] Under 90 seconds
