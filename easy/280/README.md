# [2016-08-22] Challenge #280 [Easy] 0 to 100, Real Quick

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4z04vj/20160822_challenge_280_easy_0_to_100_real_quick/) by [u/abyssalheaven](https://old.reddit.com/u/abyssalheaven)

## Prompt

# Description

Oh, how cursed we are to have but 10 digits upon our fingers. Imagine the possibilities were we able to count to numbers beyond! But halt! With 10 digits upon our two appendages, 1024 unique combinations appear! But alas, counting in this manner is cumbersome, and counting to such a number beyond reason. Surely being able to count up to 100 would suffice!


You will be given inputs which correspond to the 10 digits of a pair of hands in the following format, where 1 means the finger is raised, and 0 means the finger is down.

Example:

    **LP** | **LR** | **LM** | **LI** | **LT** |  **RT** | **RI** | **RM** | **RR** | **RP**
    ---------|-----------|-----------|----------|-----------|-----------|----------|------------|-----------|----------
    0 | 1 | 1 | 1 | 0 | 1 | 1| 1| 0| 0

    L = Left, R = Right, P = Pinky, R = Ring Finger, M = Middle Finger, I = Index Finger, T = Thumb


Your challenge is to take these inputs, and:

1. Determine if it is valid based on [this counting scheme](http://www.wikihow.com/Count-to-99-on-Your-Fingers).

2. If it is, then decode the inputs into the number represented by the digits on the hand.

# Formal Inputs and Outputs


    0111011100 -> 37
    1010010000 -> Invalid
    0011101110 -> 73
    0000110000 -> 55
    1111110001 -> Invalid

# Credit

This challenge was submitted by /u/abyssalheaven. Thank you! If you have any challenge ideas, please share them in /r/dailyprogrammer_ideas and there's a good chance we'll use them.
