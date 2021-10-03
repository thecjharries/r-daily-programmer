# [2016-09-05] Challenge #282 [Easy] Unusual Bases

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/5196fi/20160905_challenge_282_easy_unusual_bases/) by [u/SovietKetchup](https://old.reddit.com/u/SovietKetchup)

## Prompt

[Easy/Intermediate] Unusual Bases

# Description

Binary numbers (base 2) are written using `1`s and `0`s to represent which powers of 2 sum together to create the decimal number.

16 | 8 | 4 | 2 | 1
---|---|----|----|----
1 | 0 | 0 | 1 | 1

A `1` represents using that power of 2 and a `0` means not using it. In the above example there is a one in the `16`s, `2`s and the `1`s so we do:

    10011 = 16 + 2 + 1 = 19

meaning that `10011` is binary for `19`

The Fibonacci Sequence has a similar property that any positive integer can be written in the form of Fibonacci numbers (with no repeats). For example:

    25 = 21 + 3 + 1

If we use the same form as for writing binary, with the Fibonacci sequence instead of powers of 2, we can represent which Fibonacci numbers we use with a 1, and the ones we don't with a 0.

13 | 8 | 5 | 3 | 2 | 1 | 1
|--- |---|---|----|----|----|----
1 | 0 | 1 | 0 | 0 | 1 | 0

    1010010 = 13 + 5 + 1 = 19

meaning that `101001` is 'Base Fib' for `19`

The task is to create a converter to convert to and from decimal to 'Base Fib'
Due to the nature of the Fibonacci Sequence, many numbers have multiple representations in 'Base Fib', for the moment these are to be ignored - any form is acceptable.

## Input description

You will be given a line of input for each conversion, stating the base it is currently in, and the number to convert seperated by space

    10 16
    10 32
    10 9024720
    F 10
    F 1
    F 111111
    F 100000
    F 10110110100111001

## Output description

The program should output the converted number, in it's expected base, e.g.

    1001000
    10101000
    1010100101010100000010001000010010
    1
    1
    20
    8
    2868

# Notes/Hints

* [List of Fibonacci Numbers](http://planetmath.org/listoffibonaccinumbers), though you can generate these yourself quite easily.

Your language probably already has a list of primes, although for the bonus you may need to create you own list of Fibonacci Numbers

# Bonus

Now, a specific form is required for the 'Base Fib' answers.

Because each term of the sequence is the sum of the previous two, the 'Base Fib' form of a decimal number in the Fibonacci sequence can either be the term itself, or the previous two, e.g.

    8             = 100000
    8 = 5 + 3     = 11000
    8 = 5 + 2 + 1 = 10101

For the bonus challenge, give the output with the least `1`'s.


##Bonus input

    10 8
    10 16
    10 32
    10 9024720

#Bonus 2

As /u/thorwing suggested, it would be a greater challenge to write the base fib with the most `1`'s instead of the least

# Finally

Have a good challenge idea like /u/SovietKetchup?

Consider submitting it to /r/dailyprogrammer_ideas


# Edit
As some of you have pointed out, my solution had a small bug in it.

    9024720 -> 1010100101010100000010001000010010
