
pragma circom 2.1.6;
include "circomlib/comparators.circom";
include "circomlib/gates.circom";
//circuit 1
template inbetween(){
    signal input x;

    signal input ind1 == GreaterThan(252)([x, 10]);      // 1 if x<5
    signal input ind2 == LessThan(252)([x, 30]);  //

    component or = AND();
    or.a <== ind1;
    or.b <== ind2;
    or.out === 0;   // ← CRITICAL — yeh bhoolna =

}

//circuit 2 is even 
 template isEven(n){
    signal input x;
    signal output out;

    out <-- x % 2; // compute (no constraint)
    out === 0;      // constrain (verify correctness)




 }

  


//circuit 3 ismin


template isMin(n){
    signal input in[n]  ;
    signal output out;


     var min = in[0];
    for (var i = 1; i < n; i++) {
        min = in[i] < min ? in[i] : min;
    }
    signal minSig <-- min;

    component EQ[n];
    var acc = 0;    
    for (var i =0;i<n;i++){
        EQ[i] = IsEqual();
        EQ[i].in[0] <== minSig;
        EQ[i].in[1] <== in[i];
        acc += EQ[i].out;

    }  

    signal allZero <== IsEqual()([0, acc]);
    allZero === 0;
    out <== minSig;
}

