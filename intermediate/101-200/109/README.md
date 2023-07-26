# [10/30/2012] Challenge #109 [Intermediate]

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/12csm4/10302012_challenge_109_intermediate/)

## Prompt

**Description:**

A palindrome is a string of characters that are read the same way both ways (forward and backwards). Given two range of integers (a_start, a_end and b_start, b_end), if at least one of the products between the two ranges is a palindrome, print the integer-pair.

For example, if the first range of integers is [90,99] and the second is [90,99], there is at least one palindrome because 91 x 99 = 9009, which is read the same forward and backward. Thus, "91, 99" should br printed.

**Formal Inputs & Outputs:**

*Input Description:*

Integer a_start - The starting range of the integer a

Integer a_end - The ending range of the integer a

Integer b_start - The starting range of the integer b

Integer b_end - The ending range of the integer b

*Output Description:*

Print an integer pair if their product is a palindrome.

**Sample Inputs & Outputs:**

Let a_start and a_end be 10, 11, and let b_start and b_end be 10, 11. Your code, given these arguments, should print "11, 11", since 11 * 11 is 121, and is a palindrome.

**Notes:**

This problem is of an easy-intermediate difficulty level; a brute-force solution works well enough, but think of what happens when given a large range of numbers. What is the computational complexity? What can you do to optimize palindrome verification?
