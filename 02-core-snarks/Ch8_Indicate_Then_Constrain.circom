pragma circom 2.1.6;
include "circomlib/comparators.circom";
include "circomlib/gates.circom";

template XLessThan5OrGreaterThan17() {
    signal input x;
    
    // Step 1: INDICATE
    signal ind1 <== LessThan(252)([x, 5]);      // 1 if x<5
    signal ind2 <== GreaterThan(252)([x, 17]);  // 1 if x>17
    
    // Step 2: CONSTRAIN — at least one must be true
    component or = OR();
    or.a <== ind1;
    or.b <== ind2;
    or.out === 1;   // ← CRITICAL — yeh bhoolna = vulnerability!
}

template AtLeast2GreaterThan() {
    signal input k;
    signal input x;
    signal input y;
    signal input z;

    signal gtX <== GreaterThan(252)([k, x]);  // indicator
    signal gtY <== GreaterThan(252)([k, y]);
    signal gtZ <== GreaterThan(252)([k, z]);

    // Sum of indicators
    var total = gtX + gtY + gtZ;

    // Constrain: total >= 2
    signal atLeast2 <== GreaterEqThan(252)([total, 2]);
    atLeast2 === 1;
}