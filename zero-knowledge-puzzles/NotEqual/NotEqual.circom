pragma circom 2.1.4;


// Input : a , length of 2 .
// Output : c .
// In this exercise , you have to check that a[0] is NOT equal to a[1], if not equal, output 1, else output 0.
// You are free to use any operator you may like . 

// HINT:NEGATION

template NotEqual() {
    signal input in[2];
    signal output c;
    
    component eq = IsEqual();
    eq.in[0] <== in[0];
    eq.in[1] <== in[1];
    
    c <== eq.out;      // 1 if equal, 0 if not
    c === 0;           // force: must NOT be equal
}

component main = NotEqual();