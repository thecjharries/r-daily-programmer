# [8/24/2012] Challenge #91 [intermediate] (Cantor's fractions)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/yqxyy/8242012_challenge_91_intermediate_cantors/)

## Prompt

Famous mathematician Georg Cantor once thought of a cool way to enumerate strictly positive rational numbers (i.e. x > 0): in an infinite 2D matrix of all rationals where A[x,y] = (x / y), start counting from A[1,1] and then zig-zag your way out like this:

http://www.homeschoolmath.net/teaching/rationals-countable.gif

Using this enumeration, write a program that outputs the first 1000 **distinct** strictly positive fractions as decimal numbers. (The repeated ones are crossed out in the above image, e.g. 2/2 = 1/1, so skip 2/2.)
