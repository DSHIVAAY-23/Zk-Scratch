pragma circom 2.1.6;

// IsSorted
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

// Max of array
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

// AllUnique — Nested loop pattern
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