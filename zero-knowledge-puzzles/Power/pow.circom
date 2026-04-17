template Power(n) {
    signal input a;
    signal output out;

    signal temp[n];

    temp[0] <== a;

    for (var i = 1; i < n; i++) {
        temp[i] <== temp[i-1] * a;
    }

    out <== temp[n-1];
}