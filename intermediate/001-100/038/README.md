# [4/10/2012] Challenge #38 [intermediate]

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/s2na8/4102012_challenge_38_intermediate/)

## Prompt

[Reverse Polish Notation](http://en.wikipedia.org/wiki/Reverse_Polish_notation)(RPN) is a mathematical notation where every operator follows all of its operands. For instance, to add three and four, one would write "3 4 +" rather than "3 + 4". If there are multiple operations, the operator is given immediately after its second operand; so the expression written "3 − 4 + 5" would be written "3 4 − 5 +" first subtract 4 from 3, then add 5 to that.

Transform the algebraic expression with brackets into RPN form.

You can assume that for the test cases below only single letters will be used, brackets [ ] will not be used and each expression has only one RPN form (no expressions like a*b*c)

Test Input:

(a+(b*c))

((a+b)*(z+x))

((a+t)*((b+(a+c)) ^ (c+d)))



Test Output:

abc*+

ab+zx+*

at+bac++cd+ ^ *

