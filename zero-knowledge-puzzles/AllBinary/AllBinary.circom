template AllBinary(n) {
    signal input in[n];

    for (var i = 0; i < n; i++) {
        in[i] * (in[i] - 1) === 0;
    }
}

component main = AllBinary(4);