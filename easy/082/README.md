# [7/27/2012] Challenge #82 [easy] (Substring list)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/x8rl8/7272012_challenge_82_easy_substring_list/) by ???

## Prompt

Write a function that takes a number *n* as an argument and returns (or outputs) every possible unique substrings (not counting "") of the first *n* letters of the alphabet (in any order you like). For example, `substrings(5)` could be:

    a
    ab
    abc
    abcd
    abcde
    b
    bc
    bcd
    bcde
    c
    cd
    cde
    d
    de
    e

**BONUS 1:** Find a way to quickly determine how many strings this function returns for a given input. If the alphabet were 500 letters long, how much possible substrings would it have?

**BONUS 2:** Modify your function to take a string as an argument. Make sure all substrings in your output are still unique (i.e., there are two "l" substrings in "hello", but you should output only one).
