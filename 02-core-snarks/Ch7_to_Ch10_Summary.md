# Summary of Chapters 7 to 10

## Chapter 7: Public & Private Inputs

### Key Concepts
- **Private Inputs**: By default, all input signals are private and only known to the prover.
- **Public Inputs**: Explicitly declared using `component main {public [signal_name]} = Main();`.
- **Output Signals**: Always public by default.

### Witness Vector Layout
The witness vector is arranged in the following order:
1. Constant (always 1)
2. Public signals
3. Private signals
4. Intermediate signals

### Example
```circom
// Example: Multiply two private numbers and make the result public
template Main() {
    signal input a;          // private
    signal input b;          // private
    signal output c;         // public (output = always public)
    
    c <== a * b;
}
component main = Main();

// Example: Make `c` explicitly public
template Main() {
    signal input a;          // private
    signal input b;          // private
    signal input c;          // private by default
    a * b === c;
}
component main {public [c]} = Main();  // c explicitly public
```

### Common Vulnerability
Declaring an output signal without assigning a value creates a vulnerability, as the prover can assign any value to it.

```circom
// DANGEROUS — output declared but not assigned
template Bad() {
    signal input a;
    signal output out;  // not assigned — prover controls this!
}
```

---

## Chapter 8: Indicate Then Constrain

### Pattern
1. Create indicator signals (0/1) for conditions.
2. Apply constraints to the indicators using logical gates (e.g., OR, AND).

### Example
```circom
pragma circom 2.1.6;
include "circomlib/comparators.circom";
include "circomlib/gates.circom";

template XLessThan5OrGreaterThan17() {
    signal input x;
    
    // Step 1: INDICATE
    signal ind1 <== LessThan(252)([x, 5]);      // 1 if x<5
    signal ind2 <== GreaterThan(252)([x, 17]);  // 1 if x>17
    
    // Step 2: CONSTRAIN — at least one must be true
    component or = OR();
    or.a <== ind1;
    or.b <== ind2;
    or.out === 1;   // ← CRITICAL — yeh bhoolna = vulnerability!
}
```

### Common Bug
Forgetting to constrain the output of a component:
```circom
// VULNERABLE — output constrained nahi hai
template Bad() {
    signal input x;
    signal input y;
    
    component and = AND();
    and.a <== x;
    and.b <== y;
    // and.out === 1; ← YEH BHOOL GAYE!
}

// CORRECT
template Good() {
    signal input x;
    signal input y;
    
    component and = AND();
    and.a <== x;
    and.b <== y;
    and.out === 1;  // ✅ constrain karo!
}
```

---

## Chapter 9: Compute Then Constrain

### Pattern
1. Use `<--` to compute a value (no constraint).
2. Use `===` to constrain and verify the computation.

### Examples
1. **Modular Square Root**
```circom
template ValidSqrt() {
    signal input in;
    signal output out;
    
    out <-- sqrt(in);    // compute (no constraint)
    out * out === in;    // constrain (verify correctness)
}
```

2. **Modular Inverse**
```circom
template MulInv() {
    signal input in;
    signal output out;
    
    out <-- 1 / in;      // compute inverse out-of-circuit
    out * in === 1;      // constrain: verify it's correct inverse
}
```

3. **IsZero**
```circom
template IsZero() {
    signal input in;
    signal output out;
    signal inv;
    
    // Compute
    inv <-- in != 0 ? 1/in : 0;
    
    // Constrain
    out <== -in * inv + 1;
    in * out === 0;
}
```

---

## Chapter 10: Components in a Loop

### Problem
Circom does not allow direct instantiation of components inside a loop.

### Solution
Declare an array of components outside the loop and assign types inside the loop.

### Example
1. **IsSorted**
```circom
template IsSorted(n) {
    signal input in[n];
    component lt[n-1];  // n-1 comparisons for n elements
    
    for (var i = 0; i < n-1; i++) {
        lt[i] = LessThan(252);
        lt[i].in[0] <== in[i];
        lt[i].in[1] <== in[i+1];
        lt[i].out === 1;
    }
}
```

2. **Max of Array**
```circom
template Max(n) {
    signal input in[n];
    signal output out;
    
    // Compute (no constraints)
    var max = 0;
    for (var i = 0; i < n; i++) {
        max = in[i] > max ? in[i] : max;
    }
    signal maxSig <-- max;
    
    // Constrain: max >= every element
    component GTE[n];
    component EQ[n];
    var acc = 0;
    
    for (var i = 0; i < n; i++) {
        GTE[i] = GreaterEqThan(252);
        GTE[i].in[0] <== maxSig;
        GTE[i].in[1] <== in[i];
        GTE[i].out === 1;
        
        EQ[i] = IsEqual();
        EQ[i].in[0] <== maxSig;
        EQ[i].in[1] <== in[i];
        acc += EQ[i].out;
    }
    
    // maxSig must equal at least one element
    signal allZero <== IsEqual()([0, acc]);
    allZero === 0;
    out <== maxSig;
}
```

3. **AllUnique**
```circom
template AllUnique(n) {
    signal input in[n];
    
    // n*(n-1)/2 comparisons needed
    component neq[n*(n-1)/2];
    
    var idx = 0;
    for (var i = 0; i < n-1; i++) {
        for (var j = i+1; j < n; j++) {
            neq[idx] = IsEqual();
            neq[idx].in[0] <== in[i];
            neq[idx].in[1] <== in[j];
            neq[idx].out === 0;  // must NOT be equal
            idx++;
        }
    }
}
```

---

## Summary
- **Public/Private Inputs**: Understand what is visible to the prover and verifier.
- **Indicate Then Constrain**: Use indicator signals for complex conditions and constrain them.
- **Compute Then Constrain**: Perform complex computations first, then verify with constraints.
- **Components in Loop**: Use arrays of components to handle loops.

These patterns and best practices ensure secure and efficient Circom circuits.