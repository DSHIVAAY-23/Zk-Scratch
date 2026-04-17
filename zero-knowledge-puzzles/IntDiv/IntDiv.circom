template Division() {
    signal input numerator;
    signal input denominator;
    signal input quotient;
    signal input remainder;

    numerator === denominator * quotient + remainder;

    component lt = LessThan(252);
    lt.in[0] <== remainder;
    lt.in[1] <== denominator;

    lt.out === 1;
}