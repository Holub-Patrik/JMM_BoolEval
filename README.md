# JMM_BoolEval

A boolean equation evaluator. 

((a <=> b) ^ (c => (c | d))), [ a=1 ; b=0 ]

For this equation it should print out a table of all possibilities using a=1 and b=0 as fixed values and test all the other ones
It should also be able to print a simple table for n variables.
So given the number 6 it should print 
0 | 0 | 0 | 0 | 0 | 0
0 | 0 | 0 | 0 | 0 | 1
0 | 0 | 0 | 0 | 1 | 0
.
.
.

I want to try to implement this in rust, but rust forces me to use AST and interpret, or jit compilation. 
I could also try c++ and just manage my own pointers allowing for the entire eval to happen within string
If I want to be lazy I should be able to make this in python relatively fast, but python will be very slow.

