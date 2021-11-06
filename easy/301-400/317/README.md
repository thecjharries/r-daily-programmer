# [2017-05-29] Challenge #317 [Easy] Collatz Tag System

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/6e08v6/20170529_challenge_317_easy_collatz_tag_system/) by [u/thebutterflydefect](https://old.reddit.com/u/thebutterflydefect)

## Prompt


# Description

Implement the [Collatz Conjecture tag system described here](https://en.wikipedia.org/wiki/Tag_system#Example:_Computation_of_Collatz_sequences)

# Input Description

A string of n *a*'s

# Output Description

Print the string at each step. The last line should be "*a*" (assuming the Collatz conjecture)

# Challenge Input


	aaa
	aaaaa

# Challenge Output

aaa

    abc
    cbc
    caaa
    aaaaa
    aaabc
    abcbc
    cbcbc
    cbcaaa
    caaaaaa
    aaaaaaaa
    aaaaaabc
    aaaabcbc
    aabcbcbc
    bcbcbcbc
    bcbcbca
    bcbcaa
    bcaaa
    aaaa
    aabc
    bcbc
    bca
    aa
    bc
    a

aaaaaaa

    aaaaabc
    aaabcbc
    abcbcbc
    cbcbcbc
    cbcbcaaa
    cbcaaaaaa
    caaaaaaaaa
    aaaaaaaaaaa
    aaaaaaaaabc
    aaaaaaabcbc
    aaaaabcbcbc
    aaabcbcbcbc
    abcbcbcbcbc
    cbcbcbcbcbc
    cbcbcbcbcaaa
    cbcbcbcaaaaaa
    cbcbcaaaaaaaaa
    cbcaaaaaaaaaaaa
    caaaaaaaaaaaaaaa
    aaaaaaaaaaaaaaaaa
    aaaaaaaaaaaaaaabc
    aaaaaaaaaaaaabcbc
    aaaaaaaaaaabcbcbc
    aaaaaaaaabcbcbcbc
    aaaaaaabcbcbcbcbc
    aaaaabcbcbcbcbcbc
    aaabcbcbcbcbcbcbc
    abcbcbcbcbcbcbcbc
    cbcbcbcbcbcbcbcbc
    cbcbcbcbcbcbcbcaaa
    cbcbcbcbcbcbcaaaaaa
    cbcbcbcbcbcaaaaaaaaa
    cbcbcbcbcaaaaaaaaaaaa
    cbcbcbcaaaaaaaaaaaaaaa
    cbcbcaaaaaaaaaaaaaaaaaa
    cbcaaaaaaaaaaaaaaaaaaaaa
    caaaaaaaaaaaaaaaaaaaaaaaa
    aaaaaaaaaaaaaaaaaaaaaaaaaa
    aaaaaaaaaaaaaaaaaaaaaaaabc
    aaaaaaaaaaaaaaaaaaaaaabcbc
    aaaaaaaaaaaaaaaaaaaabcbcbc
    aaaaaaaaaaaaaaaaaabcbcbcbc
    aaaaaaaaaaaaaaaabcbcbcbcbc
    aaaaaaaaaaaaaabcbcbcbcbcbc
    aaaaaaaaaaaabcbcbcbcbcbcbc
    aaaaaaaaaabcbcbcbcbcbcbcbc
    aaaaaaaabcbcbcbcbcbcbcbcbc
    aaaaaabcbcbcbcbcbcbcbcbcbc
    aaaabcbcbcbcbcbcbcbcbcbcbc
    aabcbcbcbcbcbcbcbcbcbcbcbc
    bcbcbcbcbcbcbcbcbcbcbcbcbc
    bcbcbcbcbcbcbcbcbcbcbcbca
    bcbcbcbcbcbcbcbcbcbcbcaa
    bcbcbcbcbcbcbcbcbcbcaaa
    bcbcbcbcbcbcbcbcbcaaaa
    bcbcbcbcbcbcbcbcaaaaa
    bcbcbcbcbcbcbcaaaaaa
    bcbcbcbcbcbcaaaaaaa
    bcbcbcbcbcaaaaaaaa
    bcbcbcbcaaaaaaaaa
    bcbcbcaaaaaaaaaa
    bcbcaaaaaaaaaaa
    bcaaaaaaaaaaaa
    aaaaaaaaaaaaa
    aaaaaaaaaaabc
    aaaaaaaaabcbc
    aaaaaaabcbcbc
    aaaaabcbcbcbc
    aaabcbcbcbcbc
    abcbcbcbcbcbc
    cbcbcbcbcbcbc
    cbcbcbcbcbcaaa
    cbcbcbcbcaaaaaa
    cbcbcbcaaaaaaaaa
    cbcbcaaaaaaaaaaaa
    cbcaaaaaaaaaaaaaaa
    caaaaaaaaaaaaaaaaaa
    aaaaaaaaaaaaaaaaaaaa
    aaaaaaaaaaaaaaaaaabc
    aaaaaaaaaaaaaaaabcbc
    aaaaaaaaaaaaaabcbcbc
    aaaaaaaaaaaabcbcbcbc
    aaaaaaaaaabcbcbcbcbc
    aaaaaaaabcbcbcbcbcbc
    aaaaaabcbcbcbcbcbcbc
    aaaabcbcbcbcbcbcbcbc
    aabcbcbcbcbcbcbcbcbc
    bcbcbcbcbcbcbcbcbcbc
    bcbcbcbcbcbcbcbcbca
    bcbcbcbcbcbcbcbcaa
    bcbcbcbcbcbcbcaaa
    bcbcbcbcbcbcaaaa
    bcbcbcbcbcaaaaa
    bcbcbcbcaaaaaa
    bcbcbcaaaaaaa
    bcbcaaaaaaaa
    bcaaaaaaaaa
    aaaaaaaaaa
    aaaaaaaabc
    aaaaaabcbc
    aaaabcbcbc
    aabcbcbcbc
    bcbcbcbcbc
    bcbcbcbca
    bcbcbcaa
    bcbcaaa
    bcaaaa
    aaaaa
    aaabc
    abcbc
    cbcbc
    cbcaaa
    caaaaaa
    aaaaaaaa
    aaaaaabc
    aaaabcbc
    aabcbcbc
    bcbcbcbc
    bcbcbca
    bcbcaa
    bcaaa
    aaaa
    aabc
    bcbc
    bca
    aa
    bc
    a

# Notes/Hints

The [Collatz Conjecture](https://en.wikipedia.org/wiki/3x_%2B_1_problem)

If you're not familiar with tag systems, you can read the [Wikipedia article on them here](https://en.wikipedia.org/wiki/Tag_system)



# Bonus

Implement the same tag system as a cyclic tag system using the [schema described here](https://en.wikipedia.org/wiki/Tag_system#Emulation_of_tag_systems_by_cyclic_tag_systems)

# Bonus Input

	100100100

# Bonus Output

    00100100010001
    0100100010001
    100100010001
    00100010001
    0100010001
    100010001
    00010001010001
    0010001010001
    010001010001
    10001010001
    0001010001
    001010001
    01010001
    1010001
    010001100100100
    10001100100100
    0001100100100
    001100100100
    01100100100
    1100100100
    100100100100100100
    00100100100100100
    0100100100100100
    100100100100100
    00100100100100010001
    0100100100100010001
    100100100100010001
    00100100100010001
    0100100100010001
    100100100010001
    00100100010001010001
    0100100010001010001
    100100010001010001
    00100010001010001
    0100010001010001
    100010001010001
    00010001010001010001
    0010001010001010001
    010001010001010001
    10001010001010001
    0001010001010001
    001010001010001
    01010001010001
    1010001010001
    010001010001100100100
    10001010001100100100
    0001010001100100100
    001010001100100100
    01010001100100100
    1010001100100100
    010001100100100100100100
    10001100100100100100100
    0001100100100100100100
    001100100100100100100
    01100100100100100100
    1100100100100100100
    100100100100100100100100100
    00100100100100100100100100
    0100100100100100100100100
    100100100100100100100100
    00100100100100100100100010001
    0100100100100100100100010001
    100100100100100100100010001
    00100100100100100100010001
    0100100100100100100010001
    100100100100100100010001
    00100100100100100010001010001
    0100100100100100010001010001
    100100100100100010001010001
    00100100100100010001010001
    0100100100100010001010001
    100100100100010001010001
    00100100100010001010001010001
    0100100100010001010001010001
    100100100010001010001010001
    00100100010001010001010001
    0100100010001010001010001
    100100010001010001010001
    00100010001010001010001010001
    0100010001010001010001010001
    100010001010001010001010001
    00010001010001010001010001
    0010001010001010001010001
    010001010001010001010001
    10001010001010001010001
    0001010001010001010001100
    001010001010001010001100
    01010001010001010001100
    1010001010001010001100
    010001010001010001100
    10001010001010001100
    0001010001010001100100
    001010001010001100100
    01010001010001100100
    1010001010001100100
    010001010001100100
    10001010001100100
    0001010001100100100
    001010001100100100
    01010001100100100
    1010001100100100
    010001100100100
    10001100100100
    0001100100100100
    001100100100100
    01100100100100
    1100100100100
    100100100100
    00100100100010001
    0100100100010001
    100100100010001
    00100100010001
    0100100010001
    100100010001
    00100010001010001
    0100010001010001
    100010001010001
    00010001010001
    0010001010001
    010001010001
    10001010001
    0001010001100
    001010001100
    01010001100
    1010001100
    010001100
    10001100
    0001100100
    001100100
    01100100
    1100100
    100100
    00100010001
    0100010001
    100010001
    00010001
    0010001
    010001
    10001
    0001100
    001100
    01100
    1100
    100

# Credit

This challenge was proposed by /u/thebutterflydefect, many thanks. If you have a challenge idea, please share it in /r/dailyprogrammer_ideas  and there's a good chance we'll use it.
