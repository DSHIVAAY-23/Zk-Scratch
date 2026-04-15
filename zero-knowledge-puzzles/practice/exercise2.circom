pragma circom 2.1.6;

include "circomlib/comparators.circom";

template AtLeastTwo() {
    signal input in[4];
    signal output out;

    // Step 1: Binary constraint
    for (var i = 0; i < 4; i++) {
        in[i] * (in[i] - 1) === 0;
    }

    // Step 2: Sum of inputs
    signal sum;
    sum <== in[0] + in[1] + in[2] + in[3];

    // Step 3: sum >= 2
    component ge = GreaterEqThan(3);
    ge.in[0] <== sum;
    ge.in[1] <== 2;

    out <== ge.out;
}

component main = AtLeastTwo();