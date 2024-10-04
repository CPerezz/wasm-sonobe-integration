pragma circom 2.0.3;

template Example () {
    signal input ivc_input;
    signal output ivc_output;   
    signal temp;
    
    temp <== ivc_input * ivc_input;
    ivc_output <== temp * ivc_input + ivc_input + 5;
}

component main {public [ivc_input]} = Example();