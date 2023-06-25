# [9/15/2012] Challenge #98 [intermediate] (Multiple cycling)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/zx98u/9152012_challenge_98_intermediate_multiple_cycling/)

## Prompt

Write a function that takes two arguments: a limit, *lim*, and a list of integers, *x*. The function counts up from 0 by cycling through *x* and skipping numbers until we find the next number that's a multiple of *x[i]*. For example, when x is the list [5, 7, 3], start counting from 0:

1. Next multiple of 5 is 5
* Next multiple of 7 is 7
* Next multiple of 3 is 9
* Next multiple of 5 is 10
* Next multiple of 7 is 14
* Next multiple of 3 is 15

When the count reaches *lim* or a number above it, return the number of steps it took to reach it. (`multiple_cycle(15, [5, 7, 3])` would return 6.)

What is the result of `multiple_count(1000000000, [5395, 7168, 2367, 9999, 3])`?
