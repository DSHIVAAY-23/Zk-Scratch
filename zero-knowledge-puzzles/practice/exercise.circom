pragma circom 2.1.6;
include "../node_modules/circomlib/circuits/gates.circom";


template practice1() {
   signal input in[4];
   signal output out;


   component or = OR();
   component or1 = OR();
   component or2 = OR();


   or.a <== in[0];
   or.b <== in[1];


   or1.a <== or.out;
   or1.b <== in[2];


   or2.a <== or1.out;
   or2.b <== in[3];


   out <== or2.out;
}





