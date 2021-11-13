# [2017-08-7] Challenge #326 [Easy] Nearest Prime Numbers

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/6s70oh/2017087_challenge_326_easy_nearest_prime_numbers/) by [u/tulanir](https://old.reddit.com/u/tulanir)

## Prompt


# Description

A prime number is any integer greater than 1 which can only be evenly divided by 1 or itself. For this challenge, you will output two numbers: the nearest prime below the input, and the nearest prime above it.

# Input Description

The input will be a number on each line, called *n*.

# Output Description

The format of the output will be:
> *p1 < n < p2*

where *p1* is the smaller prime, *p2* is the larger prime and *n* is the input.

If *n* already is a prime, the output will be:
>*n* is prime.

# Challenge Input

	270
	541
	993
	649

# Challenge Output

	269 < 270 < 271
	541 is prime.
	991 < 993 < 997
	647 < 649 < 653

# Bonus

Write the program to work for numbers with big gaps to the nearest primes. This requires a clever solution and cannot be efficiently bruteforced.

    2010741
    1425172824437700148

# Credit

This challenge was suggested by user /u/tulanir, many thanks! If you have an idea for a challenge please share it on /r/dailyprogrammer_ideas and there's a good chance we'll use it.
