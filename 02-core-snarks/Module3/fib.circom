pragma circom 2.1.6;


template fib(n){
    signal fib1[n+1];
    signal output out;
     
    fib1[0] <== 0;
    fib1[1] <== 1;

    for (var i = 2; i <= n; i++) {
        fib1[i] <== fib1[i-1] + fib1[i-2];
    }

    out <== fib1[n];

}

component main = fib(5);