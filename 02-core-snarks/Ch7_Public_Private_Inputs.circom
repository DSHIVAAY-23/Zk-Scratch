pragma circom 2.1.6;

// Example: "Maine do private numbers multiply kiye aur result public hai"
template Main() {
    signal input a;          // private
    signal input b;          // private  
    signal output c;         // public (output = always public)
    
    c <== a * b;
}
component main = Main();

// vs agar c public input ho:
template Main() {
    signal input a;          // private
    signal input b;          // private
    signal input c;          // private by default
    a * b === c;
}
component main {public [c]} = Main();  // c explicitly public