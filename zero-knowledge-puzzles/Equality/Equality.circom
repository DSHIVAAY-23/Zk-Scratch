pragma circom 2.1.4;

// Input 3 values using 'a'(array of length 3) and check if they all are equal.
// Return using signal 'c'.

template Equality() {
    signal input in[3];
    signal output c;
    in[0]===in[1];
   in[1]==in[2];
    c<==in[1];



   
}

component main = Equality();