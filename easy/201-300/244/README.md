# [2015-12-09] Challenge #244 [Easy]er - Array language (part 3) - J Forks

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/3wdm0w/20151209_challenge_244_easyer_array_language_part/) by [u/Godspiral](https://old.reddit.com/user/Godspiral)

## Prompt

This challenge does not require doing the previous 2 parts.  If you want something harder, the rank conjunction from Wednesday's challenge requires concentration.

# Forks

A fork is a function that takes 3 functions that are all "duck defined" to take 2 parameters with 2nd optional or ignorable.

for 3 functions,  `f(y,x= default):` , `g(y,x= default):` , `h(y,x= default):` , where the function g is a "genuine" 2 parameter function,

the call `Fork(f,g,h)` executes the function composition:

     g(f(y,x),h(y,x))  (data1,data2)

**1.  Produce the string that makes the function call from string input:**

      sum divide count

(above input are 3 function names to Fork)

**2. Native to your favorite language, create an executable function from above string input**

or 3. create a function that takes 3 functions as input, and returns a function.

      Fork(sum, divide ,count)  (array data)

should return the mean of that array.  Where divide works similarly to add from Monday's challenge.

**4. Extend above functions to work for any odd number of function parameters**

for 5 parameters,  Fork(a, b, c, d, e) is:

       b(a, Fork(c,d,e))      NB. should expand this if producing strings.

# challenge input
(25 functions)

     a b c d e f g h i j k l m n o p q r s t u v w x y
