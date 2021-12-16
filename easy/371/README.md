# [2018-12-31] Challenge #371 [Easy] N queens validator

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/ab9mn7/20181231_challenge_371_easy_n_queens_validator/) by [u/Cosmologicon](https://old.reddit.com/user/Cosmologicon)

## Prompt

For the purpose of this challenge, the N queens problem consists of putting one queen on every column (labeled a, b, c, ...) of an NxN chessboard, such that no two queens are in the same row or diagonal. An example valid solution for N = 6 is:

    6  . . Q . . .
    5  . . . . . Q
    4  . Q . . . .
    3  . . . . Q .
    2  Q . . . . .
    1  . . . Q . .
       a b c d e f

In chess notation, the squares with queens in this solution are called `a2`, `b4`, `c6`, `d1`, `e3`, and `f5`. We'll represent solutions by listing the rows that each column's queen appears in from left to right, so this solution is represented as the array `{2, 4, 6, 1, 3, 5}`.

Solving the N queens problem was [#25 (difficult)](https://www.reddit.com/r/dailyprogrammer/comments/qxv8h/3152012_challenge_25_difficult/) on r/dailyprogrammer, but you don't need to actually solve it for today's challenge.

# Challenge

Given an array of 8 integers between 1 and 8, determine whether it represents a valid 8 queens solution.

    qcheck({4, 2, 7, 3, 6, 8, 5, 1}) => true
    qcheck({2, 5, 7, 4, 1, 8, 6, 3}) => true
    qcheck({5, 3, 1, 4, 2, 8, 6, 3}) => false   (b3 and h3 are on the same row)
    qcheck({5, 8, 2, 4, 7, 1, 3, 6}) => false   (b8 and g3 are on the same diagonal)
    qcheck({4, 3, 1, 8, 1, 3, 5, 2}) => false   (multiple problems)

You may optionally handle solutions for any N, not just N = 8.

# Optional bonus

In this bonus, you are given an invalid solution where it's possible to swap two numbers and produce a valid solution, which you must find. (Be aware that most invalid solutions will not have this property.)

For example, `{8, 6, 4, 2, 7, 1, 3, 5}` is invalid because `c4` and `f1` are on the same diagonal. But if you swap the 8 and the 4 (i.e. replace `a8` and `c4` with `a4` and `c8`), you get the valid solution `{4, 6, 8, 2, 7, 1, 3, 5}`.

    qfix({8, 6, 4, 2, 7, 1, 3, 5}) => {4, 6, 8, 2, 7, 1, 3, 5}
    qfix({8, 5, 1, 3, 6, 2, 7, 4}) => {8, 4, 1, 3, 6, 2, 7, 5}
    qfix({4, 6, 8, 3, 1, 2, 5, 7}) => {4, 6, 8, 3, 1, 7, 5, 2}
    qfix({7, 1, 3, 6, 8, 5, 2, 4}) => {7, 3, 1, 6, 8, 5, 2, 4}
