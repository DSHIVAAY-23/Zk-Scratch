template MultiANDNoOut(n) {
    signal input in[n];

    for (var i = 0; i < n; i++) {
        in[i] === 1;
    }
}

component main = MultiANDNoOut(4);