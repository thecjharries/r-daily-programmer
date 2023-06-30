# [9/08/2012] Challenge #97 [intermediate] (Sierpinski carpet)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/zker3/9082012_challenge_97_intermediate_sierpinski/)

## Prompt

Write a function that accepts an integer *n* and returns a (3^*n* × 3^n ) boolean matrix containing a *n*th-iteration [Sierpinski carpet](http://en.wikipedia.org/wiki/Sierpinski_carpet) fractal.

* How many 1 bits are there in `carpet(7)`?
* What is the largest value of `n` for which the matrix returned by  `carpet(n)` fits in a terabyte?

For bonus points, write a general function `center_iter(d, n)` that generates fractals like the Sierpinski carpet in *d* dimensions. (`center_iter(1, n)` is the [Cantor set](http://en.wikipedia.org/wiki/Cantor_set), center_iter(2, n) the Sierpinski carpet, `center_iter(3, 1)` a 3x3x3 cube with the center piece removed, etc.)
