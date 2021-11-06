# [2017-05-08] Challenge #314 [Easy] Concatenated Integers

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/69y21t/20170508_challenge_314_easy_concatenated_integers/) by [u/jnazario](https://old.reddit.com/user/jnazario)

## Prompt

# Description

Given a list of integers separated by a single space on standard input, print out the largest and smallest values that can be obtained by concatenating the integers together on their own line. This is from [Five programming problems every Software Engineer should be able to solve in less than 1 hour](http://www.shiftedup.com/2015/05/07/five-programming-problems-every-software-engineer-should-be-able-to-solve-in-less-than-1-hour), problem 4. Leading 0s are not allowed (e.g. 01234 is not a valid entry).

This is an easier version of [#312I](https://www.reddit.com/r/dailyprogrammer/comments/67q3s6/20170426_challenge_312_intermediate_next_largest/?utm_content=title&utm_medium=hot&utm_source=reddit&utm_name=dailyprogrammer).

# Sample Input

You'll be given a handful of integers per line. Example:

	5 56 50

# Sample Output

You should emit the smallest and largest integer you can make, per line. Example:

	50556 56550

# Challenge Input

	79 82 34 83 69
	420 34 19 71 341
	17 32 91 7 46

# Challenge Output

	3469798283 8382796934
	193413442071 714203434119
	173246791 917463217

# Bonus

**EDIT** My solution uses permutations, which is inefficient. Try and come up with a more efficient approach.
