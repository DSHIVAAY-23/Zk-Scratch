# M3 — Unsafe Rust & Raw Memory: The Interview Gauntlet

**Timeline**: Weeks 3–4  
**Roles**: ZK dev · Blockchain dev

> Unsafe Rust is not a zone where rules don't apply — it is a zone where YOU are responsible for upholding the rules the compiler can no longer verify. Every unsafe block you write must have a safety proof. This module teaches you to write that proof, not just the code.

---

## 📂 Files in This Module

```
m3-unsafe-rust/
├── README.md                  ← you are here
├── device_buffer.rs           ← Task 1: DeviceBuffer<T> with raw pointer + PhantomData
├── send_sync.rs               ← Task 2: impl Send + Sync for DeviceBuffer with safety comment
├── aligned_allocator.rs       ← Task 3: custom GlobalAlloc for 64-byte aligned fields
├── ptr_ops.rs                 ← Task 4: ptr::read / ptr::write with drop counter test
├── bump_allocator.rs          ← Task 5: bump allocator + safety justification for prover use
├── arkworks_annotation.rs     ← Task 6: 20 lines of annotated arkworks unsafe code
└── INTERVIEW.md               ← Task 7: unsafe justification Q&A
```

---

## ✅ Tasks

### Task 1 — `device_buffer.rs`
**Build `DeviceBuffer<T>`: raw `*mut T`, `PhantomData`, `Drop` calling `cuda_free` stub**

- Define `struct DeviceBuffer<T> { ptr: *mut T, len: usize, _marker: PhantomData<T> }`.
- Write a stub `fn cuda_alloc(bytes: usize) -> *mut u8` and `fn cuda_free(ptr: *mut u8)`.
- Write a constructor `fn new(len: usize) -> Self` that calls `cuda_alloc`.
- Implement `Drop` that calls `cuda_free` on the raw pointer.
- Write a `// SAFETY:` comment explaining why the Drop impl is sound.
- Use `PhantomData<T>` — write a comment explaining what variance / ownership signal it sends to the compiler.

**What to understand**: `PhantomData` tells the compiler "this type logically owns a `T`" so it participates in drop check and variance analysis. Without it, `DeviceBuffer` would have undefined variance over `T`.

---

### Task 2 — `send_sync.rs`
**Implement `Send + Sync` for `DeviceBuffer<T>` — write the safety comment justifying each**

- Write `unsafe impl<T: Send> Send for DeviceBuffer<T>`.
- Write `unsafe impl<T: Send + Sync> Sync for DeviceBuffer<T>`.
- Before each impl, write a multi-line `// SAFETY:` comment that explains:
  - What invariant you are asserting
  - Why it is true given your usage (single owner, CUDA handles are not thread-local, etc.)
  - What would break if this were wrong

**What to understand**: `Send` and `Sync` are marker traits with no methods — they are promises to the compiler about thread safety. Making them `unsafe impl` means you own the proof obligation. A wrong safety comment is a bug, not just bad style.

---

### Task 3 — `aligned_allocator.rs`
**Write a custom allocator using `std::alloc::GlobalAlloc` for 64-byte aligned fields**

- Implement a struct `AlignedAllocator` and implement `GlobalAlloc` for it.
- `alloc` must return memory aligned to 64 bytes (for SIMD / cache line alignment).
- `dealloc` must correctly free the memory without leaking.
- Write a test that allocates a `[u64; 8]` through your allocator and checks the pointer is 64-byte aligned (`ptr as usize % 64 == 0`).
- Write a `// SAFETY:` comment on every `unsafe` block.

**What to understand**: SIMD operations (AVX-512) require 64-byte alignment. NTT butterfly operations benefit from aligned memory for auto-vectorisation. This is why arkworks uses custom allocators.

---

### Task 4 — `ptr_ops.rs`
**Use `ptr::read` / `ptr::write` to move values without calling drop — test with a drop counter**

- Create a `DropCounter` struct that holds an `Arc<AtomicUsize>` and increments it in `Drop::drop`.
- Allocate two `DropCounter` instances on the heap using `Box::into_raw`.
- Use `ptr::write` to overwrite one with the other **without** calling `drop` on the overwritten value.
- Use `ptr::read` to copy a value out **without** moving ownership.
- Assert the drop counter was called the correct number of times at end of test.
- Write `// SAFETY:` on every raw pointer operation.

**What to understand**: `ptr::write` is `memcpy` — it does not call `drop` on the destination. Used in data structures that manage memory manually (e.g., custom `Vec`). Getting this wrong → double-free or leaked memory.

---

### Task 5 — `bump_allocator.rs`
**Implement a bump allocator — explain why it is safe in a prover context**

- Implement `BumpAllocator` over a fixed `[u8; N]` backing array.
- `fn alloc(&mut self, size: usize, align: usize) -> *mut u8` — advance a cursor by `size` aligned to `align`.
- If the backing array is exhausted, return a null pointer.
- No `dealloc` — all memory is freed at once when `BumpAllocator` is dropped.
- In a `// PROVER RATIONALE:` comment block, explain why "free everything at once" is correct for a ZK prover (short-lived witness generation, no long-lived objects).

**What to understand**: A bump allocator is the fastest possible allocator (O(1) alloc, O(1) bulk free) but cannot free individual items. ZK provers allocate for one proof then free everything — perfect fit.

---

### Task 6 — `arkworks_annotation.rs`
**Read and annotate 20 lines of arkworks unsafe code — write a blog-style explanation**

- Find 20 consecutive lines of unsafe code in the arkworks-rs repo (suggested: `ark-ff/src/fields/models/`).
- Copy the lines into this file verbatim as a comment block.
- Below each line, write a `// ANNOTATION:` comment explaining what it does and why it is safe.
- At the bottom of the file, write a blog-style paragraph (150–200 words) explaining the block to a reader who knows Rust but has not read arkworks.

**What to understand**: Reading unsafe code written by experts teaches you the patterns — how safety comments are structured, what invariants are relied on, what makes a given unsafe block necessary vs. avoidable.

---

## 🎙️ Interview Drill — `INTERVIEW.md`

**Question**: *"You have unsafe code in a hot path. A colleague says to remove it. How do you justify keeping it?"*

**Have a principled answer ready**: benchmark evidence, documented safety invariants, fuzzing, Miri testing.

---

## 📚 Reference Reading

- [Rustonomicon — Unsafe Rust](https://doc.rust-lang.org/nomicon/)
- [Rustonomicon — PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [std::ptr module docs](https://doc.rust-lang.org/std/ptr/)
- [GlobalAlloc trait](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)
