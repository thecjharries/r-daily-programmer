# [2018-09-04] Challenge #367 [Easy] Subfactorials - Another Twist on Factorials

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/9cvo0f/20180904_challenge_367_easy_subfactorials_another/) by [u/jnazario](https://old.reddit.com/user/jnazario)

## Prompt

# Description

Most everyone who programs is familiar with the factorial - _n!_ - of a number, the product of the series from _n_ to _1_. One interesting aspect of the factorial operation is that it's also the number of permutations of a set of _n_ objects.

Today we'll look at the _subfactorial_, defined as the [*derangement*](https://en.wikipedia.org/wiki/Derangement) of a set of _n_ objects, or a permutation of the elements of a set, such that no element appears in its original position. We denote it as _!n_.

Some basic definitions:

- !1 -> 0 because you always have {1}, meaning 1 is always in it's position.
- !2 -> 1 because you have {2,1}.
- !3 -> 2 because you have {2,3,1} and {3,1,2}.

And so forth.

Today's challenge is to write a subfactorial program. Given an input _n_, can your program calculate the correct value for _n_?

# Input Description

You'll be given inputs as one integer per line. Example:

    5

# Output Description

Your program should yield the subfactorial result. From our example:

    44

(EDIT earlier I had 9 in there, but that's incorrect, that's for an input of 4.)

# Challenge Input

    6
    9
    14

# Challenge Output

    !6 -> 265
    !9 -> 133496
    !14 -> 32071101049

# Bonus

Try and do this as code golf - the shortest code you can come up with.

# Double Bonus

Enterprise edition - the most heavy, format, ceremonial code you can come up with in the enterprise style.

# Notes

This was inspired after watching the Mind Your Decisions video about the "3 3 3 10" puzzle, where a subfactorial was used in one of the solutions.
