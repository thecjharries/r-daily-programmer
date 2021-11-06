# [2017-05-22] Challenge #316 [Easy] Knight's Metric

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/6coqwk/20170522_challenge_316_easy_knights_metric/) by [u/Cosmologicon](https://old.reddit.com/u/Cosmologicon)

## Prompt

# Description

A knight piece in chess can only make L-shaped moves. Specifically, it can only move x steps to the right and y steps up if (x,y) is one of:

    (-1,-2) ( 1,-2) (-1, 2) ( 1, 2)
    (-2,-1) ( 2,-1) (-2, 1) ( 2, 1)

(For this problem, assume the board extends infinitely in all directions, so a knight may always make eight moves from any starting point.) A knight starting from (0,0) can reach any square, but it may have to take a roundabout route. For instance, to reach the square (0,1) requires three steps:

     2,  1
    -1,  2
    -1, -2

(Notice that the x's add up to 0, and the y's add up to 1.) Write a program, that, given a square (x,y), returns how many moves it takes a knight to reach that square starting from (0,0).

# Example Input

    3 7

# Example Output

    4

Optional: also output one route the knight could take to reach this square.

# Credit

This challenge was suggested by /u/Cosmologicon, a well-known moderator of this sub. Many thanks! This one was hiding in the archives ...
