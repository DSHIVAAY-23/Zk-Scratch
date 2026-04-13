pragma circom 2.1.6;

template ValidSqrt() {
    signal input in;
    signal output out;
    
    out <-- sqrt(in);    // compute (no constraint)
    out * out === in;    // constrain (verify correctness)
}

template MulInv() {
    signal input in;
    signal output out;
    
    out <-- 1 / in;      // compute inverse out-of-circuit
    out * in === 1;      // constrain: verify it's correct inverse
}

template IsZero() {
    signal input in;
    signal output out;
    signal inv;
    
    // Compute
    inv <-- in != 0 ? 1/in : 0;
    
    // Constrain
    out <== -in * inv + 1;
    in * out === 0;
    
    // Logic:
    // in=0  → inv=0, out=1, 0*1=0 ✅
    // in≠0  → inv=1/in, out=0, in*0=0 ✅
}